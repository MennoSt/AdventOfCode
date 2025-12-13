use lib::filereader;
use lib::utils;
// use lib::utils::Coordinate;
use std::time::Instant;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Hash)]
pub struct Coordinate {
    pub x: i128,
    pub y: i128,
}

static INPUT: &str = "../input/day09";

fn parse_data(input: &str) -> Vec<Coordinate> {
    filereader::_input(input)
        .lines()
        .map(|line| {
            let mut it = line.split(',');
            Coordinate {
                x: it.next().unwrap().parse().unwrap(),
                y: it.next().unwrap().parse().unwrap(),
            }
        })
        .collect()
}

fn p1(input: &str) -> i128 {
    let coordinates = parse_data(input);


    let mut max_area = 0;
    for i in 0..coordinates.len() {
        for j in 0..coordinates.len()
        {
            let area = ((coordinates[i].x - coordinates[j].x).abs()+1) * ((coordinates[i].y - coordinates[j].y).abs() +1);
            if area > max_area
            {
                max_area = area;
            }
        }
    }

    max_area
}

fn p2(input: &str) -> i128 {
    0
}

fn main() {
    let start = Instant::now();

    parse_data(INPUT);
    let part1 = p1(INPUT);
    println!("{}",part1);
    let part2 = p2(INPUT);

    // utils::answer((part1, 0), (part2, 0));

    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT_EXAMPLE: &str = "example";

    #[test]
    fn test1() {
        let p1 = p1(INPUT_EXAMPLE);
        assert_eq!(p1, 50);
    }
}
