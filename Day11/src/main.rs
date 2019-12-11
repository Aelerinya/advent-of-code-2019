use intcode_computer::{Interpreter, Program};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

type Pos = (isize, isize);

fn main() {
    let file = std::env::args().skip(1).next().expect("No input file");
    let input = std::fs::read_to_string(file).expect("Could not read input file");
    let program = input.parse::<Program>().expect("Invalid program");
    let pos = Rc::new(RefCell::new((0, 0)));
    let map: Rc<RefCell<HashMap<Pos, bool>>> = Rc::new(RefCell::new(HashMap::new()));
    map.borrow_mut().insert((0, 0), true);
    let mut interpreter = Interpreter::complete(
        program,
        {
            let pos = pos.clone();
            let map = map.clone();
            move || match map.borrow().get(&pos.borrow()) {
                Some(color) => {
                    if *color {
                        1
                    } else {
                        0
                    }
                }
                None => 0,
            }
        },
        {
            let pos = pos.clone();
            let map = map.clone();
            let direction = Rc::new(RefCell::new(0_i8));
            let first = Rc::new(RefCell::new(true));
            move |i| {
                let is_first = *first.borrow();
                if is_first {
                    let color = match i {
                        0 => false,
                        1 => true,
                        _ => panic!("Invalid color {}", i)
                    };
                    map.borrow_mut().insert(*pos.borrow(), color);
                    *first.borrow_mut() = false;
                } else {
                    match i {
                        0 => {
                            let new = (*direction.borrow() + 1) % 4;
                            *direction.borrow_mut() = new;
                        }
                        1 => {
                            let mut new = *direction.borrow() - 1;
                            if new < 0 {
                                new += 4
                            };
                            *direction.borrow_mut() = new;
                        },
                        _ => panic!("Invalid turn output {}", i)
                    }
                    let old_pos = *pos.borrow();
                    match *direction.borrow() {
                        0 => *pos.borrow_mut() = (old_pos.0, old_pos.1 - 1),

                        1 => *pos.borrow_mut() = (old_pos.0 - 1, old_pos.1),

                        2 => *pos.borrow_mut() = (old_pos.0, old_pos.1 + 1),

                        3 => *pos.borrow_mut() = (old_pos.0 + 1, old_pos.1),
                        i => panic!("Invalid direction {}", i)
                    }
                    *first.borrow_mut() = true;
                }
            }
        },
    );
    interpreter.execute().unwrap();
    let painted_panels_number = map.borrow().keys().count();
    println!("{} panels were painted at least once", painted_panels_number);
    let min_x = map.borrow().keys().min_by_key(|p1| p1.0).unwrap().0;
    let min_y = map.borrow().keys().min_by_key(|p1| p1.1).unwrap().1;
    let mut image: Vec<Vec<bool>> = Vec::new();
    for (pos, color) in map.borrow().iter() {
        let pos = ((pos.0 - min_x) as usize, (pos.1 - min_y) as usize);
        if pos.1 >= image.len() {
            image.resize(pos.1 + 1, Vec::new());
        }
        if pos.0 >= image[pos.1].len() {
            image[pos.1].resize(pos.0 + 1, false);
        }
        image[pos.1][pos.0] = *color;
    }

    for line in image {
        for c in line {
            if c {
                print!("\u{2588}");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    //dbg!(map);
}
