function setupMenu() {
    browser.menus.create({
          id: "reload",
          title: "Reload",
          contexts: ["all"]
    }, null);
    
    browser.menus.onClicked.addListener(listener);
}

function listener(info, tab) {
    console.log("listening");
}
