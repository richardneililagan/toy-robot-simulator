use wasm_bindgen::prelude::*;

// :: ---

pub struct Tabletop {
    width: i32,
    height: i32,
}

impl Tabletop {
    pub fn new(width: i32, height: i32) -> Self {
        Tabletop { width, height }
    }

    pub fn get_width(&self) -> i32 {
        self.width
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }
}

// :: ---

#[cfg(test)]
mod tests {
    use super::*;
}
