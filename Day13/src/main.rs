use intcode_computer::Program;

mod arcade;

use arcade::{Arcade, TileType};

fn main() {
    let mut program = Program::from_stdin().unwrap();
    program.write(0, 2).unwrap();
    let mut arcade = Arcade::new(program);
    arcade.execute();
    let block_count = arcade
        .tiles()
        .iter()
        .map(|l| l.iter())
        .flatten()
        .filter(|t| **t == TileType::Block)
        .count();
    println!(
        "Total number of blocks at the end of the game: {}",
        block_count
    );
    println!("Final score: {}", arcade.score());
    arcade.display_map();
}
