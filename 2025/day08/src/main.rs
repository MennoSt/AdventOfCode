use lib::filereader;
use lib::utils;
use lib::utils::Coordinate;
use ordered_float::OrderedFloat;
use std::time::Instant;
use std::collections::HashSet;
use std::hash::Hash;

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

fn merge_overlapping<T: Eq + Hash + Clone>(mut groups: Vec<Vec<T>>) -> Vec<Vec<T>> {
    loop {
        let mut merged_any = false;
        let mut result: Vec<Vec<T>> = Vec::new();

        while let Some(mut g1) = groups.pop() {
            let mut i = 0;
            while i < groups.len() {
                let g2 = &groups[i];

                // Check overlap: any element in g1 is also in g2
                if g1.iter().any(|e| g2.contains(e)) {
                    // Merge g2 into g1
                    g1.extend(g2.iter().cloned());
                    groups.remove(i);
                    merged_any = true;
                } else {
                    i += 1;
                }
            }
            result.push(g1);
        }

        groups = result;

        // No more merging possible -> done
        if !merged_any {
            break;
        }
    }

    // Dedup + cleanup
    for g in groups.iter_mut() {
        let set: HashSet<_> = g.drain(..).collect();
        g.extend(set.into_iter());
    }

    groups
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

fn three_largest_lengths<T>(vectors: &[Vec<T>]) -> Vec<usize> {
    let mut lengths: Vec<usize> = vectors.iter().map(|v| v.len()).collect();
    lengths.sort_unstable_by(|a, b| b.cmp(a)); // sort descending
    lengths.into_iter().take(3).collect()
}

fn p1(input: &str, connections: usize) -> i128 {
    let coordinates = parse_data(input);
    let chains = calculate_chains(coordinates, connections);

    let lengths = three_largest_lengths(&chains);

    let answer = lengths[0] * lengths[1] * lengths[2];
    answer as i128
}

fn calculate_chains(
    coordinates: Vec<Coordinate3D>,
    n_connections: usize,
) -> Vec<Vec<Coordinate3D>> {
    let mut jdistances = calculate_jdistances(coordinates.clone());
    jdistances.sort_by_key(|item| OrderedFloat(item.distance));

    let mut chains: Vec<Vec<Coordinate3D>> = Vec::new();

    for d in jdistances.iter().take(n_connections) {
        let c1 = d.pair.0;
        let c2 = d.pair.1;

        let mut found_chain = false;
        for chain in chains.iter_mut() {
            if chain.contains(&c1) && chain.contains(&c2) {
                found_chain = true;
                println!("NOT added already present {:?} and {:?}", c1, c2);
                break;
            } else if chain.contains(&c1) || chain.contains(&c2) {
                chain.push(c1);
                chain.push(c2);
                println!("added {:?} and {:?}", c1, c2);
                chain.sort_by_key(|p| p.x);
                chain.dedup_by(|a, b| a.x == b.x);
                found_chain = true;
                break;
            }
        }

        if found_chain == false {
            let mut chain: Vec<Coordinate3D> = Vec::new();
            chain.push(c1);
            chain.push(c2);
            chain.sort_by_key(|p| p.x);
            chain.dedup_by(|a, b| a.x == b.x);
            chains.push(chain);
            println!("added new chain {:?} and {:?}", c1, c2);
        }
        
        chains = merge_overlapping(chains.clone());

    }

    /// add lonely chains:
    for coord in coordinates.clone() {
        let mut exists = false;
        for chain in &chains {
            if chain.contains(&coord) {
                exists = true;
            }
        }
        if !exists {
            chains.push(vec![coord]);
        }
    }

    // for chain in &mut chains {
    //     chain.sort_by_key(|p| p.x);
    // }

    // for chain in &mut chains {
    //     chain.dedup_by(|a, b| a.x == b.x);

    //     println!("lenght: {} coordinate {:?}", chain.len(), chain);
    // }
    chains
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

    println!("length distances {:?}", jdistances.len());
    jdistances
}

fn main() {
    let start = Instant::now();

    let part1 = p1(INPUT, 1000);
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
        let size = p1(INPUT_EXAMPLE, 10);
        assert_eq!(size, 40);
    }

    #[test]
    fn test2() {
        let jdistances = vec![
            Coordinate3D { x: 1, y: 1, z: 1 },
            Coordinate3D { x: 2, y: 2, z: 2 },
            Coordinate3D { x: 4, y: 4, z: 4 },
        ];
        let chains = calculate_chains(jdistances, 1);

        let answer = vec![
            vec![
                Coordinate3D { x: 1, y: 1, z: 1 },
                Coordinate3D { x: 2, y: 2, z: 2 },
            ],
            vec![Coordinate3D { x: 4, y: 4, z: 4 }],
        ];
        assert_eq!(chains, answer);
    }

    #[test]
    fn test3() {
        let jdistances = vec![
            Coordinate3D { x: 1, y: 1, z: 1 },
            Coordinate3D { x: 2, y: 2, z: 2 },
            Coordinate3D { x: 4, y: 4, z: 4 },
            Coordinate3D { x: 8, y: 8, z: 8 },
        ];
        let chains = calculate_chains(jdistances, 2);

        let answer = vec![
            vec![
                Coordinate3D { x: 1, y: 1, z: 1 },
                Coordinate3D { x: 2, y: 2, z: 2 },
                Coordinate3D { x: 4, y: 4, z: 4 },
            ],
            vec![Coordinate3D { x: 8, y: 8, z: 8 }],
        ];

        assert_eq!(chains, answer);
    }
}
