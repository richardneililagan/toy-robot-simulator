#![allow(clippy::unused_unit)]

use wasm_bindgen::prelude::*;

use super::common::*;

// :: ---

#[derive(Clone, PartialEq, Debug)]
#[wasm_bindgen]
pub struct Tabletop {
    width: i32,
    height: i32,

    obstacles: Vec<Position>,
}

#[wasm_bindgen]
impl Tabletop {
    #[wasm_bindgen(constructor)]
    pub fn new(width: i32, height: i32) -> Result<Tabletop, String> {
        match (width, height) {
            (width, height) if width > 0 && height > 0 => Ok(Tabletop {
                width,
                height,
                obstacles: vec![],
            }),

            _ => Err("Tabletop dimensions need to be positive integers.".to_string()),
        }
    }

    pub fn add_obstacle(&mut self, x: i32, y: i32) -> Result<(), String> {
        match (x, y) {
            (x, y) if x >= 0 && y >= 0 && x < self.width && y < self.height => {
                self.obstacles.push(Position { x, y });
                Ok(())
            }

            _ => Err(format!("Position ({}, {}) is out of bounds.", x, y)),
        }
    }

    /// Checks if an item can be placed on the Tabletop at the position provided.
    pub fn request_place(&self, position: &Position) -> Result<(), String> {
        // :: Check if the requested position is outside of the bounds of the table.
        if !(position.x >= 0
            && position.y >= 0
            && position.x < self.width
            && position.y < self.height)
        {
            Err(format!(
                "Position ({}, {}) is out of bounds.",
                position.x, position.y
            ))
        }
        // :: Check if the requested position is on top of a known obstacle.
        else if self.obstacles.contains(position) {
            Err(format!(
                "Position ({}, {}) is blocked.",
                position.x, position.y,
            ))
        } else {
            Ok(())
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

    #[test]
    fn tabletop_can_add_obstacles_correctly() {
        let mut tabletop = Tabletop::new(5, 5).unwrap();

        // :: should be ok
        assert!(tabletop.add_obstacle(1, 1).is_ok());
        assert!(tabletop.add_obstacle(4, 4).is_ok());

        assert_eq!(tabletop.obstacles.len(), 2);

        // :: should fail
        assert!(tabletop.add_obstacle(-1, 3).is_err());
        assert!(tabletop.add_obstacle(1, -1).is_err());
        assert!(tabletop.add_obstacle(-1, -3).is_err());

        assert!(tabletop.add_obstacle(6, 1).is_err());
        assert!(tabletop.add_obstacle(1, 6).is_err());
        assert!(tabletop.add_obstacle(6, 6).is_err());

        assert_eq!(tabletop.obstacles.len(), 2);
    }

    #[test]
    fn tabletop_does_not_allow_movement_to_obstacles() {
        let mut tabletop = Tabletop::new(5, 5).unwrap();

        assert!(tabletop.add_obstacle(1, 1).is_ok());
        assert!(tabletop.add_obstacle(4, 4).is_ok());

        // :: should be ok
        assert!(tabletop.request_place(&Position { x: 0, y: 0 }).is_ok());
        assert!(tabletop.request_place(&Position { x: 3, y: 3 }).is_ok());

        // :: should fail
        assert!(tabletop.request_place(&Position { x: 1, y: 1 }).is_err());
        assert!(tabletop.request_place(&Position { x: 4, y: 4 }).is_err());
    }
}
