use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::TryInto;
use std::lazy::SyncLazy;
use std::time::Duration;
use tokio::sync::Mutex;
use url::Url;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::window;

pub static RREFRESH_STORAGE: SyncLazy<Mutex<HashMap<String, RefreshConfig>>> =
    SyncLazy::new(|| Mutex::new(HashMap::new()));

static OPEN_TABS: SyncLazy<std::sync::Mutex<HashMap<u32, Tab>>> =
    SyncLazy::new(|| std::sync::Mutex::new(HashMap::new()));

pub mod macros {
    #[cfg(debug_assertions)]
    macro_rules! log {
        ( $( $t:tt )* ) => {
            web_sys::console::log_1(&format!( $( $t )* ).into());
        }
    }
    #[cfg(not(debug_assertions))]
    macro_rules! log {
        ( $( $t:tt )* ) => {};
    }
    pub(crate) use log;
}

pub fn create_window_timer(duration: Duration, cfg: &RefreshConfig) -> Result<i32, JsValue> {
    let cb = Closure::wrap(Box::new(|a| {
        refresh_tab(a);
    }) as Box<dyn Fn(JsValue)>);
    let timer_id = window()
        .unwrap()
        .set_interval_with_callback_and_timeout_and_arguments_1(
            cb.as_ref().unchecked_ref(),
            duration.as_millis().try_into().unwrap(),
            &JsValue::from_serde(cfg).unwrap(),
        );
    cb.forget();
    timer_id
}

pub fn remove_window_timer(time_id: i32) -> () {
    window().unwrap().clear_interval_with_handle(time_id);
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tab {
    pub id: u32,
    url: String,
    pub active: bool,
}

impl Tab {
    pub fn new(id: u32, url: &str, active: bool) -> Tab {
        Tab {
            id,
            url: url.to_string(),
            active,
        }
    }
    pub fn get_url(&self) -> String {
        return self.url.clone();
    }
}

#[wasm_bindgen]
pub struct URL {
    path: String,
    domain: String,
    full_url: String,
}

#[wasm_bindgen]
impl URL {
    pub fn get_subpath(&self) -> String {
        self.path.clone()
    }
    pub fn get_domain(&self) -> String {
        self.domain.clone()
    }
    pub fn get_full_url(&self) -> String {
        self.full_url.clone()
    }
    pub fn default() -> URL {
        return URL {
            path: "".to_string(),
            domain: "".to_string(),
            full_url: "".to_string(),
        };
    }
}

#[wasm_bindgen]
pub fn parse_url(url: &str) -> URL {
    let cur_tab_url = match Url::parse(url) {
        Ok(u) => u,
        Err(_) => return URL::default(),
    };
    let path = cur_tab_url.path().to_string();
    let domain = match cur_tab_url.host() {
        Some(s) => s.to_string(),
        None => "".to_string(),
    };
    URL {
        path,
        domain,
        full_url: url.to_string(),
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshConfig {
    site: String,
    url_pattern: String,
    pub url_type: u8,
    pub subpath_depth: u32,
    pub refresh_time: u32,
    pub pause_on_typing: bool,
    pub paused: bool,
    pub sticky_reload: bool,
    pub timer: i32,
    pub enabled: bool,
}

#[wasm_bindgen]
impl RefreshConfig {
    pub fn get_site(&self) -> String {
        self.site.clone()
    }
    pub fn get_url_pattern(&self) -> String {
        self.url_pattern.clone()
    }
    pub fn set_url_pattern(&mut self) -> () {
        let url: URL = parse_url(&self.site);
        self.url_pattern = match self.url_type.try_into().unwrap() {
            UrlType::Domain => url.domain,
            UrlType::Subpath => {
                let path_parts = url.path.split("/").collect::<Vec<&str>>();
                url.domain + &path_parts[0..(self.subpath_depth + 1) as usize].join("/")
            }
            UrlType::FullUrl => url.full_url,
        };
    }
    pub fn match_url(&self, url: &str) -> bool {
        match self.url_type.try_into().unwrap() {
            UrlType::Domain => parse_url(url).domain == self.url_pattern,
            UrlType::Subpath => parse_url(url).path.starts_with(&self.url_pattern),
            UrlType::FullUrl => self.url_pattern == url,
        }
    }
    pub fn default() -> RefreshConfig {
        RefreshConfig::default_with_url("")
    }
    pub fn default_with_url(url: &str) -> RefreshConfig {
        RefreshConfig {
            site: url.to_string(),
            url_pattern: url.to_string(),
            url_type: 2,
            subpath_depth: 0,
            refresh_time: 30,
            pause_on_typing: false,
            paused: false,
            sticky_reload: false,
            timer: 0,
            enabled: false,
        }
    }
}

#[wasm_bindgen]
pub fn default_refresh_config() -> RefreshConfig {
    RefreshConfig::default()
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
enum UrlType {
    Domain,
    Subpath,
    FullUrl,
}

#[wasm_bindgen]
pub async fn is_sticky(url: String) -> bool {
    let configs = get_config_by_url(&url).await;
    for cfg in configs.iter() {
        if cfg.sticky_reload {
            return true;
        }
    }
    false
}

async fn get_config_by_url(url: &str) -> Vec<RefreshConfig> {
    let mut results = vec![];
    for (_site, cfg) in RREFRESH_STORAGE.lock().await.iter() {
        if cfg.match_url(url) {
            results.push(cfg.clone());
        }
    }
    results
}

#[wasm_bindgen]
pub fn refresh_tab(cfg: JsValue) -> () {
    let cfg: RefreshConfig = cfg.into_serde().unwrap();
    macros::log!("Refreshing tab with config {:?}", cfg);
    for (k, v) in OPEN_TABS.lock().unwrap_throw().iter() {
        if cfg.match_url(&v.url) {
            macros::log!("Refreshing tab {:?}", v);
            refresh(*k);
        }
    }
}

#[wasm_bindgen]
pub async fn load_refresh_config(site: String) -> RefreshConfig {
    macros::log!("Loading from: {:?}", RREFRESH_STORAGE);
    match RREFRESH_STORAGE.lock().await.get(&site) {
        Some(cfg) => {
            macros::log!("Got config {:?}", cfg);
            cfg.clone()
        }
        None => {
            let mut url: String = "".to_string();
            for (_id, tab) in OPEN_TABS.lock().unwrap_throw().iter() {
                if tab.active == true {
                    url = tab.url.clone();
                }
            }
            let cfg = RefreshConfig::default_with_url(&url);
            macros::log!("Loading default config {:?}", cfg);
            cfg
            //panic!("No config found for this site"),
        }
    }
}

pub fn apply_refresh(time: u64, cfg: &RefreshConfig) -> i32 {
    if cfg.enabled {
        macros::log!("Setting timer");
        match create_window_timer(Duration::from_secs(time), cfg) {
            Ok(t) => t,
            Err(_) => 0,
        }
    } else {
        0
    }
}

#[wasm_bindgen]
pub async fn set_refresh(
    site: String,
    url_type: u8,
    subpath_depth: u32,
    time_in_sec: u32,
    pause_on_typing: bool,
    sticky_reload: bool,
    refresh_enabled: bool,
) -> () {
    // It doesn't matter if this throws an error, we are going to continue anyway
    match remove_refresh(site.clone()).await {
        Ok(_) => {}
        Err(_) => {}
    }
    macros::log!("Setting refresh");
    let mut config = RefreshConfig {
        site: site.to_string(),
        url_pattern: "".to_string(),
        url_type: url_type,
        subpath_depth: subpath_depth,
        refresh_time: time_in_sec,
        pause_on_typing: pause_on_typing,
        paused: false,
        sticky_reload: sticky_reload,
        timer: 0,
        enabled: refresh_enabled,
    };
    config.set_url_pattern();
    config.timer = apply_refresh(time_in_sec as u64, &config);
    macros::log!("{:?}", RREFRESH_STORAGE);
    RREFRESH_STORAGE.lock().await.insert(site, config);
}

#[wasm_bindgen]
pub async fn remove_refresh(site: String) -> Result<(), JsValue> {
    macros::log!("Removing refresh");
    macros::log!("{:?}", RREFRESH_STORAGE);
    match RREFRESH_STORAGE.lock().await.get(&site) {
        Some(cfg) => {
            macros::log!("Removed refresh for {}", site);
            remove_window_timer(cfg.timer);
            Ok(())
        }
        None => Err(JsValue::from_str(&format!("{} not found in storage", site))),
    }
}

#[wasm_bindgen]
pub async fn set_pause(site: String, pause_reason: String) -> Result<(), JsValue> {
    macros::log!("Setting pause for {}", site);
    let mut new_cfg = None;
    let ret_val = match RREFRESH_STORAGE.lock().await.get(&site) {
        Some(cfg) => {
            if (pause_reason == "keydown" && cfg.pause_on_typing) || (pause_reason == "button") {
                if cfg.timer != 0 && cfg.paused == false {
                    let mut tmp_cfg = cfg.clone();
                    macros::log!("Pause set on: {}", site);
                    remove_window_timer(cfg.timer);
                    tmp_cfg.timer = 0;
                    tmp_cfg.paused = true;
                    new_cfg = Some(tmp_cfg);
                }
            } else {
                macros::log!("Did not pause, {:?}", pause_reason);
            }
            Ok(())
        }
        None => Err(JsValue::from_str(&format!("{} not found in storage", site))),
    };
    match new_cfg {
        Some(cfg) => RREFRESH_STORAGE.lock().await.insert(site.clone(), cfg),
        None => None,
    };
    ret_val
}

#[wasm_bindgen]
pub async fn remove_pause(site: String) -> Result<(), JsValue> {
    macros::log!("Removing pause");
    let mut new_cfg = None;
    let ret_val = match RREFRESH_STORAGE.lock().await.get(&site) {
        Some(cfg) => {
            if cfg.timer == 0 && cfg.paused {
                let mut tmp_cfg = cfg.clone();
                let timer = create_window_timer(Duration::from_secs(cfg.refresh_time as u64), &cfg);
                tmp_cfg.timer = match timer {
                    Ok(t) => t,
                    Err(_) => {
                        macros::log!("Failed to set new timer after pause");
                        0
                    }
                };
                tmp_cfg.paused = false;
                new_cfg = Some(tmp_cfg);
            }
            Ok(())
        }
        None => Err(JsValue::from_str(&format!("{} not found in storage", site))),
    };
    match new_cfg {
        Some(cfg) => RREFRESH_STORAGE.lock().await.insert(site, cfg),
        None => None,
    };
    ret_val
}

#[wasm_bindgen]
pub async fn trigger_tab_reload() -> () {
    //macros::log!("Reloading tabs");
    let tabs: Vec<Tab> = getOpenTabs().await.into_serde().unwrap();
    for tab in tabs {
        OPEN_TABS.lock().unwrap_throw().insert(tab.id, tab);
    }
    //macros::log!("Got tabs {:?}", OPEN_TABS);
}

#[wasm_bindgen]
pub fn update_tabs(tab_id: u32, site: String, active: bool) -> () {
    macros::log!("Updating tabs");
    OPEN_TABS
        .lock()
        .unwrap_throw()
        .insert(tab_id, Tab::new(tab_id, &site, active));
}

#[wasm_bindgen]
pub fn remove_tab(tab_id: u32) -> () {
    macros::log!("Removing tab");
    OPEN_TABS.lock().unwrap_throw().remove(&tab_id);
}

#[wasm_bindgen]
pub async fn get_all_configs() -> js_sys::Array {
    let all_configs: Vec<RefreshConfig> = RREFRESH_STORAGE
        .lock()
        .await
        .iter()
        .map(|(_k, v)| v.clone())
        .collect();
    all_configs.into_iter().map(JsValue::from).collect()
}

#[wasm_bindgen]
extern "C" {
    pub fn refresh(tab_index: u32);
    async fn getOpenTabs() -> JsValue;
}
