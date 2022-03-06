use wasm_bindgen::prelude::*;

use super::common::*;

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

    /// Checks if an item can be placed on the Tabletop at the position provided.
    pub fn request_place(&self, position: &Position) -> Result<(), String> {
        if position.x >= 0 && position.y >= 0 && position.x < self.width && position.y < self.height
        {
            Ok(())
        } else {
            Err(format!(
                "Position ({}, {}) is out of bounds.",
                position.x, position.y
            ))
        }
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

    #[test]
    fn tabletop_can_only_place_within_its_dimensions() {
        let tabletop = Tabletop::new(5, 5).unwrap();

        // :: should succeed
        assert!(tabletop.request_place(&Position { x: 3, y: 3 }).is_ok());
        assert!(tabletop.request_place(&Position { x: 0, y: 3 }).is_ok());
        assert!(tabletop.request_place(&Position { x: 3, y: 0 }).is_ok());
        assert!(tabletop.request_place(&Position { x: 0, y: 0 }).is_ok());

        // :: should fail
        assert!(tabletop.request_place(&Position { x: -1, y: 3 }).is_err());
        assert!(tabletop.request_place(&Position { x: 1, y: -3 }).is_err());
        assert!(tabletop.request_place(&Position { x: -1, y: -3 }).is_err());
        assert!(tabletop.request_place(&Position { x: 5, y: 3 }).is_err());
        assert!(tabletop.request_place(&Position { x: 1, y: 5 }).is_err());
        assert!(tabletop.request_place(&Position { x: 5, y: 5 }).is_err());
        assert!(tabletop.request_place(&Position { x: 50, y: 5 }).is_err());
        assert!(tabletop.request_place(&Position { x: 5, y: 50 }).is_err());
        assert!(tabletop.request_place(&Position { x: 50, y: 50 }).is_err());
    }
}
