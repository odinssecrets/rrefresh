async function refresh (tab_index){
    browser.tabs.reload(tab_index, {bypassCache: true});
}

async function getOpenTabs() {
    var curtabs = await browser.tabs.query({}).then();
    curtabs = curtabs.map(function (tab) {
        return {"id": tab.id, "url": tab.url};
    });
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
      return Promise.resolve({response: "Refresh set"});
  }
}

browser.runtime.onMessage.addListener(handleMessage);
