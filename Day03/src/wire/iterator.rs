use super::{Movement, Direction};
use crate::position::Position;
use std::iter::Iterator;

pub struct WireIterator<'a> {
    pos: Position,
    movement_index: usize,
    distance_index: usize,
    path: &'a [Movement],
}

impl WireIterator<'_> {
    pub fn new<'a>(path: &'a [Movement]) -> WireIterator<'a> {
        WireIterator {
            pos: Position { x: 0, y: 0 },
            movement_index: 0,
            distance_index: 0,
            path,
        }
    }
}

impl Iterator for WireIterator<'_> {
    type Item = Position;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(movement) = self.path.get(self.movement_index) {
            match &movement.direction {
                Direction::Up => self.pos = Position {
                    x: self.pos.x,
                    y: self.pos.y + 1
                },
                Direction::Down => self.pos = Position {
                    x: self.pos.x,
                    y: self.pos.y - 1
                },
                Direction::Left => self.pos = Position {
                    x: self.pos.x - 1,
                    y: self.pos.y
                },
                Direction::Right => self.pos = Position {
                    x: self.pos.x + 1,
                    y: self.pos.y
                },
            }
            self.distance_index += 1;
            if self.distance_index >= movement.distance {
                self.distance_index = 0;
                self.movement_index += 1;
            }
            Some(self.pos)
        } else {
            None
        }
    }
}
