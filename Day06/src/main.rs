mod orbits_map;

use std::io::Read;
use orbits_map::{OrbitsMap, OrbitObject};

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let map = input.parse::<OrbitsMap>().unwrap();
    let orbits_count = map.orbit_count_checksum();
    println!("Total number of orbits: {}", orbits_count);
    let you = OrbitObject::Object("YOU".to_string());
    let santa = OrbitObject::Object("SAN".to_string());
    let distance = map.minimal_orbital_transfers(&you, &santa).unwrap();
    println!("Minimum number of orbital transfers: {}", distance);
}
