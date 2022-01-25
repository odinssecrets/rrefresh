use js_sys::Promise;
use num_enum::TryFromPrimitive;
use std::convert::TryInto;
use std::time::Duration;
use url::{ParseError, Url};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::window;

pub mod macros {
    macro_rules! log {
        ( $( $t:tt )* ) => {
            web_sys::console::log_1(&format!( $( $t )* ).into());
        }
    }
    pub(crate) use log;
}

pub async fn sleep(duration: Duration) {
    JsFuture::from(Promise::new(&mut |yes, _| {
        window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                &yes,
                duration.as_millis() as i32,
            )
            .unwrap();
    }))
    .await
    .unwrap();
}

#[wasm_bindgen]
pub fn update_context_menu() {
    macros::log!("Test menu item");
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

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct RefreshConfig {
    site: String,
    url_pattern: String,
    pub refresh_time: u32,
    pub pause_on_typing: bool,
    pub sticky_reload: bool,
}

#[wasm_bindgen]
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
enum URL_TYPE {
    Domain,
    Subpath,
    FullUrl,
}

fn generate_url_pattern(url: &str, url_type: URL_TYPE) -> String {
    let url: URL = parse_url(url);
    match url_type {
        URL_TYPE::Domain => format!("{}://{}/{}", "*", url.domain, "*"),
        URL_TYPE::Subpath => format!("{}://{}", "*", url.path),
        URL_TYPE::FullUrl => format!("{}", url.full_url),
    }
}

#[wasm_bindgen]
pub fn set_refresh(
    site: &str,
    url: &str,
    url_type: u32,
    time_in_sec: u32,
    pause_on_typing: bool,
    sticky_reload: bool,
) -> RefreshConfig {
    let match_str = generate_url_pattern(url, url_type.try_into().unwrap());
    let config = RefreshConfig {
        site: site.to_string(),
        url_pattern: match_str,
        refresh_time: time_in_sec,
        pause_on_typing: pause_on_typing,
        sticky_reload: sticky_reload,
    };
    refresh(config.clone());
    config
}

#[wasm_bindgen]
extern "C" {
    pub fn doBgCall(function_name: &str, args: &str);
    pub fn refresh(refresh_config: RefreshConfig);
}
