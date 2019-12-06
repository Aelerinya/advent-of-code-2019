mod program;
mod instruction;
mod parameter;
mod interpreter;

pub use self::program::{Program, OutOfBoundError, ParseProgramError};
pub use self::instruction::{Instruction, InvalidInstruction, InstructionResult};
pub use self::parameter::{Parameter, ParameterError};
pub use self::interpreter::{Interpreter, InterpreterError};
