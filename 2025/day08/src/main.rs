use lib::filereader;
use lib::utils;
use lib::utils::Coordinate;
use ordered_float::OrderedFloat;
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
    pair: (Coordinate3D, Coordinate3D),
    distance: f64,
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

fn p1(input: &str) -> i128 {
    let coordinates = parse_data(input);
    let mut jdistances = calculate_jdistances(coordinates);
    jdistances.sort_by_key(|item| OrderedFloat(item.distance));

    let mut chains: Vec<Vec<Coordinate3D>> = Vec::new();

    for d in jdistances.iter().take(1000) {
        let c1 = d.pair.0;
        let c2 = d.pair.1;

        let mut found_chain = false;
        for chain in chains.iter_mut() {
            if chain.contains(&c1) && chain.contains(&c2) {
                found_chain = true;
                break;
            } else if chain.contains(&c1) || chain.contains(&c2) {
                chain.push(c1);
                chain.push(c2);
                chain.sort_by_key(|p| p.x);
                chain.dedup_by(|a, b| a.x == b.x);
                found_chain = true;
                break;
            }
        }

        if chains.is_empty() || found_chain == false {
            let mut chain: Vec<Coordinate3D> = Vec::new();
            chain.push(c1);
            chain.push(c2);
            chains.push(chain)
        }
    }

    for chain in &mut chains {
        chain.sort_by_key(|p| p.x);
    }

    for chain in &mut chains {
        chain.dedup_by(|a, b| a.x == b.x);

        println!("lenght: {} coordinate {:?}", chain.len(), chain);
    }

    let product: usize = chains
        .iter()
        .map(|chain| chain.len())
        .fold(1, |acc, len| acc * len);

    // println!("{:?}", chains);

    product as i128
}

fn calculate_jdistances(coordinates: Vec<Coordinate3D>) -> Vec<JDistance> {
    let n = coordinates.len();

    let pairs: Vec<_> = (0..n)
        .flat_map(|i| {
            let coords = &coordinates;
            (i + 1..n).map(move |j| (&coords[i], &coords[j]))
        })
        .collect();

    let mut jdistances: Vec<JDistance> = Vec::new();
    for pair in pairs {
        let distance = calculate_distance(pair.0, pair.1);
        jdistances.push(JDistance {
            pair: (pair.0.clone(), pair.1.clone()),
            distance: distance,
        });
    }
    jdistances
}

fn main() {
    let start = Instant::now();

    let part1 = p1(INPUT);
    println!("{}", part1);
    // assert_eq!(part1, 1541);
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
