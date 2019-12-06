use crate::instruction::{Instruction, InstructionResult};
use crate::parameter::{Parameter, ParameterError};
use crate::program::Program;
use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fmt;
use std::rc::Rc;

#[derive(Debug)]
pub struct Interpreter {
    program: Rc<RefCell<Program>>,
    instruction_pointer: usize,
    instructions: HashMap<u8, Instruction>,
}

#[derive(Debug)]
pub enum InterpreterError {
    InvalidOpcode(isize),
    UnknownOpcode(u8),
    InvalidParameter(ParameterError),
    UnexpectedEndOfFile,
}

impl std::error::Error for InterpreterError {}

impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use InterpreterError::*;
        match self {
            InvalidOpcode(op) => write!(f, "Invalid opcode found in program: {}", op),
            UnknownOpcode(op) => write!(f, "Now instruction provided for opcode {}", op),
            InvalidParameter(e) => write!(f, "{}", e),
            UnexpectedEndOfFile => write!(
                f,
                "Interpreter reached end-of-file without encountering a stop instruction (99)"
            ),
        }
    }
}

impl From<ParameterError> for InterpreterError {
    fn from(e: ParameterError) -> InterpreterError {
        InterpreterError::InvalidParameter(e)
    }
}

fn number_to_digits(n: usize) -> Vec<u8> {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect()
}

#[derive(PartialEq)]
enum InterpreterState {
    End,
    Continue,
}

impl Interpreter {
    pub fn new(program: Program) -> Interpreter {
        Interpreter {
            program: Rc::new(RefCell::new(program)),
            instruction_pointer: 0,
            instructions: HashMap::new(),
        }
    }

    pub fn add_instruction(&mut self, instruction: Instruction) {
        self.instructions.insert(instruction.opcode(), instruction);
    }

    fn execute_one(&mut self) -> Result<InterpreterState, InterpreterError> {
        let v = self
            .program
            .borrow()
            .read(self.instruction_pointer)
            .unwrap();
        let v: usize = match v.try_into() {
            Ok(v) => v,
            Err(_) => return Err(InterpreterError::InvalidOpcode(v)),
        };
        let opcode = (v % 100) as u8;

        let access_modes = v / 100;
        let program = self.program.clone();
        let instruction_pointer = self.instruction_pointer;
        if let Some(instruction) = self.instructions.get_mut(&opcode) {
            let mut access_modes = number_to_digits(access_modes);
            access_modes.reverse();
            let access_modes = access_modes.iter().cloned().chain(std::iter::repeat(0));
            let parameters_number = instruction.arguments_number();
            let parameters = (0..parameters_number)
                .zip(access_modes)
                .map(|(param, mode)| {
                    Parameter::new(
                        mode,
                        program
                            .borrow()
                            .read(instruction_pointer + param as usize + 1)?,
                        program.clone(),
                    )
                })
                .collect::<Result<Vec<_>, _>>()?;
            match instruction.call(parameters)? {
                InstructionResult::Quit => return Ok(InterpreterState::End),
                InstructionResult::Continue => {
                    self.instruction_pointer += 1 + instruction.arguments_number() as usize;
                },
                InstructionResult::JumpTo(pos) => self.instruction_pointer = pos
            };
            if self.instruction_pointer >= self.program.borrow().len() {
                Err(InterpreterError::UnexpectedEndOfFile)
            } else {
                Ok(InterpreterState::Continue)
            }
        } else {
            Err(InterpreterError::UnknownOpcode(opcode as u8))
        }
    }

    pub fn execute(&mut self) -> Result<(), InterpreterError> {
        while self.execute_one()? != InterpreterState::End {}
        Ok(())
    }
}
