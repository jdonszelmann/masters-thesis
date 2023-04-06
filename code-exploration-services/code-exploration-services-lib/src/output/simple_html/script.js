function change_theme(new_theme) {
    for (const i of document.getElementsByClassName("change-theme")) {
        i.className = `change-theme ${new_theme}`;
    }
}

const highlights = new Map();

function register_highlight(location) {
    for (const i of highlights.keys()) {
        clearTimeout(highlights.get(i));
        for (const el of document.getElementsByClassName(i)) {
            el.classList.remove("highlighted")
        }
    }
    highlights.clear()

    for (const i of document.getElementsByClassName(location)) {
        i.classList.add("highlighted")
    }

    const cancel = setTimeout(() => {
        for (const i of document.getElementsByClassName(location)) {
            i.classList.remove("highlighted")
        }

        highlights.delete(location);
    }, 5000);

    highlights.set(location, cancel);
}

function go_to(location) {
    register_highlight(location);

    const el = document.getElementsByClassName(location)[0];

    el.scrollIntoView({
        behavior: "smooth",
        block: "center",
        inline: "nearest"
    })
}

document.getElementById("change-theme").onchange = function () {
    change_theme(this.value)
};

document.onreadystatechange = () => {
    change_theme(document.getElementById("change-theme").value);

    for (const i of document.getElementsByClassName("outline-header")) {
        i.onclick = () => {
            go_to(i.dataset.outlineClass);
        }
    }
}
