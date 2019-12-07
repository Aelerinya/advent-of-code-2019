use intcode_computer::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Amplifier {
        interpreter: Interpreter,
        input: Rc<RefCell<Option<isize>>>,
        output: Rc<RefCell<Option<isize>>>,
        skip_output: Rc<RefCell<bool>>
}

impl Amplifier {
        pub fn new(program: Program, phase: isize) -> Result<Amplifier, Box<dyn std::error::Error>> {

                let mut amplifier = Amplifier {
                        interpreter: Interpreter::new(program),
                        input: Rc::new(RefCell::new(Some(phase))),
                        output: Rc::new(RefCell::new(None)),
                        skip_output: Rc::new(RefCell::new(false)),
                };
                let interpreter = &mut amplifier.interpreter;
                // Instruction: add
                interpreter.add_instruction(Instruction::new(1, 3, |mut params| {
                        let sum = params[0].read()? + params[1].read()?;
                        params[2].write(sum)?;
                        Ok(InstructionResult::Continue)
                })?);

                // Instruction: mulitply
                interpreter.add_instruction(Instruction::new(2, 3, |mut params| {
                        let product = params[0].read()? * params[1].read()?;
                        params[2].write(product)?;
                        Ok(InstructionResult::Continue)
                })?);

                // Interpreter: jump-if-true
                interpreter.add_instruction(Instruction::new(5, 2, |params| {
                        if params[0].read()? != 0 {
                                Ok(InstructionResult::JumpTo(params[1].read()? as usize))
                        } else {
                                Ok(InstructionResult::Continue)
                        }
                })?);
                // Interpreter: jump-if-false
                interpreter.add_instruction(Instruction::new(6, 2, |params| {
                        if params[0].read()? == 0 {
                                Ok(InstructionResult::JumpTo(params[1].read()? as usize))
                        } else {
                                Ok(InstructionResult::Continue)
                        }
                })?);

                // Instruction: less than
                interpreter.add_instruction(Instruction::new(7, 3, |mut params| {
                        let result = if params[0].read()? < params[1].read()? {
                                1
                        } else {
                                0
                        };
                        params[2].write(result)?;
                        Ok(InstructionResult::Continue)
                })?);

                // Instruction: equals
                interpreter.add_instruction(Instruction::new(8, 3, |mut params| {
                        let result = if params[0].read()? == params[1].read()? {
                                1
                        } else {
                                0
                        };
                        params[2].write(result)?;
                        Ok(InstructionResult::Continue)
                })?);

                // Interpreter: quit
                interpreter
                        .add_instruction(Instruction::new(99, 0, |_| Ok(InstructionResult::Quit))?);

                let output_copy = amplifier.output.clone();
                let skip_output_copy = amplifier.skip_output.clone();

                // Interpreter: output
                interpreter.add_instruction(Instruction::new(4, 1, move |params| {
                        let mut skip = skip_output_copy.borrow_mut();
                        if !*skip {
                                *output_copy.borrow_mut() = Some(params[0].read()?);
                                *skip = true;
                                Ok(InstructionResult::Quit)
                        } else {
                                *skip = false;
                                Ok(InstructionResult::Continue)
                        }
                })?);

                let input_copy = amplifier.input.clone();
                // Instruction: input
                interpreter.add_instruction(Instruction::new(3, 1, move |mut params| {
                        let mut input = input_copy.borrow_mut();
                        if let Some(input) = input.take() {
                                params[0].write(input)?;
                                Ok(InstructionResult::Continue)
                        } else {
                                Ok(InstructionResult::Quit)
                        }
                })?);

                Ok(amplifier)
        }

        pub fn execute(&mut self) -> Option<isize> {
                self.interpreter.execute().unwrap();

                let result = self.output.borrow_mut().take();
                result
        }

        pub fn input(&mut self, input: isize) {
                *self.input.borrow_mut() = Some(input);
        }
}
