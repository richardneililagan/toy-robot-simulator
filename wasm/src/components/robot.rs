use wasm_bindgen::prelude::*;

use super::common::*;
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
    tabletop: Option<Tabletop>,
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
            tabletop: Some(*tabletop),
            position: None,
            orientation: None,
        })
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
    fn robot_tabletop_reference_is_equivalent_to_source() {
        let tabletop = Tabletop::new(5, 5).unwrap();
        let robot = Robot::create(&tabletop).unwrap();

        assert_eq!(tabletop, robot.tabletop.unwrap());
    }
}
