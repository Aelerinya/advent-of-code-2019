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
    relative_offset: isize,
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
            relative_offset: 0,
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
            //dbg!(&instruction);
            let mut access_modes = number_to_digits(access_modes);
            access_modes.reverse();
            let access_modes = access_modes.iter().cloned().chain(std::iter::repeat(0));
            let parameters_number = instruction.arguments_number();
            let relative_offset = self.relative_offset;
            let parameters = (0..parameters_number)
                .zip(access_modes)
                .map(|(param, mode)| {
                    Parameter::new(
                        mode,
                        program
                            .borrow()
                            .read(instruction_pointer + param as usize + 1)?,
                        program.clone(),
                        relative_offset,
                    )
                })
                .collect::<Result<Vec<_>, _>>()?;
            match instruction.call(parameters)? {
                InstructionResult::Quit => return Ok(InterpreterState::End),
                InstructionResult::Continue => {
                    self.instruction_pointer += 1 + instruction.arguments_number() as usize;
                }
                InstructionResult::JumpTo(pos) => self.instruction_pointer = pos,
                InstructionResult::UpdateRelativeOffset(off) => {
                    self.instruction_pointer += 1 + instruction.arguments_number() as usize;
                    self.relative_offset += off;
                }
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
        while self.execute_one()? != InterpreterState::End {
            // dbg!(&self);
            // let mut line = String::new();
            // std::io::stdin().read_line(&mut line).unwrap();
        }
        Ok(())
    }

    pub fn complete<F, G>(program: Program, mut input_fn: F, mut output_fn: G) -> Interpreter
    where
        F: FnMut() -> isize + 'static,
        G: FnMut(isize) + 'static,
    {
        let mut interpreter = Interpreter::new(program);

        // Instruction: add
        interpreter.add_instruction(Instruction::new(1, 3, |mut params| {
            let sum = params[0].read()? + params[1].read()?;
            params[2].write(sum)?;
            Ok(InstructionResult::Continue)
        }).unwrap());

        // Instruction: mulitply
        interpreter.add_instruction(Instruction::new(2, 3, |mut params| {
            let product = params[0].read()? * params[1].read()?;
            params[2].write(product)?;
            Ok(InstructionResult::Continue)
        }).unwrap());

        // Instruction: input
        interpreter.add_instruction(Instruction::new(3, 1, move |mut params| {
            let input = input_fn();
            params[0].write(input)?;
            Ok(InstructionResult::Continue)
        }).unwrap());

        // Interpreter: output
        interpreter.add_instruction(Instruction::new(4, 1, move |params| {
            let output = params[0].read()?;
            output_fn(output);
            Ok(InstructionResult::Continue)
        }).unwrap());

        // Interpreter: quit
        interpreter.add_instruction(Instruction::new(99, 0, |_| Ok(InstructionResult::Quit)).unwrap() );

        // Interpreter: jump-if-true
        interpreter.add_instruction(Instruction::new(5, 2, |params| {
            if params[0].read()? != 0 {
                Ok(InstructionResult::JumpTo(params[1].read()? as usize))
            } else {
                Ok(InstructionResult::Continue)
            }
        }).unwrap());
        // Interpreter: jump-if-false
        interpreter.add_instruction(Instruction::new(6, 2, |params| {
            if params[0].read()? == 0 {
                Ok(InstructionResult::JumpTo(params[1].read()? as usize))
            } else {
                Ok(InstructionResult::Continue)
            }
        }).unwrap());

        // Instruction: less than
        interpreter.add_instruction(Instruction::new(7, 3, |mut params| {
            let result = if params[0].read()? < params[1].read()? {
                1
            } else {
                0
            };
            params[2].write(result)?;
            Ok(InstructionResult::Continue)
        }).unwrap());

        // Instruction: equals
        interpreter.add_instruction(Instruction::new(8, 3, |mut params| {
            let result = if params[0].read()? == params[1].read()? {
                1
            } else {
                0
            };
            params[2].write(result)?;
            Ok(InstructionResult::Continue)
        }).unwrap());

        // Instruction: adjust the relative base
        interpreter.add_instruction(Instruction::new(9, 1, |params| {
            let new_offset = params[0].read()?;
            Ok(InstructionResult::UpdateRelativeOffset(new_offset))
        }).unwrap());

        interpreter
    }
}
