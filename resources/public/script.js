var account;

var o = (url) => {
    window.open("https://" + url);
};

var s = (tab) => {
    tabU = tab.charAt(0).toUpperCase() + tab.slice(1);
    document.title = tabU + " / Project Era";
    for (article of document.getElementsByTagName("article")) {
        if (article.id == tab) {
            location.href = '#' + tab;
            article.classList.remove("disabled");
        }
        else article.classList.add("disabled");
    }
};

window.onload = () => {
    const tab = location.href.split("#")[1];
    if (tab) s(tab);
    else s("home");
}