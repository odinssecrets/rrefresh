async function load_wasm() {
    // Load the wasm file by awaiting the Promise returned by `wasm_bindgen`.
    return wasm_bindgen('/wasm/rrefresh_bg.wasm');
}

function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

function handleResponse(message) {
    console.log(`background script sent a response: ${message.response}`);
}

function handleError(error) {
    console.log(`Error: ${error}`);
}

async function getSite() {
    const curtab = await browser.tabs.query({"active":true});
    return curtab[0].url;
}

async function updateSelectedUrl () {
    // Load the parse_url function from the wasm
    const { parse_url } = wasm_bindgen;

    var url = await getSite();
    var parsed_url = parse_url(url);

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

async function applyRefresh() {
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
    var url             = await getSite();
    var data            = {
        func: "set_refresh",
        url: url,
        urlType: parseInt(urlType),
        time: refreshTime.valueAsNumber,
        pause: refreshPause.checked,
        sticky: refreshSticky.checked,
        enabled: refreshEnable.checked,
    };
    var set_promise = await browser.runtime.sendMessage({ 
        content : data
    });
    await set_promise;
}

async function resetRefresh() {
    removeRefresh();
    var content = {
        func: "default_refresh_config",
    };
    try {
        var response = await browser.runtime.sendMessage( 
            {content: content}
        );
        var config = response.data;
        await setConfig(config);
    }
    catch (error) {
        console.log("Failed to load default config");
        console.log(error);
    }
}

async function removeRefresh() {
    var content = {
        func: "remove_refresh",
        url: await getSite()
    };
    try {
        var response = await browser.runtime.sendMessage( 
            {content: content}
        );
    }
    catch (error) {
        console.log("Failed to remove refresh");
        console.log(error);
    }
}

async function loadConfig() {
    var content = {
        func: "load_refresh_config",
        url: await getSite()
    };
    try {
        var response = await browser.runtime.sendMessage( 
            {content: content}
        );
        var config = response.data;
        await setConfig(config);
    }
    catch (error) {
        console.log("Failed to load config");
        console.log(error);
    }
}

async function setConfig(config) {
    if (!config) {
        console.log("Error setting config. Config object not found.");
        return;
    }

    var refreshEnable   = document.forms["input-items"].elements["refresh-enabled"];
    var refreshTime     = document.forms["input-items"].elements["refresh-time"];
    var refreshPause    = document.forms["input-items"].elements["refresh-pause"];
    var refreshSticky   = document.forms["input-items"].elements["refresh-sticky"];
    var urlLabel        = document.getElementById("selected-url");

    refreshEnable.checked = config.enabled;
    refreshTime.value = config.refreshTime;
    refreshPause.checked = config.pauseOnTyping;
    refreshSticky.checked = config.stickyReload; 
    urlLabel.textContent = config.url;
    await updateSelectedUrl();
}

async function setup() {
    var radios = document.forms["input-items"].elements["select"];
    for(var i = 0, max = radios.length; i < max; i++) {
        radios[i].onclick = updateSelectedUrl;
    }

    var apply = document.forms["input-items"].elements["apply"];
    apply.onclick = applyRefresh;
    var reset = document.forms["input-items"].elements["reset"];
    reset.onclick = resetRefresh;

    await load_wasm(); 
    await loadConfig();
}

setup();
