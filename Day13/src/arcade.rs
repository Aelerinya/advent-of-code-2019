use intcode_computer::{Interpreter, Program};
use std::cell::{Ref, RefCell};
use std::rc::Rc;
use std::cmp::Ordering;

type Pos = (isize, isize);

#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

type TileMap = Vec<Vec<TileType>>;

impl From<isize> for TileType {
    fn from(n: isize) -> TileType {
        match n {
            0 => TileType::Empty,
            1 => TileType::Wall,
            2 => TileType::Block,
            3 => TileType::Paddle,
            4 => TileType::Ball,
            _ => TileType::Empty,
        }
    }
}

impl Into<char> for TileType {
    fn into(self) -> char {
        match self {
            TileType::Empty => ' ',
            TileType::Wall => '#',
            TileType::Block => 'X',
            TileType::Paddle => '-',
            TileType::Ball => 'o',
        }
    }
}

#[derive(Clone, Copy)]
enum JoystickPosition {
    Left = -1,
    Neutral = 0,
    Right = 1,
}

struct ArcadeState {
    tiles: TileMap,
    score: isize,
    joystick: JoystickPosition,
    ball_pos: Pos,
    paddle_pos: Pos
}

pub struct Arcade {
    interpreter: Interpreter,
    state: Rc<RefCell< ArcadeState>>,
}

impl Arcade {
    pub fn new(program: Program) -> Arcade {
        let state = Rc::new(RefCell::new(ArcadeState {
            tiles: Vec::new(),
            score: 0,
            joystick: JoystickPosition::Neutral,
            ball_pos: (0, 0),
            paddle_pos: (0, 0)
        }));
        Arcade {
            interpreter: Interpreter::complete(
                program,
                // Game input: joystick
                {
                    let state = state.clone();
                    move || {
                        let state = state.borrow();
                        let joystick = match state.paddle_pos.0.cmp(&state.ball_pos.0) {
                            Ordering::Less => JoystickPosition::Right,
                            Ordering::Greater => JoystickPosition::Left,
                            Ordering::Equal => JoystickPosition::Neutral
                        };
                        joystick as isize
                    }
                },
                // Game output: print on the tiles map
                {
                    let mut input_state = Box::new(0);
                    let mut pos: Box<Pos> = Box::new((0, 0));
                    let state = state.clone();
                    move |output| {
                        let mut state = state.borrow_mut();
                        match *input_state {
                            0 => pos.0 = output,
                            1 => pos.1 = output,
                            2 => {
                                if *pos == (-1, 0) {
                                    state.score = output;
                                } else {
                                    let len_y = state.tiles.len();
                                    let pos_y = pos.1 as usize;
                                    if pos_y >= len_y {
                                        state.tiles.resize(pos_y + 1, Vec::new());
                                    }
                                    let len_x = state.tiles[pos_y].len();
                                    let pos_x = pos.0 as usize;
                                    if pos_x >= len_x {
                                        state.tiles[pos_y]
                                            .resize(pos_x + 1, TileType::Empty);
                                    }
                                    let tile_type = TileType::from(output);
                                    state.tiles[pos_y][pos_x] = tile_type;
                                    if tile_type == TileType::Ball {
                                        state.ball_pos = *pos;
                                    }
                                    if tile_type == TileType::Paddle {
                                        state.paddle_pos = *pos;
                                    }
                                }
                            }
                            _ => panic!("Invalid arcarde output state: {}", *input_state),
                        }
                        *input_state = (*input_state + 1) % 3;
                    }
                },
            ),
            state
        }
    }

    pub fn execute(&mut self) {
        self.interpreter.execute().expect("Arcade intcode error")
    }

    pub fn tiles(&self) -> Ref<TileMap> {
        let r = self.state.borrow();
        Ref::map(r, |s| &s.tiles)
    }

    pub fn score(&self) -> isize {
        self.state.borrow().score
    }

    pub fn display_map(&self) {
        for line in &self.state.borrow().tiles {
            for tile in line {
                let c: char = (*tile).into();
                print!("{}", c);
            }
            println!();
        }
    }
}
