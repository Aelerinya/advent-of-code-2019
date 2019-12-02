use crate::program::{self, Program};
use std::error::Error;
use std::fmt;

pub struct Interpreter {
    program: Program,
    instruction_pointer: usize,
}

#[derive(Debug)]
pub enum InterpreterError {
    InvalidOpcode(isize),
    OutOfBound(program::OutOfBoundError),
}

impl Error for InterpreterError {}

impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InterpreterError::InvalidOpcode(op) => write!(f, "Invalid in the program: {}", op),
            InterpreterError::OutOfBound(e) => write!(f, "{}", e),
        }
    }
}

impl From<program::OutOfBoundError> for InterpreterError {
    fn from(e: program::OutOfBoundError) -> InterpreterError {
        InterpreterError::OutOfBound(e)
    }
}

impl Interpreter {
    pub fn new(program: &Program, noun: isize, verb: isize) -> Self {
        let mut program = program.clone();
        program[1] = noun;
        program[2] = verb;
        Self {
            program,
            instruction_pointer: 0,
        }
    }

    pub fn run(&mut self) -> Result<isize, InterpreterError> {
        while self.program[self.instruction_pointer] != 99 {
            match self.program[self.instruction_pointer] {
                1 => {
                    let input1 = self.program.read(self.instruction_pointer + 1)? as usize;
                    let input2 = self.program.read(self.instruction_pointer + 2)? as usize;
                    let output = self.program.read(self.instruction_pointer + 3)? as usize;
                    self.program.write(output, self.program.read(input1)? + self.program.read(input2)?)?;
                }
                2 => {
                    let input1 = self.program.read(self.instruction_pointer + 1)? as usize;
                    let input2 = self.program.read(self.instruction_pointer + 2)? as usize;
                    let output = self.program.read(self.instruction_pointer + 3)? as usize;
                    self.program.write(output, self.program.read(input1)? * self.program.read(input2)?)?;
                }
                _ => {
                    return Err(InterpreterError::InvalidOpcode(
                        self.program.read(self.instruction_pointer)?,
                    ))
                }
            }
            self.instruction_pointer += 4;
        }
        Ok(self.program[0])
    }
}
