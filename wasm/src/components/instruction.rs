use std::iter::Inspect;

use super::common::*;

// :: ---

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Place {
        x: i32,
        y: i32,
        orientation: Orientation,
    },

    Obstacle {
        x: i32,
        y: i32,
    },

    Left,
    Right,
    Move,
    Report,
}

impl Instruction {
    /// Translates a plaintext command to an `Instruction`.
    pub fn parse(plaintext: &str) -> Result<Self, String> {
        let normalized_text = plaintext.to_uppercase();
        let words = normalized_text.split_whitespace().collect::<Vec<&str>>();

        // :: This pattern validates any required arguments given the command,
        //    but just drops all other unexpected additional arguments.
        //    Will need to change approach if commands are invalidated if
        //    provided too many arguments.
        match words[0] {
            "PLACE" if words.len() >= 2 => {
                let args_fragment = words[1..].join("");
                let args = args_fragment
                    .split(',')
                    .map(|fragment| fragment.trim())
                    .collect::<Vec<&str>>();

                if args.len() < 3 {
                    return Err(format!(
                        "Arguments for command {} were incomplete.",
                        words[0]
                    ));
                }

                // :: ---

                let rx = args[0].parse::<i32>();
                let ry = args[1].parse::<i32>();
                let rorientation = Orientation::parse(args[2]);

                // :: If x, y, and orientation all parsed correctly
                if let (Ok(x), Ok(y), Ok(orientation)) = (rx, ry, rorientation) {
                    Ok(Instruction::Place { x, y, orientation })
                } else {
                    Err(format!("Arguments for command {} were invalid.", words[0]))
                }
            }

            "OBSTACLE" if words.len() >= 2 => {
                let args_fragment = words[1..].join("");
                let args = args_fragment
                    .split(',')
                    .map(|fragment| fragment.trim())
                    .collect::<Vec<&str>>();

                if args.len() < 2 {
                    return Err(format!(
                        "Arguments for command {} were incomplete.",
                        words[0]
                    ));
                }

                let rx = args[0].parse::<i32>();
                let ry = args[1].parse::<i32>();

                // :: If x and y are parsed correctly
                if let (Ok(x), Ok(y)) = (rx, ry) {
                    Ok(Instruction::Obstacle { x, y })
                } else {
                    Err(format!("Arguments for command {} were invalid.", words[0]))
                }
            }

            "MOVE" => Ok(Instruction::Move),
            "LEFT" => Ok(Instruction::Left),
            "RIGHT" => Ok(Instruction::Right),
            "REPORT" => Ok(Instruction::Report),

            _ => Err(format!(
                "Command {} was not recognized or is malformed.",
                words[0]
            )),
        }
    }
}

// :: ---

#[cfg(test)]
mod tests {
    use super::*;

    fn expect_conversion(plaintext: &str, instruction: Instruction) {
        let result = Instruction::parse(plaintext);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), instruction);
    }

    #[test]
    fn move_instruction_is_parsed_correctly() {
        expect_conversion("MOVE", Instruction::Move);
        expect_conversion("move", Instruction::Move);
        expect_conversion("move 1 2 3", Instruction::Move);
        expect_conversion("MOVE 1,3,5,7", Instruction::Move);
        expect_conversion("move right left", Instruction::Move);
    }

    #[test]
    fn left_instruction_is_parsed_correctly() {
        expect_conversion("LEFT", Instruction::Left);
        expect_conversion("left", Instruction::Left);
        expect_conversion("LEFT 1,3,5,1", Instruction::Left);
        expect_conversion("left 3 5 1", Instruction::Left);
        expect_conversion("left right report", Instruction::Left);
    }

    #[test]
    fn right_instruction_is_parsed_correctly() {
        expect_conversion("RIGHT", Instruction::Right);
        expect_conversion("right", Instruction::Right);
        expect_conversion("RIGHT 1,3,1", Instruction::Right);
        expect_conversion("right 10 2 23", Instruction::Right);
        expect_conversion("right left move", Instruction::Right);
    }

    #[test]
    fn report_instruction_is_parsed_correctly() {
        expect_conversion("REPORT", Instruction::Report);
        expect_conversion("report", Instruction::Report);
        expect_conversion("REPORT 1, 10, 2", Instruction::Report);
        expect_conversion("report 1 10 10", Instruction::Report);
        expect_conversion("report move place", Instruction::Report);
    }

    #[test]
    fn place_instruction_is_parsed_correctly() {
        expect_conversion(
            "PLACE 0,0,NORTH",
            Instruction::Place {
                x: 0,
                y: 0,
                orientation: Orientation::North,
            },
        );

        expect_conversion(
            "place 2,10,EAST",
            Instruction::Place {
                x: 2,
                y: 10,
                orientation: Orientation::East,
            },
        );
    }

    #[test]
    fn place_instruction_requires_at_least_three_args() {
        assert!(Instruction::parse("PLACE").is_err());
        assert!(Instruction::parse("PLACE 1").is_err());
        assert!(Instruction::parse("PLACE 1,2").is_err());

        assert!(Instruction::parse("PLACE 4,10,NORTH").is_ok());
        assert!(Instruction::parse("PLACE 3,15,SOUTH,2").is_ok());
    }

    #[test]
    fn place_instruction_requires_comma_delimiter() {
        assert!(Instruction::parse("PLACE 2 10 WEST").is_err());
        assert!(Instruction::parse("PLACE 2,10,WEST").is_ok());
    }

    #[test]
    fn obstacle_instruction_is_parsed_correctly() {
        expect_conversion("OBSTACLE 2,2", Instruction::Obstacle { x: 2, y: 2 });
        expect_conversion("obstacle 1,3", Instruction::Obstacle { x: 1, y: 3 });
    }

    #[test]
    fn obstacle_instruction_requires_at_least_two_args() {
        assert!(Instruction::parse("OBSTACLE").is_err());
        assert!(Instruction::parse("OBSTACLE 2").is_err());

        assert!(Instruction::parse("OBSTACLE 1,3").is_ok());
        assert!(Instruction::parse("OBSTACLE 1,3,3").is_ok());
        assert!(Instruction::parse("OBSTACLE 1,3,2,4").is_ok());
    }

    #[test]
    fn obstacle_instruction_requires_comma_delimiter() {
        assert!(Instruction::parse("OBSTACLE 2 3").is_err());
        assert!(Instruction::parse("OBSTACLE 2,3").is_ok());
    }

    #[test]
    fn whitespace_around_arguments_are_trimmed() {
        assert!(Instruction::parse("PLACE 5   , 10     , SOUTH ").is_ok());
        assert!(Instruction::parse("PLACE                10 , 20, NORTH").is_ok());
        assert!(Instruction::parse("         MOVE      ").is_ok());
        assert!(Instruction::parse("               REPORT").is_ok());
    }
}
