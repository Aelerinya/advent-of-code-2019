use std::fmt;
use std::str::FromStr;

mod iterator;

use iterator::WireIterator;

#[derive(Debug)]
pub struct Wire {
    path: Vec<Movement>,
}

#[derive(Debug)]
pub enum ParseWireError {
    InvalidMovement(String),
}

impl std::error::Error for ParseWireError {}

impl fmt::Display for ParseWireError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseWireError::InvalidMovement(s) => write!(f, "Invalid Movement: {}", s),
        }
    }
}

impl From<MovementError> for ParseWireError {
    fn from(e: MovementError) -> ParseWireError {
        match e {
            MovementError::InvalidMovement(s) => ParseWireError::InvalidMovement(s),
        }
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub struct Movement {
    direction: Direction,
    distance: usize,
}

#[derive(Debug)]
pub enum MovementError {
    InvalidMovement(String),
}

impl std::error::Error for MovementError {}

impl fmt::Display for MovementError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MovementError::InvalidMovement(s) => write!(f, "Invalid Movement: {}", s),
        }
    }
}

impl FromStr for Wire {
    type Err = ParseWireError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Wire {
            path: s
                .split(",")
                .map(|movement| movement.trim().parse::<Movement>())
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl FromStr for Movement {
    type Err = MovementError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let direction = match chars.next() {
            Some(d) => d,
            None => return Err(MovementError::InvalidMovement(s.to_owned())),
        };
        let distance = match chars.collect::<String>().parse::<usize>() {
            Ok(v) => v,
            Err(_) => return Err(MovementError::InvalidMovement(s.to_owned())),
        };
        match direction {
            'U' => Ok(Movement {
                direction: Direction::Up,
                distance,
            }),
            'D' => Ok(Movement {
                direction: Direction::Down,
                distance,
            }),
            'L' => Ok(Movement {
                direction: Direction::Left,
                distance,
            }),
            'R' => Ok(Movement {
                direction: Direction::Right,
                distance,
            }),
            _ => Err(MovementError::InvalidMovement(s.to_owned())),
        }
    }
}

impl Wire {
    pub fn positions<'a>(&'a self) -> WireIterator<'a> {
        WireIterator::new(&self.path)
    }
}
