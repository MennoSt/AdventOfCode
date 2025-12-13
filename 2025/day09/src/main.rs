use lib::filereader;
use lib::utils;
// use lib::utils::Coordinate;
use std::time::Instant;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Hash)]
struct Coordinate {
    pub x: i128,
    pub y: i128,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Hash)]
struct Area {
    coordinate1: Coordinate,
    coordinate2: Coordinate,
    area: i128,
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

fn area(a: &Coordinate, b: &Coordinate) -> i128 {
    ((a.x - b.x).abs() + 1) * ((a.y - b.y).abs() + 1)
}

fn p1(input: &str) -> i128 {
    let coordinates = parse_data(input);
    coordinates
        .iter()
        .flat_map(|a| coordinates.iter().map(move |b| area(a, b)))
        .max()
        .unwrap_or(0)
}

fn p2(input: &str) -> i128 {
    let coordinates = parse_data(input);
    let mut areas: Vec<Area> = coordinates
        .iter()
        .flat_map(|a| {
            coordinates.iter().map(move |b| {
                let ar = area(a, b);
                Area{coordinate1:a.clone(),coordinate2:b.clone(),area:ar}
            })
        })
        .collect();
    
    areas.sort_by(|a, b| b.area.cmp(&a.area));
    println!("{:?}", areas);

    0
}

fn isAreaValid(area:&Area, coordinates: &Vec<Coordinate>)
{
    
}

fn main() {
    let start = Instant::now();

    parse_data(INPUT);
    let part1 = p1(INPUT);
    println!("{}", part1);
    assert_eq!(part1, 4781377701);
    // let part2 = p2(INPUT);

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
    #[test]
    fn test2() {
        let p2 = p2(INPUT_EXAMPLE);
        // assert_eq!(p2, 50);
    }
}
