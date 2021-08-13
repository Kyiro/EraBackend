const DISCORD_OAUTH2 = "https://discord.com/api/oauth2/authorize?client_id=874789085737287741&redirect_uri=http%3A%2F%2F127.0.0.1%3A60101%2Fid%2Fapi%2Fdiscord%2Foauth2&response_type=code&scope=identify";

var account;

// skidded
function getCookie(name) {
    const value = `; ${document.cookie}`;
    const parts = value.split(`; ${name}=`);
    if (parts.length === 2) return parts.pop().split(';').shift();
}

function getAvatar(id, avatar) {
    return `https://cdn.discordapp.com/avatars/${id}/${avatar}.webp?size=128`
}

var o = (url) => {
    window.open("https://" + url);
};

var s = (tab) => {
    tabU = tab.charAt(0).toUpperCase() + tab.slice(1);
    document.title = tabU + " / Project Era";
    if (tab == "account" && !account) {
        location.href = DISCORD_OAUTH2;
        return;
    }
    for (article of document.getElementsByTagName("article")) {
        if (article.id == tab) {
            location.href = '#' + tab;
            article.classList.remove("disabled");
        }
        else article.classList.add("disabled");
    }
};

window.onload = async () => {
    let req = await fetch('/id/api/user/@me');
    if (req.status == 200) {
        let data = await req.json();
        
        let img = document.getElementById('account-img');
        img.src = getAvatar(data["discord_id"], data["discord_avatar"]);
        img.classList.add("avatar");
        document.getElementById('account-name').innerHTML = data["display_name"];
        account = data;
    }
    const tab = location.href.split("#")[1];
    if (tab) s(tab);
    else s("home");
}