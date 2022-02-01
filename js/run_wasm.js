function updateTabs(tabId, changeInfo, tab) {
    const { update_tabs } = wasm_bindgen;
    update_tabs(tabId, tab.url, tab.active);
}

function removeTab(tabId, removeInfo) {
    const { remove_tab } = wasm_bindgen;
    remove_tab(tabId);
}

async function run() {
    await wasm_bindgen(browser.runtime.getURL('wasm/rrefresh_bg.wasm'));
    browser.tabs.onUpdated.addListener(updateTabs);
    browser.tabs.onRemoved.addListener(removeTab);
}
run();
