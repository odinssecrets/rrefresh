function doBgCall(funcName, args) {
    browser.runtime.sendMessage(
    {
        "function": funcName,
        "args":     args
    });
}
