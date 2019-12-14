use std::error::Error;
use std::fmt;
use std::io::Read;
use std::num::ParseIntError;
use std::ops::{Index, IndexMut};
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct Program {
    pub memory: Vec<isize>,
}

#[derive(Debug)]
pub enum ParseProgramError {
    InvalidValue(ParseIntError),
}

impl Error for ParseProgramError {}

impl fmt::Display for ParseProgramError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseProgramError::InvalidValue(v) => write!(f, "Invalid value in program data: {}", v),
        }
    }
}

impl FromStr for Program {
    type Err = ParseProgramError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let memory = match s
            .split(",")
            .map(|s| s.trim().parse::<isize>())
            .collect::<Result<Vec<_>, _>>()
        {
            Ok(v) => v,
            Err(e) => return Err(ParseProgramError::InvalidValue(e)),
        };
        Ok(Program { memory })
    }
}

impl Index<usize> for Program {
    type Output = isize;

    fn index(&self, index: usize) -> &Self::Output {
        return &self.memory[index];
    }
}

impl IndexMut<usize> for Program {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return &mut self.memory[index];
    }
}

#[derive(Debug)]
pub struct OutOfBoundError(usize);

impl Error for OutOfBoundError {}

impl fmt::Display for OutOfBoundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid access at address {}", self.0)
    }
}

impl Program {
    pub fn read(&self, pos: usize) -> Result<isize, OutOfBoundError> {
        Ok(*self.memory.get(pos).unwrap_or(&0))
    }

    pub fn write(&mut self, pos: usize, value: isize) -> Result<(), OutOfBoundError> {
        if pos >= self.memory.len() {
            self.memory.resize(pos + 1, 0);
        }
        match self.memory.get_mut(pos) {
            Some(v) => {
                *v = value;
                Ok(())
            }
            None => Err(OutOfBoundError(pos)),
        }
    }

    pub fn len(&self) -> usize {
        self.memory.len()
    }

    pub fn from_stdin() -> Result<Program, Box<dyn std::error::Error>> {
        let mut s = String::new();
        std::io::stdin().read_to_string(&mut s)?;
        Ok(s.parse::<Program>()?)
    }
}
