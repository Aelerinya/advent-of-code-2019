use snafu::Snafu;
use std::collections::{HashMap, HashSet};
use std::iter::Iterator;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
enum MapCell {
    Asteroid,
    Empty,
}

impl From<char> for MapCell {
    fn from(c: char) -> MapCell {
        match c {
            '#' => MapCell::Asteroid,
            '.' => MapCell::Empty,
            _ => panic!("Invalid char {}", c),
        }
    }
}

#[derive(Debug)]
pub struct Map {
    content: Vec<Vec<MapCell>>,
}

#[derive(Snafu, Debug)]
pub enum ParseMapError {
    #[snafu(display("Invalid character in map: {}", c))]
    InvalidCharacter { c: char },
}

impl FromStr for Map {
    type Err = ParseMapError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().all(|c| ".#\n".contains(c)) {
            let content = s
                .lines()
                .map(|line| line.chars().map(|c| MapCell::from(c)).collect::<Vec<_>>())
                .collect::<Vec<_>>();
            Ok(Map { content })
        } else {
            let c = s.chars().find(|c| !(".#\n".contains(*c))).unwrap();
            Err(ParseMapError::InvalidCharacter { c })
        }
    }
}

type Pos = (usize, usize);

pub struct Asteroids<'a> {
    map: &'a Map,
    pos: Pos,
}

impl Iterator for Asteroids<'_> {
    type Item = Pos;

    fn next(&mut self) -> Option<Self::Item> {
        while *self.map.content.get(self.pos.1)?.get(self.pos.0)? != MapCell::Asteroid {
            self.pos.0 += 1;
            if self.pos.0 >= self.map.content.get(self.pos.1)?.len() {
                self.pos.1 += 1;
                self.pos.0 = 0;
                if self.pos.1 >= self.map.content.len() {
                    return None;
                }
            }
        }
        let result = Some(self.pos);
        self.pos.0 += 1;
        if self.pos.0 >= self.map.content.get(self.pos.1)?.len() {
            self.pos.1 += 1;
            self.pos.0 = 0;
        }
        result
    }
}

impl Map {
    pub fn asteroids<'a>(&'a self) -> Asteroids<'a> {
        Asteroids {
            map: self,
            pos: (0, 0),
        }
    }

    fn count_visible_asteroids(&self, (pos_x, pos_y): Pos) -> usize {
        let mut angles_already_seen = HashSet::new();
        let mut asteroids_visible = 0;
        for (x, y) in self.asteroids() {
            if x == pos_x && y == pos_y {
                continue;
            }
            let dx = pos_x as f32 - x as f32;
            let dy = pos_y as f32 - y as f32;
            let angle = (dy.atan2(dx) * 1024.0 * 1024.0).round() as i64; //Ratio::new(dy, dx);
            if !angles_already_seen.contains(&angle) {
                angles_already_seen.insert(angle);
                asteroids_visible += 1;
            }
        }
        asteroids_visible
    }

    pub fn get_asteroid_with_most_visibility(&self) -> Option<(Pos, usize)> {
        self.asteroids()
            .map(|a| (a, self.count_visible_asteroids(a)))
            .max_by_key(|a| a.1)
    }

    pub fn get_complete_vaporization_by_giant_laser_order(&self, laser_pos: Pos) -> Vec<Pos> {
        let asteroids = self.asteroids().filter(|p| *p != laser_pos).map(|(x, y)| {
            let dx = x as f32 - laser_pos.0 as f32;
            let dy = y as f32 - laser_pos.1 as f32;
            let mut angle = dy.atan2(dx);
            if dx < 0. && dy < 0. {
                angle += 2.0 * std::f32::consts::PI;
            }
            let angle = (angle * 1024.0 * 1024.0).round() as i64;
            let distance = (dx.hypot(dy) * 1024.0 * 1024.0).round()  as i64;
            ((x, y), angle, distance)
        });
        let mut order: HashMap<i64, Vec<_>> = HashMap::new();
        for (pos, angle, distance) in asteroids {
            let entry = order.entry(angle).or_insert(Vec::new());
            entry.push((pos, distance));
        }
        for v in order.values_mut() {
            v.sort_unstable_by_key(|(_, distance)| distance.clone());
        }
        let mut order = order.drain().collect::<Vec<_>>();
        order.sort_unstable_by_key(|v| v.0);
        let mut asteroids = Vec::new();
        while !order.is_empty() {
            for v in &mut order {
                let (pos, _) = v.1.remove(0);
                asteroids.push(pos);
            }
            order.retain(|(_, v)| v.len() > 0);
        }
        asteroids
    }
}
