async function refresh (refreshConfig){
    var curtab = await browser.tabs.query(
        {"url":refreshConfig.get_url_pattern()}
    );
    if (curtab.length != 1) {
        return;
    }
    curtab = curtab.id;
    browser.tabs.reload(curtab, {bypassCache: true});
}

async function getOpenTabs() {
    var curtabs = await browser.tabs.query({}).then();
    curtabs = curtabs.map(function (tab) {
        return {"id": tab.id, "url": tab.url};
    });
    console.log("Returning :");
    console.log(curtabs)
    return curtabs;
}

function handleMessage(request, sender, sendResponse) {
  if (request.content.func === "set_refresh") {
      const { set_refresh } = wasm_bindgen;
      var rconfig = set_refresh(
            request.content.url,
            request.content.urlText, 
            request.content.urlTrype,
            request.content.time,
            request.content.pause,
            request.content.sticky,
      );
      console.log(rconfig);
      return Promise.resolve({response: "Refresh set"});
  }
}

browser.runtime.onMessage.addListener(handleMessage);
