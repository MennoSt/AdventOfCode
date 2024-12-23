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

pub fn is_odd(n: i128) -> bool {
    n % 2 != 0
}

pub fn answer<T:std::fmt::Debug + std::cmp::PartialEq>(part1: (T,T), part2:(T,T)) {
    let answer = [part1, part2];
    for part in answer {
        println!("{:?}",part.0);
        assert_eq!(part.0, part.1);
    }
}
