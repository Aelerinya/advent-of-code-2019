mod map;

use map::Map;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let map = input.parse::<Map>().unwrap();
    let (pos, visibility) = map.get_asteroid_with_most_visibility().unwrap();
    println!(
        "Asteroid ({},{}) has the most visibility, and can see {} other asteroids.",
        pos.0, pos.1, visibility
    );
    let order = map.get_complete_vaporization_by_giant_laser_order(pos);
    let (x, y) = order[199];
    println!("Asteroid ({},{}) will be vaporized in 200th position !", x, y);
    println!("X*100+Y = {}", x * 100 + y);
}
