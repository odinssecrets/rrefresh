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

/// Creates a timer in the browser which refreshes a tab every time the
/// timer is triggered. This is not updated automatically with the RefreshConfig,
/// it must be removed and re-created.
///
/// Returns an result with the id of the timer on success
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

/// Remove a timer from the browser
///
/// # Arguments
///
/// * `timer_id` - The id of the timer to remove
pub fn remove_window_timer(timer_id: i32) -> () {
    window().unwrap().clear_interval_with_handle(timer_id);
}

/// The data from a browser tab struct that we store locally
#[derive(Debug, Serialize, Deserialize)]
pub struct Tab {
    pub id: u32,
    url: String,
    pub active: bool,
}

impl Tab {
    /// Creates a new tab
    ///
    /// # Arguments
    ///
    /// * `id` - Browser's id for the tab
    /// * `url` - The url string of the tab
    /// * `active` - Boolean value for if this is the window's active tab
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
