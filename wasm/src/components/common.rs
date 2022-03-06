use wasm_bindgen::prelude::*;

// :: ---

#[derive(Debug, PartialEq)]
#[wasm_bindgen]
pub enum Orientation {
    North,
    East,
    West,
    South,
}

#[wasm_bindgen]
impl Orientation {
    pub fn parse(plaintext: &str) -> Result<Orientation, String> {
        match plaintext.to_uppercase().as_str() {
            "NORTH" => Ok(Orientation::North),
            "EAST" => Ok(Orientation::East),
            "WEST" => Ok(Orientation::West),
            "SOUTH" => Ok(Orientation::South),

            _ => Err(format!("Unexpected orientation: {}", plaintext)),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[wasm_bindgen]
pub struct Position {
    pub x: i32,
    pub y: i32,
}
