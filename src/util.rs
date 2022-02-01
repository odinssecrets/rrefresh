use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::TryInto;
use std::lazy::SyncLazy;
use std::sync::Mutex;
use std::time::Duration;
use url::Url;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::window;

static RREFRESH_STORAGE: SyncLazy<Mutex<HashMap<String, RefreshConfig>>> =
    SyncLazy::new(|| Mutex::new(HashMap::new()));

static OPEN_TABS: SyncLazy<Mutex<HashMap<u32, Tab>>> = SyncLazy::new(|| Mutex::new(HashMap::new()));

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
    url_type: u8,
    pub refresh_time: u32,
    pub pause_on_typing: bool,
    pub sticky_reload: bool,
    timer: i32,
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
    pub fn match_url(&self, url: &str) -> bool {
        // TODO update this to do real pattern matching
        self.url_pattern == url
    }
    pub fn default() -> RefreshConfig {
        RefreshConfig::default_with_url("")
    }
    pub fn default_with_url(url: &str) -> RefreshConfig {
        RefreshConfig {
            site: url.to_string(),
            url_pattern: url.to_string(),
            url_type: 0,
            refresh_time: 30,
            pause_on_typing: false,
            sticky_reload: false,
            timer: 0,
            enabled: true,
        }
    }
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
enum UrlType {
    Domain,
    Subpath,
    FullUrl,
}

#[wasm_bindgen]
pub fn is_sticky(url: String) -> bool {
    let configs = get_config_by_url(&url);
    for cfg in configs.iter() {
        if cfg.sticky_reload {
            return true;
        }
    }
    false
}

fn get_config_by_url(url: &str) -> Vec<RefreshConfig> {
    let mut results = vec![];
    for (_site, cfg) in RREFRESH_STORAGE.lock().unwrap().iter() {
        if cfg.match_url(url) {
            results.push(cfg.clone());
        }
    }
    results
}

fn generate_url_pattern(url: &str, url_type: UrlType) -> String {
    let url: URL = parse_url(url);
    match url_type {
        UrlType::Domain => format!("{}://{}/{}", "*", url.domain, "*"),
        UrlType::Subpath => format!("{}://{}", "*", url.path),
        UrlType::FullUrl => format!("{}", url.full_url),
    }
}

#[wasm_bindgen]
pub fn refresh_tab(cfg: JsValue) -> () {
    macros::log!("Refreshing tab");
    let cfg: RefreshConfig = cfg.into_serde().unwrap();
    for (k, v) in OPEN_TABS.lock().unwrap_throw().iter() {
        if cfg.site == *v.url {
            macros::log!("Refreshing tab {:?}", v);
            refresh(*k);
        }
    }
}

#[wasm_bindgen]
pub fn load_refresh_config(site: String) -> RefreshConfig {
    macros::log!("Loading from: {:?}", RREFRESH_STORAGE);
    match RREFRESH_STORAGE.lock().unwrap().get(&site) {
        Some(cfg) => cfg.clone(),
        None => {
            let mut url: String = "".to_string();
            for (_id, tab) in OPEN_TABS.lock().unwrap_throw().iter() {
                if tab.active == true {
                    url = tab.url.clone();
                }
            }
            RefreshConfig::default_with_url(&url)
            //panic!("No config found for this site"),
        }
    }
}

#[wasm_bindgen]
pub async fn set_refresh(
    site: String,
    url_type: u8,
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
    let match_str = generate_url_pattern(&site, url_type.try_into().unwrap_throw());
    let mut config = RefreshConfig {
        site: site.to_string(),
        url_pattern: match_str,
        url_type: url_type,
        refresh_time: time_in_sec,
        pause_on_typing: pause_on_typing,
        sticky_reload: sticky_reload,
        timer: 0,
        enabled: refresh_enabled,
    };
    if config.enabled {
        macros::log!("Setting timer");
        let timer = create_window_timer(Duration::from_secs(time_in_sec as u64), &config);
        config.timer = match timer {
            Ok(t) => t,
            Err(_) => panic!("Error setting timer"),
        };
    }
    macros::log!("{:?}", RREFRESH_STORAGE);
    RREFRESH_STORAGE.lock().unwrap_throw().insert(site, config);
}

#[wasm_bindgen]
pub async fn remove_refresh(site: String) -> Result<(), JsValue> {
    macros::log!("Removing refresh");
    macros::log!("{:?}", RREFRESH_STORAGE);
    match RREFRESH_STORAGE.lock().unwrap_throw().get(&site) {
        Some(cfg) => {
            macros::log!("Removed refresh for {}", site);
            remove_window_timer(cfg.timer);
            Ok(())
        }
        None => Err(JsValue::from_str(&format!("{} not found in storage", site))),
    }
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
extern "C" {
    pub fn refresh(tab_index: u32);
    async fn getOpenTabs() -> JsValue;
}
