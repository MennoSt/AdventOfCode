use std::time::Instant;
use lib::filereader;
use lib::utils;

static INPUT: &str = "../input/day05";

fn main()
{
    let start = Instant::now();

    let contents = filereader::_input(INPUT);

    let part1 = reduce_polymer(&contents);
    let part2 = reduce_polymer_with_exclusion(&contents);

    utils::answer((part1,11264), (part2, 4552));

    println!("Execution time: {:?}", start.elapsed());
}

fn reduce_polymer(polymer: &str) -> usize {
    let mut stack: Vec<char> = Vec::with_capacity(polymer.len());

    for unit in polymer.chars() {
        if let Some(&last) = stack.last() {
            let same = last.to_ascii_lowercase() == unit.to_ascii_lowercase();
            let upper_and_lowercase =  last.is_lowercase() != unit.is_lowercase(); 
            if same && upper_and_lowercase {
                stack.pop();
                continue;
            }
        }
        stack.push(unit);
    }

    stack.len()
}

fn reduce_polymer_with_exclusion(polymer: &str) -> usize {
    let alphabet = b"abcdefghijklmnopqrstuvwxyz";
    let mut min_size = usize::MAX;

    for &letter in alphabet {
        let filtered = polymer
            .bytes()
            .filter(|&c| c != letter && c != letter.to_ascii_uppercase())
            .map(|b| b as char)
            .collect::<String>();

        let reduced_len = reduce_polymer(&filtered);
        min_size = min_size.min(reduced_len);
    }

    min_size
}

#[cfg(test)]
mod tests {
    use crate::{reduce_polymer, reduce_polymer_with_exclusion};

    #[test]
    fn test1() {
        let test_input = "dabAcCaCBAcCcaDA";
        let polymer = reduce_polymer(test_input);
        assert_eq!(polymer, 10);
    }

    #[test]
    fn test2() {
        let test_input = "dabAcCaCBAcCcaDA";
        let polymer = reduce_polymer_with_exclusion(test_input);
        assert_eq!(polymer, 4); 
    }  
}