use std::sync::mpsc::Receiver;
use std::time::Instant;
use lib::filereader;
use lib::utils;
use lib::utils::*;
use lib::grid::Grid;
use std::fmt::Debug;

static INPUT: &str = "../input/day05";
fn main()
{
    let start = Instant::now();

    let contents = filereader::_input(INPUT);
    let answerp1 = reduce_polymer(contents.clone());
    println!("{}",answerp1);
    assert_eq!(answerp1, 11264);
    let answerp2 = reduce_polymer_with_exclusion(contents);
    println!("{}",answerp2);
    assert_eq!(answerp2, 4552);

    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
}

fn reduce_polymer(polymer: String) -> usize {
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

fn reduce_polymer_with_exclusion(polymer: String) -> usize{

    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let mut vec:Vec<usize> = Vec::new();

    for letter in alphabet.chars()
    {
        let result = polymer.chars().filter(|&c| c != letter.to_ascii_uppercase() && c != letter)
            .collect();
    
        let length = reduce_polymer(result);
        vec.push(length);
    }
    
    let size = vec.iter().min().unwrap();
    *size
}

#[cfg(test)]
mod tests {
    use crate::{reduce_polymer, reduce_polymer_with_exclusion};

    #[test]
    fn test1() {
        let test_input = "dabAcCaCBAcCcaDA";
        let polymer = reduce_polymer(test_input.to_string());
        assert_eq!(polymer, 10);
    }

    #[test]
    fn test2() {
        let test_input = "dabAcCaCBAcCcaDA";
        let polymer = reduce_polymer_with_exclusion(test_input.to_string());
        assert_eq!(polymer, 4); 
    }  
}