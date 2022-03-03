use super::engine_command::EngineCommand;
use wasm_bindgen::prelude::*;

#[allow(dead_code)]
#[wasm_bindgen]
pub struct Tabletop {
    width: u32,
    height: u32,
}

#[wasm_bindgen]
impl Tabletop {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32) -> Self {
        Tabletop { width, height }
    }

    #[wasm_bindgen]
    pub fn evaluate_command(&self, plaintext_command: &str) {
        println!("Command: {}", plaintext_command);
        let _command = EngineCommand::parse(plaintext_command);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tabletop_applies_provided_dimensions() {
        let tabletop = Tabletop::new(5, 5);
        assert_eq!(tabletop.width, 5);
        assert_eq!(tabletop.height, 5);
    }
}
