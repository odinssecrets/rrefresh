use crate::util::{apply_refresh, macros, RefreshConfig, RREFRESH_STORAGE};
use wasm_bindgen::prelude::*;

pub fn save_items(items: Vec<(String, String)>) -> Result<(), String> {
    let window = match web_sys::window() {
        Some(w) => w,
        None => return Err("Failed to open JS window object".to_string()),
    };
    if let Ok(Some(local_storage)) = window.local_storage() {
        for item in items.iter() {
            match local_storage.set_item(&item.0, &item.1) {
                Ok(_) => {}
                Err(_) => {
                    macros::log!("Failed to store value {:?}", item);
                }
            }
        }
        Ok(())
    } else {
        Err("Failed to get local storage".to_string())
    }
}

pub async fn load_items() -> Result<(), String> {
    let window = match web_sys::window() {
        Some(w) => w,
        None => return Err("Failed to open JS window object".to_string()),
    };
    if let Ok(Some(local_storage)) = window.local_storage() {
        let storage_keys = js_sys::Object::keys(&local_storage);
        macros::log!("Local storage contains: {:?}", storage_keys);
        for key in storage_keys.iter() {
            let str_key = key.as_string().expect("Failed to get key");
            if let Ok(Some(item)) = local_storage.get(&str_key) {
                let mut cfg: RefreshConfig = match serde_json::from_str(&item) {
                    Ok(c) => c,
                    Err(_) => {
                        macros::log!("Failed to deserialize {:?}", item);
                        continue;
                    }
                };
                macros::log!("Loaded saved value: {:?}", cfg);
                cfg.timer = apply_refresh(cfg.refresh_time as u64, &cfg);
                RREFRESH_STORAGE.lock().await.insert(str_key, cfg);
            };
        }
        Ok(())
    } else {
        Err("Failed to get local storage".to_string())
    }
}

#[wasm_bindgen]
pub async fn save_to_storage() {
    let items: Vec<(String, String)> = RREFRESH_STORAGE
        .lock()
        .await
        .iter()
        .map(|(k, v)| (k.to_string(), serde_json::to_string(v).unwrap()))
        .collect();
    match save_items(items) {
        Ok(_) => {
            macros::log!("Saved items to local browser storage");
        }
        Err(_) => panic!("Failed to save items to local browser storage"),
    };
}
