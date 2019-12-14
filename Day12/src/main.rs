mod gravity_system;
mod position;

use gravity_system::GravitySystem;
use position::Position;
use std::io::BufRead;

fn main() {
    let pos = std::io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().parse::<Position>().unwrap())
        .collect::<Vec<_>>();
    let mut system = GravitySystem::new(pos);
    // for s in 0..=100 {
    //     if s != 0 {
    //         system.step();
    //     }
    //     println!("After {} steps:", s);
    //     println!("{}", system);
    // }
    for _ in 0..1000 {
        system.step();
    }
    let energy = system.total_energy();
    println!("The total energy of the system is {}", energy);
}
