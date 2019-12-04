mod position;
mod wire;

use position::Position;
use std::cmp;
use std::collections::hash_map::HashMap;
use std::io::Read;
use wire::Wire;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let lines = input.lines().map(|s| s.to_owned()).collect::<Vec<String>>();
    let wire1 = lines[0].parse::<Wire>()?;
    let wire2 = lines[1].parse::<Wire>()?;
    let mut wire1_position: HashMap<Position, usize> = HashMap::new();
    for (index, pos) in wire1.positions().enumerate() {
        if !wire1_position.contains_key(&pos){
        wire1_position.insert(pos, index);}
    }
    let center = Position { x: 0, y: 0 };
    let mut min_distance = None;
    let mut min_delay = None;
    for (index, pos) in wire2.positions().enumerate() {
        if let Some(length_wire1) = wire1_position.get(&pos) {
            let distance = position::manhattan_distance(&pos, &center);
            min_distance = match min_distance {
                None => Some(distance),
                Some(previous_distance) => Some(cmp::min(previous_distance, distance)),
            };
            let delay = length_wire1 + index + 2;
            min_delay = match min_delay {
                None => Some(delay),
                Some(previous_delay) => Some(cmp::min(previous_delay, delay))
            }
        }
    }
    println!("The minimum distance is {}", min_distance.unwrap());
    println!("The minimum delay is {}", min_delay.unwrap());
    Ok(())
}
