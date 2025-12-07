use lib::filereader;
use lib::utils;
use std::time::Instant;

static INPUT: &str = "../input/day05";

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
struct IDRange {
    start: i128,
    end: i128,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
struct Ingredients {
    id_ranges: Vec<IDRange>,
    ingredient_ids: Vec<i128>,
}

fn parse_data(input: &str) -> Ingredients {
    let content = filereader::_input(input);
    let mut ranges: Vec<IDRange> = Vec::new();
    let mut ingredients: Vec<i128> = Vec::new();

    let mut second = false;
    for content in content.lines() {
        if second == false && !content.is_empty() {
            let mut parts = content.split('-').map(|p| p.parse::<i128>().unwrap());
            let a: i128 = parts.next().unwrap();
            let b: i128 = parts.next().unwrap();
            let id_range = IDRange { start: a, end: b };
            ranges.push(id_range);
        }

        if second == true {
            let parts: i128 = content.parse().unwrap();
            ingredients.push(parts);
        }

        if content.is_empty() {
            second = true;
        }
    }
    Ingredients {
        id_ranges: ranges,
        ingredient_ids: ingredients,
    }
}

fn p1(input: &str) -> i128 {
    let ingredients = parse_data(input);
    let mut sum = 0;

    for id in &ingredients.ingredient_ids {
        for range in &ingredients.id_ranges {
            if *id >= range.start && *id <= range.end 
            {
                println!("{}", id);
                sum+=1;
                break;
            }
        }
    }

    sum
}
fn main() {
    let start = Instant::now();
    let part1 = p1(INPUT);
    println!("{:?}", part1);
    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT_EXAMPLE: &str = "example";

    #[test]
    fn test1() {
        let sum = p1(INPUT_EXAMPLE);
        assert_eq!(sum, 3);
    }
}
