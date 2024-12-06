#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

pub fn is_odd(n: i32) -> bool {
    n % 2 != 0
}