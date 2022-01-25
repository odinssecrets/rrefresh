mod util;

use util::macros::log;
use wasm_bindgen::prelude::*;
use web_sys::*;

#[wasm_bindgen(start)]
pub async fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
}
