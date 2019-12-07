use intcode_computer::*;

mod amplifier;
mod phase;

use amplifier::Amplifier;
use phase::PhaseGenerator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = std::env::args().skip(1).next().unwrap();
    let input = std::fs::read_to_string(file)?;
    let program = input.parse::<Program>()?;

    let mut max = 0;
    let mut max_phase = None;
    for phase in PhaseGenerator::new(true) {
        let mut input = 0;
        let mut amplifiers = Vec::new();
        let mut output = None;
        for i in 0..5 {
            let phase = phase[i] as isize;
            amplifiers.push(Amplifier::new(program.clone(), phase)?)
        }
        loop {
            for i in 0..5 {
                let mut amplifier = &mut amplifiers[i];
                amplifier.execute();
                amplifier.input(input);
                input = match amplifier.execute() {
                    Some(o) => o,
                    None => {
                        output = Some(input);
                        break;
                    }
                };
            }
            if output != None {
                break;
            }
        }
        let output = output.unwrap();
        if output > max {
            max = output;
            max_phase = Some(phase);
        }
    }

    println!("max phase is {:?} with power {}", max_phase, max);
    //dbg!(interpreter);
    Ok(())
}
