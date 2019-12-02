use std::io::Read;

mod interpreter;
mod program;

use interpreter::Interpreter;
use program::Program;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut program = String::new();
    std::io::stdin().read_to_string(&mut program)?;
    let program = program.parse::<Program>()?;
    let mut noun = 0;
    let mut verb = 0;
    loop {
        let mut interpreter = Interpreter::new(&program, noun, verb);
        let result = interpreter.run();
        if let Ok(result) = result {
            if result == 19690720 {
                break;
            }
        }
        verb += 1;
        if verb > 1000 {
            verb = 0;
            noun += 1;
        }
    }
    println!("The noun is {} and the verb is {}", noun, verb);
    Ok(())
}
