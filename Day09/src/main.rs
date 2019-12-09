use intcode_computer::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = std::env::args().skip(1).next().unwrap();
    let input = std::fs::read_to_string(file)?;
    let program = input.parse::<Program>()?;
    let mut interpreter = Interpreter::new(program);

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

    // Instruction: input
    interpreter.add_instruction(Instruction::new(3, 1, |mut params| {
        let mut input_line = String::new();
        std::io::stdin()
            .read_line(&mut input_line)
            .expect("Could not read stdin");
        let input = input_line
            .trim()
            .parse::<isize>()
            .expect("Invalid input. Expected integer.");
        params[0].write(input)?;
        Ok(InstructionResult::Continue)
    })?);

    // Interpreter: output
    interpreter.add_instruction(Instruction::new(4, 1, |params| {
        println!("{}", params[0].read()?);
        Ok(InstructionResult::Continue)
    })?);

    // Interpreter: quit
    interpreter.add_instruction(Instruction::new(99, 0, |_| Ok(InstructionResult::Quit))?);

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

    // Instruction: adjust the relative base
    interpreter.add_instruction(Instruction::new(9, 1, |params| {
        let new_offset = params[0].read()?;
        Ok(InstructionResult::UpdateRelativeOffset(new_offset))
    })?);

    interpreter.execute()?;
    //dbg!(interpreter);
    Ok(())
}
