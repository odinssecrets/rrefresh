use crate::rr_error::rr_error;
use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::TryInto;
use std::lazy::SyncLazy;
use std::sync::Mutex;
use std::time::Duration;
use url::Url;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::window;

#[allow(dead_code)] // There are functions that are used but only from JS calls

static RREFRESH_STORAGE: SyncLazy<Mutex<HashMap<String, RefreshConfig>>> =
    SyncLazy::new(|| Mutex::new(HashMap::new()));

static OPEN_TABS: SyncLazy<Mutex<HashMap<u32, String>>> =
    SyncLazy::new(|| Mutex::new(HashMap::new()));

pub mod macros {
    macro_rules! log {
        ( $( $t:tt )* ) => {
            web_sys::console::log_1(&format!( $( $t )* ).into());
        }
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
}

impl Tab {
    pub fn new(id: u32, url: &str) -> Tab {
        Tab {
            id,
            url: url.to_string(),
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
}

#[wasm_bindgen]
pub fn parse_url(url: &str) -> URL {
    let cur_tab_url = Url::parse(url).expect("Failed parsing url");
    let path = cur_tab_url.path().to_string();
    let domain = cur_tab_url.host().expect("Failed to get host").to_string();
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
    pub refresh_time: u32,
    pub pause_on_typing: bool,
    pub sticky_reload: bool,
    timer: i32,
}

impl RefreshConfig {
    pub fn get_site(&self) -> String {
        return self.site.clone();
    }
    pub fn get_url_pattern(&self) -> String {
        return self.url_pattern.clone();
    }
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u32)]
enum UrlType {
    Domain,
    Subpath,
    FullUrl,
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
    let cfg: RefreshConfig = cfg.into_serde().unwrap();
    /*let promise = js_sys::Promise::resolve(&getOpenTabs().await);
    let tabs: JsValue = JsFuture::from(promise).await.unwrap();
    let tabs: Vec<Tab> = tabs.into_serde().unwrap();
    macros::log!("Testing tabs: {:?}", &tabs);

    let tab_id = tabs
        .into_iter()
        .filter(|tab| tab.url == cfg.get_url_pattern())
        .collect::<Vec<Tab>>()[0]
        .id;*/
    for (k, v) in OPEN_TABS.lock().unwrap_throw().iter() {
        if cfg.site == *v {
            macros::log!("Refreshing tab {:?}", v);
            refresh(*k);
        }
    }
}

#[wasm_bindgen]
pub async fn set_refresh(
    site: String,
    url: String,
    url_type: u32,
    time_in_sec: u32,
    pause_on_typing: bool,
    sticky_reload: bool,
) -> () {
    match remove_refresh(site.clone()).await {
        Ok(_) => {}
        Err(_) => {}
    }

    let match_str = generate_url_pattern(&url, url_type.try_into().unwrap());
    let mut config = RefreshConfig {
        site: site.to_string(),
        url_pattern: match_str,
        refresh_time: time_in_sec,
        pause_on_typing: pause_on_typing,
        sticky_reload: sticky_reload,
        timer: 0,
    };
    let timer = create_window_timer(Duration::from_secs(time_in_sec as u64), &config);
    config.timer = match timer {
        Ok(t) => t,
        Err(_) => panic!("Error setting timer"),
    };
    RREFRESH_STORAGE.lock().unwrap_throw().insert(site, config);
}

#[wasm_bindgen]
pub async fn remove_refresh(site: String) -> Result<(), JsValue> {
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
pub fn update_tabs(tab_id: u32, site: String) -> () {
    OPEN_TABS.lock().unwrap_throw().insert(tab_id, site);
}

#[wasm_bindgen]
pub fn remove_tab(tab_id: u32) -> () {
    OPEN_TABS.lock().unwrap_throw().remove(&tab_id);
}
#[wasm_bindgen]
extern "C" {
    pub fn refresh(tab_index: u32);
    async fn getOpenTabs() -> JsValue;
}
