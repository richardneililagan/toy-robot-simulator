use super::tabletop::*;

// :: ---

#[derive(Debug, PartialEq)]
pub enum EngineCommand {
    Place { x: u32, y: u32 },
    Move,
    Left,
    Right,
    Report,
}

impl EngineCommand {
    pub fn parse(plaintext: &str) -> Option<Self> {
        let words = plaintext.split_whitespace().collect::<Vec<&str>>();

        match words[0] {
            "PLACE" => Option::Some(EngineCommand::Place { x: 0, y: 0 }),
            "MOVE" => Option::Some(EngineCommand::Move),
            "LEFT" => Option::Some(EngineCommand::Left),
            "RIGHT" => Option::Some(EngineCommand::Right),
            "REPORT" => Option::Some(EngineCommand::Report),
            _ => Option::None,
        }
    }

    pub fn execute(&self, tabletop: &Tabletop) -> Option<&str> {
        Option::None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn engine_command_parses_move_correctly() {
        // :: standard expected usage
        assert_eq!(EngineCommand::parse("MOVE").unwrap(), EngineCommand::Move);
    }
}
