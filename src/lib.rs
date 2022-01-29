#![feature(once_cell)]
#![feature(async_closure)]

mod rr_error;
mod util;
use util::macros::log;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub async fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
}
