async function refresh (tab_index){
    browser.tabs.reload(tab_index, {bypassCache: true});
}

async function getOpenTabs() {
    var curtabs = await browser.tabs.query({}).then();
    curtabs = curtabs.map(function (tab) {
        return {"id": tab.id, "url": tab.url, "active": tab.active};
    });
    console.log(curtabs);
    return curtabs;
}

async function handleMessage(request, sender, sendResponse) {
    try {
        //console.log("Got message");
        //console.log(request.content);
        if (request.content.func === "set_refresh") {
            const { set_refresh } = wasm_bindgen;
            var set_promise = await set_refresh(
                request.content.url,
                request.content.urlType,
                request.content.time,
                request.content.pause,
                request.content.sticky,
                request.content.enabled,
            );
            return Promise.resolve({
                response: "Refresh set",
                data: set_promise,
            });
        }
        else if (request.content.func === "remove_refresh") {
            const { remove_refresh } = wasm_bindgen;
            await remove_refresh(request.content.url);
            return Promise.resolve({response: "Refresh removed"});
        }
        else if (request.content.func === "load_refresh_config") {
            const { load_refresh_config } = wasm_bindgen;
            var config = load_refresh_config(request.content.site);
            config = {
                site: config.get_site(), 
                url_patter: config.get_url_pattern(),
                enabled: config.enabled,
                stickyReload: config.sticky_reload,
                pauseOnTyping: config.pause_on_typing,
                refreshTime: config.refresh_time,
            };
            return Promise.resolve({
                response: "Record found for " + request.content.site,
                data: config
            });
        }
    }
    catch (Error) {
        console.log("Got an error handling message");
        console.log(Error);
    }
}

browser.runtime.onMessage.addListener(handleMessage);
