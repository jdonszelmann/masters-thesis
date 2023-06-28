#!/usr/bin/env python3
from __future__ import annotations

import glob
import json
import os
import shutil
import subprocess
import sys
from pathlib import Path
from multiprocessing import Pool

import requests as requests

script_dir = Path(os.path.dirname(os.path.realpath(__file__)))
crates_dir = script_dir / "crates"

crates_dir.mkdir(parents=True, exist_ok=True)

crates_list = json.load(open(script_dir / "crate-information.json"))

# for i in crates_list:
#     name = i["name"]
#     path = crates_dir / name
#     response = requests.get(f"https://crates.io/api/v1/crates/{name}")
#     response_json = response.json()
#     url = response_json["crate"]["repository"]
#     if not os.path.exists(path):
#         try:
#             status = subprocess.run([
#                 "git",
#                 "clone",
#                 "--depth=1",
#                 url,
#                 path
#             ], stdout=sys.stdout)
#             status.check_returncode()
#         except subprocess.CalledProcessError as e:
#             print(e)

in_list = [
    "addr2line",
    "adler",
    "aho-corasick",
    "allocator-api2",
    "ansi_term",
    "anstream",
    "anyhow",
    "approx",
    "arc-swap",
    "arrayvec",
    "async-recursion",
    "async-trait",
    "atomic",
    "atty",
    "autocfg",
    "backtrace",
    "base64",
    "bit_field",
    "bit-set",
    "bit-vec",
    "bitflags",
    "block-buffer",
    "bytemuck",
    "byteorder",
    "bytes",
    "cc",
    "cfg-if",
    "chrono",
    "color_quant",
    "cookie",
    "cookie_store",
    "crc32fast",
    "crossbeam",
    "crypto-common",
    "csv",
    "data-encoding",
    "new_debug_unreachable",
    "derivative",
    "destructure_traitobject",
    "either",
    "env_logger",
    "equivalent",
    "error-chain",
    "fallible-streaming-iterator",
    "fastrand",
    "fdeflate",
    "filetime",
    "fixedbitset",
    "flate2",
    "flume",
    "fnv",
    "foreign-types",
    "generic-array",
    "getrandom",
    "gif",
    "glob",
    "half",
    "hashbrown",
    "hashlink",
    "heck",
    "hmac",
    "html5ever",
    "http",
    "http-body",
    "httparse",
    "httpdate",
    "humantime",
    "hyper",
    "hyper-tls",
    "iana-time-zone",
    "indexmap",
    "io-lifetimes",
    "iovec",
    "ipnet",
    "is-terminal",
    "itertools",
    "itoa",
    "jpeg-decoder",
    "lazy_static",
    "lebe",
    "libsqlite3-sys",
    "linked-hash-map",
    "lock_api",
    "log",
    "log4rs",
    "log-mdc",
    "mac",
    "matches",
    "matrixmultiply",
    "md-5",
    "memmap",
    "memoffset",
    "mime",
    "mime_guess",
    "minimal-lexical",
    "mio",
    "num",
    "num-complex",
    "num_cpus",
    "num-integer",
    "num-rational",
    "num-traits",
    "object",
    "once_cell",
    "openssl",
]

items = []
had = set()
for i in crates_list:
    name = i["name"]
    path = crates_dir / name

    loc = int(subprocess.getoutput(f"find {path} -name '*.rs' | grep -v target  | xargs wc -l").splitlines()[
                  -1].strip().split(" ")[0])

    out = subprocess.getoutput(f"du -ad 0 --bytes {path}").strip().split("\t")[0]
    if loc != 0:
        if loc not in had and name in in_list:
            # print(name, "-", loc, "-", out)
            print(out)
            items.append((name, path, loc))
            had.add(loc)
#
#
# def analyze(name, path, loc):
#     out = crates_dir / f"codex-analyis-of-{name}.codex"
#     status = subprocess.run([
#         "cargo", "run", "--", "analyse", "--file", path, "--output", out
#     ], cwd=script_dir / "code-exploration-services")
#     status.check_returncode()
#
#     out = subprocess.getoutput(f"du --bytes {out}")
#     with open("results.txt", "a") as f:
#         print(name, loc, out, file=f)
#
#
# with Pool(14) as p:
#     p.starmap(analyze, items)


# for i in glob.glob(f"{crates_dir}/*.codex"):
#     p = Path(i)
#     out = str(p) + ".zst"
#     subprocess.run(["zstd", p]).check_returncode()
#     out = subprocess.getoutput(f"du --bytes {out}")
#     with open("results_compressed.txt", "a") as f:
#         print(p.name, out, file=f)
