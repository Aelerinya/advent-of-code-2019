use crate::position::Position;
use std::cmp::Ordering;
use std::fmt;

#[derive(Clone, Copy)]
struct Object {
    position: Position,
    velocity: Position,
}

pub struct GravitySystem {
    objects: Vec<Object>,
}

fn update_velocity(mut o1: Object, mut o2: Object) -> (Object, Object) {
    let mut v1 = &mut o1.velocity;
    let mut v2 = &mut o2.velocity;
    let p1 = o1.position;
    let p2 = o2.position;

    match p1.x.cmp(&p2.x) {
        Ordering::Less => {
            v1.x += 1;
            v2.x -= 1;
        }
        Ordering::Greater => {
            v1.x -= 1;
            v2.x += 1;
        }
        Ordering::Equal => {}
    }
    match p1.y.cmp(&p2.y) {
        Ordering::Less => {
            v1.y += 1;
            v2.y -= 1;
        }
        Ordering::Greater => {
            v1.y -= 1;
            v2.y += 1;
        }
        Ordering::Equal => {}
    }
    match p1.z.cmp(&p2.z) {
        Ordering::Less => {
            v1.z += 1;
            v2.z -= 1;
        }
        Ordering::Greater => {
            v1.z -= 1;
            v2.z += 1;
        }
        Ordering::Equal => {}
    }
    (o1, o2)
}

impl Object {
    fn update_position(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
        self.position.z += self.velocity.z;
    }

    fn potential_energy(&self) -> usize {
        self.position.x.abs() as usize
            + self.position.y.abs() as usize
            + self.position.z.abs() as usize
    }

    fn cynetic_energy(&self) -> usize {
        self.velocity.x.abs() as usize
            + self.velocity.y.abs() as usize
            + self.velocity.z.abs() as usize
    }

    fn total_energy(&self) -> usize {
        self.cynetic_energy() * self.potential_energy()
    }
}

impl GravitySystem {
    pub fn new(positions: Vec<Position>) -> GravitySystem {
        GravitySystem {
            objects: positions
                .iter()
                .map(|p| Object {
                    position: *p,
                    velocity: Position::default(),
                })
                .collect::<Vec<_>>(),
        }
    }

    pub fn step(&mut self) {
        for o1 in 0..self.objects.len() {
            for o2 in (o1 + 1)..self.objects.len() {
                let (new_o1, new_o2) = update_velocity(self.objects[o1], self.objects[o2]);
                self.objects[o1] = new_o1;
                self.objects[o2] = new_o2;
            }
        }
        for object in &mut self.objects {
            object.update_position()
        }
    }

    pub fn total_energy(&self) -> usize {
        self.objects.iter().map(|o| o.total_energy()).sum()
    }
}

impl fmt::Display for GravitySystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for object in &self.objects {
            writeln!(f, "pos={}, vel={}", object.position, object.velocity)?
        }
        Ok(())
    }
}
