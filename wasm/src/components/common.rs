#[derive(Debug, PartialEq)]
pub enum Orientation {
    North,
    East,
    West,
    South,
}

impl Orientation {
    pub fn parse(plaintext: &str) -> Result<Self, String> {
        match plaintext.to_uppercase().as_str() {
            "NORTH" => Ok(Orientation::North),
            "EAST" => Ok(Orientation::East),
            "WEST" => Ok(Orientation::West),
            "SOUTH" => Ok(Orientation::South),

            _ => Err(format!("Unexpected orientation: {}", plaintext)),
        }
    }
}

struct Position {
    x: i32,
    y: i32,
}
