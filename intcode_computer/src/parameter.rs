use crate::program::{OutOfBoundError, Program};
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

/// Struct containing a single parameter for an instruction
#[derive(Debug)]
pub struct Parameter {
    mode: ParameterMode,
    value: isize,
    program: Rc<RefCell<Program>>,
    relative_offset: isize,
}

#[derive(Debug)]
pub enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

#[derive(Debug)]
pub enum ParameterError {
    OutOfBound(OutOfBoundError),
    IncompatibleMode,
    InvalidMode(u8),
}

impl std::error::Error for ParameterError {}

impl fmt::Display for ParameterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParameterError::OutOfBound(e) => write!(f, "{}", e),
            ParameterError::IncompatibleMode => write!(f, "Immediate mode used to write value"),
            ParameterError::InvalidMode(m) => write!(f, "Invalid mode: {}", m),
        }
    }
}

impl From<OutOfBoundError> for ParameterError {
    fn from(e: OutOfBoundError) -> ParameterError {
        ParameterError::OutOfBound(e)
    }
}

impl Parameter {
    pub fn new(
        mode: u8,
        value: isize,
        program: Rc<RefCell<Program>>,
        relative_offset: isize,
    ) -> Result<Parameter, ParameterError> {
        let mode = match mode {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            2 => ParameterMode::Relative,
            _ => return Err(ParameterError::InvalidMode(mode)),
        };
        Ok(Parameter {
            mode,
            value,
            program,
            relative_offset,
        })
    }
    pub fn read(&self) -> Result<isize, ParameterError> {
        match self.mode {
            ParameterMode::Immediate => Ok(self.value),
            ParameterMode::Position => Ok(self.program.borrow_mut().read(self.value as usize)?),
            ParameterMode::Relative => {
                let address = (self.value + self.relative_offset) as usize;
                Ok(self.program.borrow_mut().read(address)?)
            },
        }
    }

    pub fn write(&mut self, value: isize) -> Result<(), ParameterError> {
        match self.mode {
            ParameterMode::Immediate => Err(ParameterError::IncompatibleMode),
            ParameterMode::Position => {
                self.program
                    .borrow_mut()
                    .write(self.value as usize, value)?;
                Ok(())
            },
            ParameterMode::Relative => {
                let address = (self.value + self.relative_offset) as usize;
                self.program
                    .borrow_mut()
                    .write(address, value)?;
                Ok(())
            }
        }
    }
}
