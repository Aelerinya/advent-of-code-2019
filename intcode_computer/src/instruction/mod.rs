use crate::parameter::{Parameter, ParameterError};
use std::fmt;

/// Trait describing an instruction
pub struct Instruction {
    opcode: u8,
    arguments_number: u8,
    call_function: Box<dyn FnMut(Vec<Parameter>) -> Result<InstructionResult, ParameterError>>,
}

pub enum InstructionResult {
    Continue,
    JumpTo(usize),
    Quit,
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Instruction")
            .field("opcode", &self.opcode)
            .field("arguments_number", &self.arguments_number)
            .finish()
    }
}

#[derive(Debug)]
pub enum InvalidInstruction {
    InvalidOpcode(u8),
}

impl std::error::Error for InvalidInstruction {}

impl fmt::Display for InvalidInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InvalidInstruction::InvalidOpcode(op) => write!(f, "Invalid opcode: {}", op),
        }
    }
}

impl Instruction {
    pub fn new<F>(opcode: u8, arguments_number: u8, f: F) -> Result<Instruction, InvalidInstruction>
    where
        F: 'static + FnMut(Vec<Parameter>) -> Result<InstructionResult, ParameterError>,
    {
        if opcode <= 99 {
            Ok(Instruction {
                opcode,
                arguments_number,
                call_function: Box::new(f),
            })
        } else {
            Err(InvalidInstruction::InvalidOpcode(opcode))
        }
    }

    pub fn opcode(&self) -> u8 {
        self.opcode
    }

    pub fn arguments_number(&self) -> u8 {
        self.arguments_number
    }

    pub fn call(&mut self, params: Vec<Parameter>) -> Result<InstructionResult, ParameterError> {
        (self.call_function)(params)
    }
}
