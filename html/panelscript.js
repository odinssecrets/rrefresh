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
    const curWindow = await browser.windows.getLastFocused(); 
    const curtab = await browser.tabs.query({"active":true, "windowId": curWindow.id});
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
    urlLabel.title = urlLabel.textContent;
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

async function getDefaultConfig() {
    if (localStorage.defaultConfig) {
        return JSON.parse(localStorage.defaultConfig);
    }
    var content = {
        func: "default_refresh_config",
    };
    try {
        var response = await browser.runtime.sendMessage( 
            {content: content}
        );
        var config = response.data;
        localStorage.defaultConfig = JSON.stringify(config);
        return config;
    }
    catch (error) {
        console.log("Failed to load default config");
        console.log(error);
    }
}

async function resetRefresh() {
    removeRefresh();
    var config = await getDefaultConfig();
    await setConfig(config);
}

async function pauseRefresh() {
    var pause = document.forms["input-items"].elements["pause"];
    var data = {
        func: "",
        url: await getSite(),
        reason: "button"
    };
    if (pause.value === "Pause") {
        pause.value = "Unpause";
        data.func = "set_pause";
    }
    else {
        pause.value = "Pause";
        data.func = "remove_pause";
    }
    await browser.runtime.sendMessage({ 
        content : data
    });
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
    var pause           = document.forms["input-items"].elements["pause"];

    refreshEnable.checked = config.Enabled;
    refreshTime.value = config.RefreshTime;
    refreshPause.checked = config.PauseOnTyping;
    refreshSticky.checked = config.StickyReload; 
    urlLabel.textContent = config.UrlPattern;
    urlLabel.title = config.UrlPattern;
    if (config.paused) {
        pause.value = "Unpause";
    } else {
        pause.value = "Pause";
    }
}

function switchTab(newTab) {
	var tabs = document.getElementsByClassName("tabs");	
	var tabBtns = document.getElementsByClassName("tablinks");	
	for (var i = 0; i < tabs.length; i++) {
		if (tabs[i].id === newTab) {
			tabs[i].hidden = false;
            tabBtns[i].classList.add("active");
		}
		else {
			tabs[i].hidden = true;
            tabBtns[i].classList.remove("active");
		}
	}
}

async function getAllConfigs() {
    var content = {
        func: "get_all_configs",
    };
    try {
        var response = await browser.runtime.sendMessage( 
            {content: content}
        );
        return response.data;
    }
    catch (error) {
        console.log("Failed to load all configs");
        console.log(error);
        return null;
    }
}

async function setEntries() {
    var configs = await getAllConfigs();
    if (!configs || configs.length == 0) {
        return;
    }
    var entryTable = document.getElementById("entry-table");
    Object.entries(configs).forEach(config => {
        config = config[1];
        var newRow = document.createElement("div");
        var siteData = document.createElement("div");
        var siteText = document.createTextNode(config.UrlPattern);
        var active = document.createElement("div");
        newRow.classList.add("tr");
        siteData.classList.add("td");
        siteData.title = config.UrlPattern;
        siteData.appendChild(siteText);
        active.classList.add("td");
        if (config.Enabled) {
            active.classList.add("active-td");
        }
        else {
            active.classList.add("inactive-td");
        }
        newRow.appendChild(siteData);
        newRow.appendChild(active);
        var newTable = document.createElement("table");
        Object.entries(config).forEach(([key,value]) => {
            var newTableRow = document.createElement("tr");
            var newData = document.createElement("td");
            var newText = document.createTextNode(value);
            var newHeader = document.createElement("td");
            var newHeaderText = document.createTextNode(key);
            newData.title = value;
            newData.appendChild(newText);
            newHeader.title = key;
            newHeader.appendChild(newHeaderText);
            newTableRow.appendChild(newHeader);
            newTableRow.appendChild(newData);
            newTable.append(newTableRow);
        });
        var hiddenRow = document.createElement("div");
        hiddenRow.appendChild(newTable);
        hiddenRow.hidden = true;
        newRow.classList.add("tr");
        newRow.onclick = function() {
            if (newRow.nextSibling.hidden) {
                newRow.nextSibling.hidden = false;
            }
            else {
                newRow.nextSibling.hidden = true;
            }
        };
        entryTable.appendChild(newRow);
        entryTable.appendChild(hiddenRow);
    });
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
    var pause = document.forms["input-items"].elements["pause"];
    pause.onclick = pauseRefresh;

	var setRefreshTab = document.getElementById("set-refresh-tab");
	setRefreshTab.onclick = function(){switchTab("set-refresh")};
    setRefreshTab.classList.add("active");
	var refreshEntriesTab = document.getElementById("refresh-entries-tab");
	refreshEntriesTab.onclick = function(){switchTab("refresh-entries")};

    await load_wasm(); 
    await loadConfig();
    await setEntries();
}

setup();
