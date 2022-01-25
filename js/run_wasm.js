async function run() {
    await wasm_bindgen(browser.runtime.getURL('wasm/rrefresh_bg.wasm'));
    // Calls into menus.js to do the context menu setup for the extnesion
    setupMenu();
}
run();
