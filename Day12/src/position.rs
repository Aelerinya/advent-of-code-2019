use lazy_static::lazy_static;
use regex::Regex;
use snafu::Snafu;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Position {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl Default for Position {
    fn default() -> Position {
        Position { x: 0, y: 0, z: 0 }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<x={}, y={}, z={}>", self.x, self.y, self.z)
    }
}

#[derive(Debug, Snafu)]
pub enum PositionParseError {
    #[snafu(display("Invalid position syntax: {}", s))]
    InvalidSyntax { s: String },
}

impl FromStr for Position {
    type Err = PositionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"<x=(?P<x>-?\d+), y=(?P<y>-?\d+), z=(?P<z>-?\d+)>").unwrap();
        }
        let cap = match RE.captures(s) {
            Some(v) => v,
            None => return Err(PositionParseError::InvalidSyntax { s: s.to_owned() }),
        };
        Ok(Position {
            x: cap.name("x").unwrap().as_str().parse::<isize>().unwrap(),
            y: cap.name("y").unwrap().as_str().parse::<isize>().unwrap(),
            z: cap.name("z").unwrap().as_str().parse::<isize>().unwrap(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn to_string() {
        let pos = Position {
            x: -7,
            y: 17,
            z: -11,
        };
        assert_eq!(pos.to_string(), "<x=-7, y=17, z=-11>")
    }

    #[test]
    fn from_string() {
        let pos = "<x=-7, y=17, z=-11>".parse::<Position>().unwrap();
        assert_eq!(
            pos,
            Position {
                x: -7,
                y: 17,
                z: -11,
            }
        );
        assert!("lol".parse::<Position>().is_err());
    }
}
