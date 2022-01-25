async function load_wasm() {
    // Load the wasm file by awaiting the Promise returned by `wasm_bindgen`.
    return wasm_bindgen('/wasm/rrefresh_bg.wasm');
}

function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

async function updateSelectedUrl () {
    // Load the parse_url function from the wasm
    const { parse_url } = wasm_bindgen;

    // Parse the url
    const curtab = await browser.tabs.query({"active":true});
    var parsed_url = parse_url(curtab[0].url);

    // Set label text
    var radios = document.forms["input-items"].elements["select"];
    var urlLabel = document.getElementById("selected-url");
    if (radios[0].checked) {        //domain
        urlLabel.textContent = parsed_url.get_domain();
    }
    else if (radios[1].checked) {   //subpath
        urlLabel.textContent = parsed_url.get_domain() + parsed_url.get_subpath();
    }
    else if (radios[2].checked) {   //full url
        urlLabel.textContent = parsed_url.get_full_url();
    }
}

async function toggleRefresh() {
    const curtab = await browser.tabs.query({"active":true});
    var radios = document.forms["input-items"].elements["select"];
    var urlType = 0;
    for (var i in [...Array(radios.length).keys()]) {
        if ( radios[i].checked ) {
            urlType = i;
        }
    }

    var refreshEnable   = document.forms["input-items"].elements["refresh-enabled"];
    var refreshTime     = document.forms["input-items"].elements["refresh-time"];
    var refreshPause    = document.forms["input-items"].elements["refresh-pause"];
    var refreshSticky   = document.forms["input-items"].elements["refresh-sticky"];
    var urlLabel        = document.getElementById("selected-url");
    var data            = {
        func: "set_refresh",
        url: curtab[0].url,
        urlText: urlLabel.textContent, 
        urlTrype: parseInt(urlType),
        time: refreshTime.valueAsNumber,
        pause: refreshPause.checked,
        sticky: refreshSticky.checked,
    };
    if (refreshEnable.checked) {
        sendMessage(data);
    }
}

function handleResponse(message) {
    //console.log(`background script sent a response: ${message.response}`);
}

function handleError(error) {
    console.log(`Error: ${error}`);
}

function sendMessage(content) {
    const sending = browser.runtime.sendMessage({content: content});
    sending.then(handleResponse, handleError);
}

async function setup() {
    var radios = document.forms["input-items"].elements["select"];
    for(var i = 0, max = radios.length; i < max; i++) {
        radios[i].onclick = updateSelectedUrl;
    }

    var applyRefresh = document.forms["input-items"].elements["apply"];
    applyRefresh.onclick = toggleRefresh;

    await load_wasm(); 
    updateSelectedUrl();
}

setup();
