#![feature(once_cell)]

mod util;
use util::macros::log;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub async fn main() {
    log!("Initializing RRefresh");
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
}
