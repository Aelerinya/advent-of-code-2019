#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub struct Position {
    pub x: isize,
    pub y: isize,
}

pub fn manhattan_distance(p1: &Position, p2: &Position) -> usize {
    (p1.x - p2.x).abs() as usize + (p1.y - p2.y).abs() as usize
}

#[cfg(tests)]
mod test {
    #[test]
    fn distance() {
        let a = Position {x: 1, y: -5};
        let b = Position {x: 6, y: 2};
        assert_eq!(manhattan_distance(a, b), 12);
    }
}
