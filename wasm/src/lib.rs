mod models;
mod utils;

use models::*;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// :: ---

#[wasm_bindgen]
pub fn initialize_table(width: u32, height: u32) -> Result<tabletop::Tabletop, JsValue> {
    let tabletop = tabletop::Tabletop::new(width, height);
    Result::Ok(tabletop)
}
