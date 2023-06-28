#!/usr/bin/env python3
from __future__ import annotations

import glob
import json
import os
import re
import shutil
import subprocess
import sys
from typing import Callable
from collections import defaultdict
from itertools import chain
from pathlib import Path

script_dir = Path(os.path.dirname(os.path.realpath(__file__)))
codex_root = script_dir / ".." / "code-exploration-services"
examples_dir = script_dir / ".." / "examples"
build_dir = script_dir / "build"
out_dir = script_dir / "out"

struct = re.compile(r"struct (?P<name>[a-zA-Z0-9_]+).*?\{.*?}\n\n", re.MULTILINE | re.DOTALL)
enum = re.compile(r"enum (?P<name>[a-zA-Z0-9_]+).*?\{.*?}\n\n", re.MULTILINE | re.DOTALL)
typedef = re.compile(r"type (?P<name>[a-zA-Z0-9_]+) = .*?;\n\n", re.MULTILINE | re.DOTALL)


def files():
    yield from (i for i in glob.iglob(f"{codex_root}/**/*.rs", recursive=True) if "target" not in i)


def line_of(text: str, offset: int):
    line = 0
    for idx, char in enumerate(text):
        if idx == offset:
            return line
        if char == "\n":
            line += 1

    return -1


def slice_line(text: str, start: int, end: int) -> str:
    line = 0
    start_idx = 0
    end_idx = 0
    for idx, char in enumerate(text):
        if line == start - 1:
            start_idx = idx

        if line == end - 1:
            end_idx = idx - 1

        if char == "\n":
            line += 1

    return text[start_idx:end_idx]


def find_codex_items(names: {str}) -> {str: [str]}:
    res = defaultdict(list)

    for file_name in files():
        with open(file_name) as f:
            c = f.read()
            for i in chain(struct.finditer(c), enum.finditer(c), typedef.finditer(c)):
                for name_parts in names:
                    for name in name_parts.split("+"):
                        if i.group("name") == name.strip():
                            start_line = line_of(c, i.start())
                            end_line = line_of(c, i.end())
                            res["".join(name_parts.split("+"))].append(slice_line(c, start_line, end_line).strip())

    return {k: "\n\n".join(v) for k, v in res.items()}


def run_codex_to_metadata(file: Path, f: Path = None):
    name = file.name
    if f is None:
        out_file = (out_dir / name).with_suffix(".json")
    else:
        out_file = f

    cmd = [
        "cargo", "run",
        "--",
        "analyse",
        "-f", file,
        "-o", out_file,
    ]

    out = subprocess.run(cmd, cwd=codex_root)
    out.check_returncode()

    with open(out_file, "r") as f:
        contents = f.read()

    with open(out_file, "w") as f:
        f.write(contents.strip())


def run_codex_to_latex(file: Path, refs: bool = False, wrap=True, usage=True):
    name = file.name
    latex_out_dir = (out_dir / name).with_suffix("")
    latex_out_dir.mkdir(parents=True)
    cmd = [
        "cargo", "run",
        "--",
        "generate", "latex",
        "-f", file,
        "-o", latex_out_dir,
        "--theme", "GitHub Light",
    ]

    if not refs:
        cmd.append("--no-refs")
    if not wrap:
        cmd.append("--no-wrap")
    if not usage:
        cmd.append("--no-usage")

    out = subprocess.run(cmd, cwd=codex_root)

    out_name = (out_dir / name).with_suffix(".tex")
    out.check_returncode()
    (latex_out_dir / "output.tex").rename(out_name)
    (latex_out_dir / "codex.sty").rename(out_dir / "codex.sty")
    latex_out_dir.rmdir()

    return out_name


def run_codex_to_html(file: Path):
    name = file.name
    html_out = (out_dir / name).with_suffix(".html")
    cmd = [
        "cargo", "run",
        "--",
        "generate", "simple-html",
        "-f", file,
        "-o", html_out,
    ]

    out = subprocess.run(cmd, cwd=codex_root)

    out.check_returncode()


def run_codex_to_latex_on_rust_string(name: str, contents: str, *args, **kwargs) -> Path:
    name = name.lower().replace(" ", "_") + ".rs"
    file_path = build_dir / name
    with open(file_path, "w") as f:
        f.write(contents)

    run_codex_to_latex(file_path, *args, **kwargs)
    return file_path


def reset():
    shutil.rmtree(build_dir, ignore_errors=True)
    shutil.rmtree(out_dir, ignore_errors=True)
    build_dir.mkdir(parents=True)
    out_dir.mkdir(parents=True)


def codex_source_item_to_latex(name: str | {str}, *args, **kwargs) -> Path:
    name, contents = next(iter(find_codex_items({name} if isinstance(name, str) else name).items()))
    return run_codex_to_latex_on_rust_string(name, contents, *args, **kwargs)


def codex_metadata_to_latex(file: Path, filter_fn: Callable[[str], bool] = None, *args, **kwargs):
    if filter_fn is None:
        filter_fn = lambda _: True

    out_file = (out_dir / ("metadata_" + file.name)).with_suffix(".json")
    run_codex_to_metadata(file, f=out_file)

    with open(out_file, "r") as f:
        contents = f.readlines()

    def remove_msg(a):
        try:
            a[1]["Info"]["Diagnostics"]["message"] = "..."
        except KeyError:
            pass

        try:
            x = a[1]["Tag"]["Colour"]
            a[1]["Tag"]["Colour"] = x.split(" ")[0]
        except KeyError:
            pass

        return a

    def concat_lines(a):
        lines = []
        skip = False
        for l, r in zip(a.splitlines(), a.splitlines()[1:] + [""]):
            if skip:
                skip = False
                continue
            if len(l + r) > 30 or "}" in l or "}" in r:
                lines.append(l)
            else:
                lines.append(l.rstrip() + r.lstrip())
                skip = True

        if len(lines) < len(a.splitlines()):
            return concat_lines("\n".join(lines))
        else:
            return "\n".join(lines)

    contents = [
        concat_lines(json.dumps(remove_msg(json.loads(i)), indent=2)) + "\n"
        if i.startswith("[")
        else i
        for i in contents
        if filter_fn(i)
    ]

    with open(out_file, "w") as f:
        f.writelines(contents)

    run_codex_to_latex(out_file, *args, **kwargs)


def metadata_is_at_offset(line: str, offset: str) -> bool:
    return line.removeprefix("[\"").startswith(offset)


def only_first(f, default, wanted_value):
    cache = set()

    def res(**outer_kwargs):
        def inner(*args, **kwargs):
            key = tuple(sorted(outer_kwargs.items()))
            if key in cache:
                return default
            else:
                res = f(*args, **outer_kwargs, **kwargs)
                if res == wanted_value:
                    cache.add(key)
                return res
        return inner
    return res


def main(_argv):
    reset()

    datastructures = script_dir / "data_structures.rs"
    run_codex_to_latex(datastructures, refs=True, usage=False)

    # f = only_first(metadata_is_at_offset, False, True)
    # codex_metadata_to_latex(datastructures)
    run_codex_to_latex(script_dir / "metadata_data_structures.json")

    run_codex_to_latex(script_dir / "paper_example.elaine", refs=True, wrap=False)
    run_codex_to_latex(script_dir / "rust_example", refs=True, wrap=False, usage=False)
    run_codex_to_html(script_dir / "rust_example" / "src" / "number.rs")


if __name__ == '__main__':
    main(sys.argv[1:])
