async function run() {
    await wasm_bindgen(chrome.extension.getURL('rrefresh_bg.wasm'));
}

run();