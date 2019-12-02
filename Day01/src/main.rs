use std::io::BufRead;

fn fuel_required(mass: usize) -> usize {
    if mass < 6 {
        0
    } else {
        mass / 3 - 2
    }
}

fn compensate_fuel_mass(mut fuel_added: usize) -> usize {
    let mut total_fuel = 0;
    loop {
        let new_fuel = fuel_required(fuel_added);
        if new_fuel == 0 {
            break;
        }
        total_fuel += new_fuel;
        fuel_added = new_fuel;
    }
    return total_fuel;
}

fn main() {
    let mut total_fuel = 0;
    for line in std::io::stdin().lock().lines() {
        let mass: usize = line.unwrap().parse().unwrap();
        let fuel = fuel_required(mass);
        let fuel_for_fuel = compensate_fuel_mass(fuel);
        total_fuel += fuel + fuel_for_fuel;
    }
    println!("Total fuel needed for modules: {}", total_fuel);
}
