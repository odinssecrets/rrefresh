#![feature(once_cell)]
#![feature(async_closure)]
#![allow(dead_code)]

mod rr_error;
mod rr_storage;
mod util;

use rr_storage::load_items;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub async fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    match load_items().await {
        Ok(_) => {}
        Err(_) => panic!("Failed to load data from local browser storage"),
    };
}
