use std::ops::Deref;
use std::iter::Iterator;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Phase([u8; 5]);

impl Phase {
    fn new(digits: [u8; 5]) -> Option<Phase> {
        let mut presence = HashSet::new();
        for digit in digits.iter() {
            if presence.contains(digit) {
                return None
            } else {
                presence.insert(digit);
            }
        }
        Some(Phase(digits))
    }
}

impl Deref for Phase {
    type Target = [u8; 5];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct PhaseGenerator {
    digits: [u8; 5],
    ended: bool,
    high: bool,
}

impl PhaseGenerator {
    pub fn new(high: bool) -> PhaseGenerator {
        PhaseGenerator {
            digits: if high {[9,8,7,6,5]} else {[4, 3, 2, 1, 0]},
            ended: false,
            high,
        }
    }

    fn increase(&mut self) {
        if self.ended {
            return;
        }
        if self.high {
            if self.digits == [9; 5] {
                self.ended = true;
                return;
            }
            for i in 0..=4 {
                self.digits[i] += 1;
                if self.digits[i] > 9 {
                    self.digits[i] = 5;
                } else {
                    break;
                }
            }
        } else {
            if self.digits == [4; 5] {
                self.ended = true;
                return;
            }
            for i in 0..=4 {
                self.digits[i] += 1;
                if self.digits[i] > 4 {
                    self.digits[i] = 0;
                } else {
                    break;
                }
            }
        }
    }
}

impl Iterator for PhaseGenerator {
    type Item = Phase;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ended {
            return None
        }
        loop {
            //dbg!(self.digits);
            if let Some(phase) = Phase::new(self.digits) {
                self.increase();
                return Some(phase);
            }
            self.increase();
            if self.ended {
                return None
            }
        }
    }
}
