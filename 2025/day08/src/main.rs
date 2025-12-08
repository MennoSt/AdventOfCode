use lib::filereader;
use lib::utils;
use lib::utils::Coordinate;
use std::time::Instant;

static INPUT: &str = "../input/day08";

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Hash, Copy)]
struct Coordinate3D {
    x: i128,
    y: i128,
    z: i128,
}

#[derive(Debug, Clone, Copy)]
struct JDistance {
    pair : (Coordinate3D, Coordinate3D),
    distance : f64,
}

fn parse_data(input: &str) -> Vec<Coordinate3D> {
    let content = filereader::_input(input);
    let mut coordinates: Vec<Coordinate3D> = Vec::new();
    for line in content.lines() {
        let mut parts = line.split(',').map(|p| p.parse::<i128>().unwrap());
        let x: i128 = parts.next().unwrap();
        let y: i128 = parts.next().unwrap();
        let z: i128 = parts.next().unwrap();
        coordinates.push(Coordinate3D { x: x, y: y, z: z });
    }
    coordinates
}

fn calculate_distance(coordinate1: &Coordinate3D, coordinate2: &Coordinate3D) -> f64 {
    let dx = coordinate2.x as f64 - coordinate1.x as f64;
    let dy = coordinate2.y as f64 - coordinate1.y as f64;
    let dz = coordinate2.z as f64 - coordinate1.z as f64;

    (dx.powi(2) + dy.powi(2) + dz.powi(2)).sqrt()
}

fn p1(input: &str) -> i32 {
    let coordinates = parse_data(input);

    let n = coordinates.len();

    let pairs: Vec<_> = (0..n)
        .flat_map(|i| {
            let coords = &coordinates;
            (i + 1..n).map(move |j| (&coords[i], &coords[j]))
        })
        .collect();

    println!("{:?}", pairs);
    println!("size: {}", pairs.len());

    let mut jdistances: Vec<JDistance> = Vec::new();
    for pair in pairs {
        let distance = calculate_distance(pair.0, pair.1);
        jdistances.push(JDistance{pair:(pair.0.clone(),pair.1.clone()), distance:distance});
    }

    println!("{:?}", jdistances);

    0
}

fn main() {
    let start = Instant::now();

    let part1 = p1(INPUT);
    println!("{}", part1);
    assert_eq!(part1, 1541);
    // let part2 = p2(INPUT);
    // println!("{}", part2);

    // utils::answer((part1,1363),(part2, 8184));

    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT_EXAMPLE: &str = "example";

    #[test]
    fn test1() {
        let size = p1(INPUT_EXAMPLE);
        assert_eq!(size, 40);
    }
}
