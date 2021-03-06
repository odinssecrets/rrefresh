async function refresh (tab_index) {
    browser.tabs.reload(tab_index, {bypassCache: true});
}

async function getOpenTabs() {
    const curWindow = await browser.windows.getLastFocused(); 
    var curtabs = await browser.tabs.query({}).then();
    curtabs = curtabs.map(function (tab) {
        return {"id": tab.id, "url": tab.url, "active": (tab.active && tab.windowId == curWindow.id)};
    });
    return curtabs;
}

async function handleMessage(request, sender, sendResponse) {
    //console.log("Received message: " + request.content.func);
    //console.log(request);
    try {
        if (request.content.func === "set_refresh") {
            const { set_refresh } = wasm_bindgen;
            var set_promise = await set_refresh(
                request.content.url,
                request.content.urlType,
                request.content.subpathDepth,
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
            var config = await load_refresh_config(request.content.url);
            config = generateJsConfig(config); 
            return Promise.resolve({
                response: "Record found for " + request.content.url,
                data: config
            });
        }
        else if (request.content.func === "default_refresh_config") {
            const { default_refresh_config } = wasm_bindgen;
            var config = await default_refresh_config();
            config = generateJsConfig(config);
            return Promise.resolve({
                response: "Default config loaded",
                data: config
            });
        }
        else if (request.content.func === "get_all_configs") {
            const { get_all_configs } = wasm_bindgen;
            var config = await get_all_configs();
            var configList = [];
            config.forEach(cfg => {
                configList.push(generateJsConfig(cfg));
            });
            return Promise.resolve({
                response: "All configs loaded",
                data: configList
            });
        }
        else if (request.content.func === "is_sticky") {
            const { is_sticky } = wasm_bindgen;
            var isSticky = await is_sticky(request.content.url);
            return Promise.resolve({
                response: isSticky
            });
        }
        else if (request.content.func === "set_pause") {
            const { set_pause } = wasm_bindgen;
            var result = await set_pause(request.content.url, request.content.reason);
            return Promise.resolve({
                response: "Pause set"
            });
        }
        else if (request.content.func === "remove_pause") {
            const { remove_pause } = wasm_bindgen;
            await remove_pause(request.content.url);
            return Promise.resolve({
                response: "Pause removed"
            });
        }
        else if (request.content.func === "trigger_tab_reload") {
            const { trigger_tab_reload } = wasm_bindgen;
            await trigger_tab_reload();
            return Promise.resolve({
                response: "Triggered tab reload"
            });
        }
    }
    catch (Error) {
        console.log("Got an error handling message");
        console.log(Error);
    }
}

function generateJsConfig(config) {
    return {
        Site: config.get_site(), 
        UrlPattern: config.get_url_pattern(),
        UrlType: config.url_type,
        SubpathDepth: config.subpath_depth,
        Enabled: config.enabled,
        StickyReload: config.sticky_reload,
        PauseOnTyping: config.pause_on_typing,
        Paused: config.paused,
        RefreshTime: config.refresh_time,
    };
}

function startRrMessageHandler() {
    browser.runtime.onMessage.addListener(handleMessage);
}

function updateTabs(tabId, changeInfo, tab) {
    const { update_tabs } = wasm_bindgen;
    update_tabs(tabId, tab.url, tab.active);
}

function removeTab(tabId, removeInfo) {
    const { remove_tab } = wasm_bindgen;
    remove_tab(tabId);
}

async function saveToStorage(){
    const { save_to_storage } = wasm_bindgen;
    await save_to_storage();
}

async function run() {
    await wasm_bindgen(browser.runtime.getURL('wasm/rrefresh_bg.wasm'));
    window.onbeforeunload = saveToStorage;
    browser.tabs.onUpdated.addListener(updateTabs);
    browser.tabs.onRemoved.addListener(removeTab);
    startRrMessageHandler();
}
run();
