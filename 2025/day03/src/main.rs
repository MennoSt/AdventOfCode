use lib::{filereader, utils};
use std::time::Instant;

static INPUT: &str = "../input/day03";

fn parse_data(input: &str) -> Vec<String> {
    let contents = filereader::_input(input);
    let mut data: Vec<String> = Vec::new();

    for c in contents.lines() {
        data.push(c.to_string());
    }
    data
}

fn largest_joltage(
    input: &str,
    added_indices: &mut Vec<usize>,
    joltage: &mut String,
    length: usize,
) {
    for i in ('1'..='9').rev() {
        if joltage.len() == length {
            return;
        }

        let positions: Vec<usize> = input
            .char_indices() // iterate over (index, char)
            .filter(|(_, c)| *c == i) // keep only i
            .map(|(i, _)| i) // take the index
            .collect();

        if positions.len() > 0 {
            for p in positions {
                if !added_indices.contains(&p) && added_indices.iter().all(|&n| p > n) {
                    let last_index_and_size_zero = p == input.len() - 1 && joltage.len() == 0;
                    if !last_index_and_size_zero {
                        joltage.push(i);
                        added_indices.push(p);
                        largest_joltage(input, added_indices, joltage, length);
                        break;
                    }
                }
            }
        }
    }
}

fn p1(input: &str) -> i64 {
    let data = parse_data(input);
    let mut sum = 0;

    for d in data {
        let mut joltage = "".to_string();
        let mut added_indices: Vec<usize> = Vec::new();
        largest_joltage(&d, &mut added_indices, &mut joltage, 2);
        let j: i64 = joltage.parse().unwrap();
        sum += j;
    }
    sum
}

fn largest_joltage_p2(input: String, length: usize) -> i64 {
    let mut joltage: String = "".to_string();
    let jol_length = length;
    let mut current_index = -1;

    while joltage.len() < jol_length {
        let mut value_added = false;
        for i in ('1'..='9').rev() {
            let positions: Vec<usize> = input
                .char_indices() // iterate over (index, char)
                .filter(|(_, c)| *c == i) // keep only i
                .map(|(i, _)| i) // take the index
                .collect();
            for pos in positions {
                let places_to_fill = jol_length - joltage.len();
                let numbers_left = input.len() - pos;
                let can_be_added = places_to_fill <= numbers_left;
                if pos as i32 > current_index && can_be_added {
                    joltage.push(i);
                    current_index = pos as i32;
                    value_added = true;
                    break;
                }
            }
            if value_added {
                break;
            }
        }
    }
    let value = joltage.parse().unwrap();
    value
}

fn p2(input: &str) -> i64 {
    let data = parse_data(input);
    let mut sum = 0;

    for d in data {
        let number = largest_joltage_p2(d, 12);
        sum += number;
    }
    sum
}

fn main() {
    let start = Instant::now();
    let part1 = p1(INPUT);
    let part2 = p2(INPUT);

    utils::answer((part1, 17087), (part2, 169019504359949));

    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT_EXAMPLE: &str = "example";

    #[test]
    fn test6() {
        let sum = p1(INPUT_EXAMPLE);
        assert_eq!(sum, 357);
    }
    #[test]
    fn test7() {
        let sum = p2(INPUT_EXAMPLE);
        assert_eq!(sum, 3121910778619);
    }
}
