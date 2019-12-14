use std::io::BufRead;

mod reaction;
mod reaction_system;

use reaction::Reaction;
use reaction_system::ReactionSystem;

fn main() {
    // lol
    let reactions = std::io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().parse::<Reaction>())
        .collect::<Result<Vec<_>, _>>()
        .expect("Invalid reactions");
    let system = ReactionSystem::new(reactions);
    let ore_needed = system.get_minimum_ore_for_one_fuel().unwrap();
    println!("Minimum ore needed to create one fuel: {}", ore_needed);
    let fuel = system
        .get_maximum_fuel_synthetizable(1_000_000_000_000_usize)
        .unwrap();
    println!("Maximum fuel synthetizable: {}", fuel);
}
