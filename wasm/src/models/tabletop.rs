use wasm_bindgen::prelude::*;

#[allow(dead_code)]
#[wasm_bindgen]
pub struct Tabletop {
    width: u32,
    height: u32,
}

#[wasm_bindgen]
impl Tabletop {
    pub fn new(width: u32, height: u32) -> Tabletop {
        Tabletop { width, height }
    }
}
