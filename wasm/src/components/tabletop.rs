use wasm_bindgen::prelude::*;

// :: ---

#[derive(Copy, Clone, PartialEq, Debug)]
#[wasm_bindgen]
pub struct Tabletop {
    width: i32,
    height: i32,
}

#[wasm_bindgen]
impl Tabletop {
    #[wasm_bindgen(constructor)]
    pub fn new(width: i32, height: i32) -> Result<Tabletop, String> {
        match (width, height) {
            (width, height) if width > 0 && height > 0 => Ok(Tabletop { width, height }),

            _ => Err("Tabletop dimensions need to be positive integers.".to_string()),
        }
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

    #[test]
    fn tabletop_is_created_with_correct_dimensions() {
        let width = 5;
        let height = 7;

        let result = Tabletop::new(width, height);
        let tabletop = result.unwrap();

        assert_eq!(tabletop.width, width);
        assert_eq!(tabletop.height, height);
    }

    #[test]
    fn tabletop_requires_positive_nonzero_dimensions() {
        assert!(Tabletop::new(-5, 5).is_err());
        assert!(Tabletop::new(5, -5).is_err());
        assert!(Tabletop::new(-5, -5).is_err());
        assert!(Tabletop::new(0, 5).is_err());
        assert!(Tabletop::new(5, 0).is_err());
        assert!(Tabletop::new(0, 0).is_err());
        assert!(Tabletop::new(0, -10).is_err());
        assert!(Tabletop::new(-10, 0).is_err());
    }
}
