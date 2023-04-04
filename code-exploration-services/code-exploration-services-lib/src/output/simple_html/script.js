document.getElementById("change-theme").onchange = function() {
    const new_theme = this.value;
    for (const i of document.getElementsByClassName("change-theme")) {
        i.className = `change-theme ${new_theme}`;
    }
};
