use wasm_bindgen::prelude::*;

use super::common::*;
use super::instruction::Instruction;
use super::tabletop::Tabletop;

// :: ---

#[wasm_bindgen]
pub struct Robot {
    //  We can't use a direct reference here yet (`Option<&Tabletop>`)
    //  because that will require us to specify a lifetime for Robot:
    //  ```
    //  struct Robot<'a> {
    //      tabletop: Option<&'a Tabletop>,
    //      // ...
    //  }
    //  ```
    //  but lifetime specifiers are not allowed / available yet in
    //  constructs meant to be exported to JS via `wasm_bindgen`.
    //
    //  It's not ideal, but we'll instead copy the Tabletop in memory
    //  on Robot creation so that Robot has a Tabletop copy that it directly owns.
    //  This works for this exercise because Tabletop is not meant to change
    //  in any way, and is fairly flat and static. To keep this operation fairly
    //  lightweight though, we'll want to keep Tabletop as thin a struct as possible.
    tabletop: Tabletop,
    position: Option<Position>,
    orientation: Option<Orientation>,
}

impl Robot {
    /// Creates a Robot and registers it (not place!) on a Tabletop.
    ///
    /// Registering a Robot to a Tabletop means that we intend to use this
    /// instance of Robot with this Tabletop. It will still need to be placed
    /// on the Tabletop before it can be given most instructions.
    pub fn create(tabletop: &Tabletop) -> Result<Robot, String> {
        Ok(Robot {
            tabletop: *tabletop,
            position: None,
            orientation: None,
        })
    }

    pub fn evaluate_command(&mut self, command: &str) -> Result<(), String> {
        let instruction_result = Instruction::parse(command);
        if let Err(message) = instruction_result {
            return Err(message);
        }

        // :: ---
        match instruction_result.unwrap() {
            Instruction::Place { x, y, orientation } => {
                let position = Position { x, y };
                self.place_on_tabletop(position, orientation)
            }

            _ => unreachable!(),
        }
    }

    fn place_on_tabletop(
        &mut self,
        position: Position,
        orientation: Orientation,
    ) -> Result<(), String> {
        let can_place = self.tabletop.request_place(&position);
        if !can_place {
            return Err("Robot cannot be placed at that position.".to_string());
        }

        // :: ---

        self.position = Some(position);
        self.orientation = Some(orientation);

        Ok(())
    }

    /// Has this Robot successfully been placed on a Tabletop?
    ///
    /// Perhaps a bit naively, Robot considers itself placed if it has been
    /// given a non-None value for their corresponding fields. It is expected
    /// that these fields are None on instantiation.
    fn is_placed(&self) -> bool {
        self.position.is_some() && self.orientation.is_some()
    }
}

// :: ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn robot_can_be_created() {
        let tabletop = Tabletop::new(5, 5).unwrap();
        assert!(Robot::create(&tabletop).is_ok());
    }

    #[test]
    fn robot_has_empty_position_and_orientation_on_creation() {
        let tabletop = Tabletop::new(5, 5).unwrap();
        let robot = Robot::create(&tabletop).unwrap();

        assert!(robot.position.is_none());
        assert!(robot.orientation.is_none());
    }

    #[test]
    fn robot_is_not_placed_on_creation() {
        let tabletop = Tabletop::new(5, 5).unwrap();
        let robot = Robot::create(&tabletop).unwrap();

        assert!(!robot.is_placed());
    }

    #[test]
    fn robot_tabletop_reference_is_equivalent_to_source() {
        let tabletop = Tabletop::new(5, 5).unwrap();
        let robot = Robot::create(&tabletop).unwrap();

        assert_eq!(tabletop, robot.tabletop);
    }

    #[test]
    fn robot_is_successfully_placed_on_tabletop() {
        let tabletop = Tabletop::new(5, 5).unwrap();
        let mut robot = Robot::create(&tabletop).unwrap();

        let result = robot.place_on_tabletop(Position { x: 3, y: 3 }, Orientation::North);
        assert!(result.is_ok());
        assert!(robot.is_placed());

        assert!(robot.position.is_some());
        assert_eq!(robot.position.unwrap(), Position { x: 3, y: 3 });

        assert!(robot.orientation.is_some());
        assert_eq!(robot.orientation.unwrap(), Orientation::North);
    }

    #[test]
    fn robot_is_successfully_oriented_on_tabletop_when_placed() {
        let tabletop = Tabletop::new(5, 5).unwrap();
        let mut robot = Robot::create(&tabletop).unwrap();

        assert!(robot
            .place_on_tabletop(Position { x: 3, y: 3 }, Orientation::North)
            .is_ok());
        assert_eq!(robot.orientation.as_ref().unwrap(), &Orientation::North);

        assert!(robot
            .place_on_tabletop(Position { x: 3, y: 3 }, Orientation::West)
            .is_ok());
        assert_eq!(robot.orientation.as_ref().unwrap(), &Orientation::West);

        assert!(robot
            .place_on_tabletop(Position { x: 3, y: 3 }, Orientation::South)
            .is_ok());
        assert_eq!(robot.orientation.as_ref().unwrap(), &Orientation::South);

        assert!(robot
            .place_on_tabletop(Position { x: 3, y: 3 }, Orientation::East)
            .is_ok());
        assert_eq!(robot.orientation.as_ref().unwrap(), &Orientation::East);
    }

    #[test]
    fn robot_can_be_replaced_and_reoriented() {
        let tabletop = Tabletop::new(5, 5).unwrap();
        let mut robot = Robot::create(&tabletop).unwrap();

        assert!(robot
            .place_on_tabletop(Position { x: 3, y: 3 }, Orientation::North)
            .is_ok());
        assert!(robot.is_placed());

        assert!(robot
            .place_on_tabletop(Position { x: 0, y: 2 }, Orientation::South)
            .is_ok());
        assert!(robot.is_placed());
        assert_eq!(robot.position.as_ref().unwrap(), &Position { x: 0, y: 2 });
        assert_eq!(robot.orientation.as_ref().unwrap(), &Orientation::South);
    }
}
