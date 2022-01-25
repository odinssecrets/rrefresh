function onCreated() {
    console.log("Testing");
}

function listener(info, tab) {
    console.log("listening");
}

function shimHandler(message) {
    if (message.function === "doButtonCreate") {
        doButtonCreate();
    }
}
browser.runtime.onMessage.addListener(shimHandler);

function doButtonCreate () {
    browser.menus.create({
          id: "reload",
          title: "Reload",
          contexts: ["all"]
    }, onCreated);

    browser.menus.onClicked.addListener(listener);
}

// This is our recommended way of loading WebAssembly.
// https://developers.google.com/web/updates/2018/04/loading-wasm
(async () => {
  const fetchPromise = fetch('rrefresh_bg.wasm');
  const { instance } = await WebAssembly.instantiateStreaming(fetchPromise);
  console.log(instance.exports);
})();

