use wasm_bindgen::prelude::*;

use super::common::*;
use super::instruction::Instruction;
use super::tabletop::Tabletop;

// :: ---

/// A Robot is a representation of a robot that can be placed on top of a Tabletop.
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

    /// Attempts to translate a provided plaintext command, and executes the respective
    /// operation/s if a known `Instruction` could be discerned.
    pub fn evaluate_command(&mut self, command: &str) -> Result<JsValue, String> {
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

            Instruction::Move => self.move_forward(),
            Instruction::Left => self.turn_left(),
            Instruction::Right => self.turn_right(),

            _ => unreachable!(),
        }
    }

    /// Places a Robot instance on top of a Tabletop at the provided position,
    /// facing the provided orientation.
    ///
    /// # Returns
    ///
    /// This returns `Result::Ok(())` if the operation was valid and enacted,
    /// and `Result::Err(message)` otherwise, with `message` containing why the
    /// operation failed.
    fn place_on_tabletop(
        &mut self,
        position: Position,
        orientation: Orientation,
    ) -> Result<JsValue, String> {
        match self.tabletop.request_place(&position) {
            Ok(_) => {
                self.position = Some(position);
                self.orientation = Some(orientation);

                Ok(JsValue::NULL)
            }

            Err(message) => Err(format!(
                "Robot cannot be placed at that position: {}",
                message
            )),
        }
    }

    /// Moves the robot forward 1 unit, in the direction it is currently oriented in.
    ///
    /// This function takes advantage of the fact that a movement in the context of
    /// this problem is the same as (re-)placing the robot in the arrival position,
    /// except that it should not be possible to do so if the robot has not been
    /// yet placed prior.
    fn move_forward(&mut self) -> Result<JsValue, String> {
        if !self.is_placed() {
            return Err("Robot is not placed; discarding instruction".to_string());
        }

        // :: ---

        let mut target_position = self.position.unwrap();
        match self.orientation.as_ref().unwrap() {
            Orientation::North => target_position.y += 1,
            Orientation::South => target_position.y -= 1,
            Orientation::East => target_position.x += 1,
            Orientation::West => target_position.x -= 1,
        }

        let can_move = self.tabletop.request_place(&target_position);
        match can_move {
            Ok(_) => {
                self.position = Some(target_position);
                Ok(JsValue::NULL)
            }

            Err(message) => Err(format!("Robot cannot be moved: {}", message)),
        }
    }

    /// Re-orients the Robot by turning it to the left.
    fn turn_left(&mut self) -> Result<JsValue, String> {
        if !self.is_placed() {
            return Err("Robot is not placed; discarding instruction.".to_string());
        }

        // :: ---

        let new_orientation = match self.orientation.as_ref().unwrap() {
            Orientation::North => Orientation::West,
            Orientation::West => Orientation::South,
            Orientation::South => Orientation::East,
            Orientation::East => Orientation::North,
        };

        self.orientation = Some(new_orientation);

        Ok(JsValue::NULL)
    }

    /// Re-orients the Robot by turning it to the right.
    fn turn_right(&mut self) -> Result<JsValue, String> {
        if !self.is_placed() {
            return Err("Robot is not placed; discarding instruction.".to_string());
        }

        // :: ---

        let new_orientation = match self.orientation.as_ref().unwrap() {
            Orientation::North => Orientation::East,
            Orientation::East => Orientation::South,
            Orientation::South => Orientation::West,
            Orientation::West => Orientation::North,
        };

        self.orientation = Some(new_orientation);

        Ok(JsValue::NULL)
    }

    /// Has this Robot successfully been placed on a Tabletop?
    ///
    /// Perhaps a bit naively, Robot considers itself placed if it has been
    /// given a non-None value for position and orientation. It is expected
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
    fn robot_is_correctly_oriented_on_tabletop_when_placed() {
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

        assert_eq!(robot.position.as_ref().unwrap(), &Position { x: 3, y: 3 });
        assert_eq!(robot.orientation.as_ref().unwrap(), &Orientation::North);

        assert!(robot
            .place_on_tabletop(Position { x: 0, y: 2 }, Orientation::South)
            .is_ok());
        assert!(robot.is_placed());

        assert_eq!(robot.position.as_ref().unwrap(), &Position { x: 0, y: 2 });
        assert_eq!(robot.orientation.as_ref().unwrap(), &Orientation::South);
    }

    #[test]
    fn robot_turns_left_correctly() {
        let tabletop = Tabletop::new(5, 5).unwrap();
        let mut robot = Robot::create(&tabletop).unwrap();

        assert!(robot
            .place_on_tabletop(Position { x: 3, y: 3 }, Orientation::North)
            .is_ok());

        assert_eq!(robot.orientation.as_ref().unwrap(), &Orientation::North);

        assert!(robot.turn_left().is_ok());
        assert_eq!(robot.orientation.as_ref().unwrap(), &Orientation::West);

        assert!(robot.turn_left().is_ok());
        assert_eq!(robot.orientation.as_ref().unwrap(), &Orientation::South);

        assert!(robot.turn_left().is_ok());
        assert_eq!(robot.orientation.as_ref().unwrap(), &Orientation::East);

        assert!(robot.turn_left().is_ok());
        assert_eq!(robot.orientation.as_ref().unwrap(), &Orientation::North);
    }

    #[test]
    fn robot_turns_right_correctly() {
        let tabletop = Tabletop::new(5, 5).unwrap();
        let mut robot = Robot::create(&tabletop).unwrap();

        assert!(robot
            .place_on_tabletop(Position { x: 3, y: 3 }, Orientation::North)
            .is_ok());

        assert_eq!(robot.orientation.as_ref().unwrap(), &Orientation::North);

        assert!(robot.turn_right().is_ok());
        assert_eq!(robot.orientation.as_ref().unwrap(), &Orientation::East);

        assert!(robot.turn_right().is_ok());
        assert_eq!(robot.orientation.as_ref().unwrap(), &Orientation::South);

        assert!(robot.turn_right().is_ok());
        assert_eq!(robot.orientation.as_ref().unwrap(), &Orientation::West);

        assert!(robot.turn_right().is_ok());
        assert_eq!(robot.orientation.as_ref().unwrap(), &Orientation::North);
    }

    #[test]
    fn robot_cannot_turn_if_not_yet_placed() {
        let tabletop = Tabletop::new(5, 5).unwrap();
        let mut robot = Robot::create(&tabletop).unwrap();

        assert!(!robot.is_placed());
        assert!(robot.turn_left().is_err());
        assert!(robot.turn_right().is_err());
    }

    #[test]
    fn robot_cannot_be_moved_if_it_has_not_been_placed_yet() {
        let tabletop = Tabletop::new(5, 5).unwrap();
        let mut robot = Robot::create(&tabletop).unwrap();

        assert!(!robot.is_placed());
        assert!(robot.move_forward().is_err());
    }

    #[test]
    fn robot_can_be_moved_forward_in_the_right_direction() {
        let tabletop = Tabletop::new(5, 5).unwrap();
        let mut robot = Robot::create(&tabletop).unwrap();

        assert!(robot
            .place_on_tabletop(Position { x: 3, y: 3 }, Orientation::North)
            .is_ok());

        assert!(robot.move_forward().is_ok());
        assert_eq!(robot.position.unwrap(), Position { x: 3, y: 4 });

        assert!(robot.move_forward().is_err()); // :: Reached the edge of the tabletop.
        assert_eq!(robot.position.unwrap(), Position { x: 3, y: 4 });

        assert!(robot.turn_right().is_ok());
        assert!(robot.move_forward().is_ok());
        assert_eq!(robot.position.unwrap(), Position { x: 4, y: 4 });

        assert!(robot.move_forward().is_err()); // :: Reached the edge of the tabletop.
        assert_eq!(robot.position.unwrap(), Position { x: 4, y: 4 });

        assert!(robot.turn_right().is_ok());
        for y in (0..=3).rev() {
            assert!(robot.move_forward().is_ok());
            assert_eq!(robot.position.unwrap(), Position { x: 4, y });
        }

        assert!(robot.move_forward().is_err()); // :: Reached the edge of the tabletop.
        assert_eq!(robot.position.unwrap(), Position { x: 4, y: 0 });

        assert!(robot.turn_right().is_ok());
        for x in (0..=3).rev() {
            assert!(robot.move_forward().is_ok());
            assert_eq!(robot.position.unwrap(), Position { x, y: 0 });
        }

        assert!(robot.move_forward().is_err()); // :: Reached the edge of the tabletop.
        assert_eq!(robot.position.unwrap(), Position { x: 0, y: 0 });
    }
}
