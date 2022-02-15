function sendMessage(data, responseHandler, errorHandler) {
    return browser.runtime.sendMessage({ 
        content: data
    }).then( responseHandler, errorHandler );
}

function messageError(err) {
    console.log("Send Message failure:");
    console.log(err);
}

function pauseRefresh() {
    console.log("Trying to pause");
    sendMessage(
        {func: "set_pause", url: window.location.href, reason: "keydown"}, 
        function () {
            if (localStorage.pauseTimer) {
                window.clearTimeout(localStorage.pauseTimer);
            }
            // Three seconds of buffer time before restarting refresh timer
            // after unpausing
            localStorage.pauseTimer = window.setTimeout(unpauseRefresh, 3000);
        },
        messageError
    );
}

function unpauseRefresh() {
    sendMessage(
        { func: "remove_pause", url: window.location.href },
        function () {
            if (localStorage.pauseTimer) {
                window.clearTimeout(localStorage.pauseTimer);
            }
        },
        messageError
    );
}

function setScroll(message) {
    console.log("Setting scoll:");
    console.log(message);
    if (message.response) {
        if (localStorage.rr_lastScrollY) {
            window.scrollTo(window.scrollX, parseInt(localStorage.rr_lastScrollY));
        }
    }
}

function getSavedScroll() {
    console.log("Saved scrollY is " + localStorage.rr_lastScrollY);
    sendMessage (
        { func: "is_sticky", url: window.location.href },
        setScroll,
        messageError
    );
}

function connectedToBackend() {
    window.clearInterval(localStorage.waitUntilLoaded);
    console.log("Got connection to backend:");
    getSavedScroll();
    document.addEventListener('keydown', pauseRefresh);
    document.addEventListener('scroll', function() {
        localStorage.rr_lastScrollY = window.scrollY;
    });
}

function run() {
    localStorage.waitUntilLoaded = window.setInterval( 
        function () {sendMessage ({func: "trigger_tab_reload"}, connectedToBackend, messageError);},
        100
    );
}
run();
