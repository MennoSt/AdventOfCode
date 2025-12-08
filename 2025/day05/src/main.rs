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
            if *id >= range.start && *id <= range.end {
                sum += 1;
                break;
            }
        }
    }

    sum
}

fn calculate_total_ranges(ingredients: Ingredients) -> i128 {
    let id_ranges = ingredients.id_ranges.clone();
    let mut total_range_values = 0;

    for range in id_ranges {
        let length = range.end - range.start + 1;
        total_range_values += length
    }
    total_range_values
}

fn calculate_duplicates(r1: IDRange, r2: IDRange) -> i128 {
    let mut total = 0;

    if r1.start >= r2.start && r1.end <= r2.end {
        let duplicates = r1.end - r1.start + 1;
        total += duplicates;
    } else if r2.start >= r1.start && r2.end <= r1.end {
        let duplicates = r2.end - r2.start + 1;
        total += duplicates;
    } else if r1.start <= r2.end && r1.start >= r2.start {
        let duplicates = r2.end - r1.start + 1;
        total += duplicates;
    } else if r2.start <= r1.end && r2.start >= r1.start {
        let duplicates = r1.end - r2.start + 1;
        total += duplicates;
    }

    total
}

fn p2(input: &str) -> i128 {
    let ingredients = parse_data(input);
    let total = calculate_total_ranges(ingredients.clone());
    let ranges_first = ingredients.id_ranges.clone();
    let ranges_copy = ingredients.id_ranges.clone();

    let mut duplicates = 0;

    for range1 in &ranges_first {
        for range2 in &ranges_copy {
            if range1 == range2 {
                continue;
            }
            let value = calculate_duplicates(range1.clone(), range2.clone());

            if value > 0 {
                duplicates += value;
                println!("duplicates: {:?} {:?}  value:{}", range1, range2, value);
            }
        }
    }
    duplicates /= 2;

    total - duplicates
}

// wrong 295612583176232
// 335848561105680
// 369151706611413

fn main() {
    let start = Instant::now();
    let part1 = p1(INPUT);
    println!("{:?}", part1);
    let part2 = p2(INPUT);
    println!("{:?}", part2);
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

    #[test]
    fn test2() {
        let sum = p2(INPUT_EXAMPLE);
        assert_eq!(sum, 14);
    }
    
    #[test]
    fn test3() {
        let sum = p2("example2");
        assert_eq!(sum, 3);
    }


    #[test]
    fn test4() {
    
        let range1 = IDRange { start: 4, end: 6 };
        let range2 = IDRange { start: 1, end: 5 };
        let result = 2;

        assert_eq!(calculate_duplicates(range1.clone(), range2.clone()), result);
        assert_eq!(calculate_duplicates(range2, range1), result);
    }

    #[test]
    fn test5() {
        let range1 = IDRange { start: 5, end: 6 };
        let range2 = IDRange { start: 1, end: 5 };
        let result = 1;

        assert_eq!(calculate_duplicates(range1.clone(), range2.clone()), result);
        assert_eq!(calculate_duplicates(range2, range1), result);
    }

    #[test]
    fn test6() {

        let range1 = IDRange { start: 5, end: 7 };
        let range2 = IDRange { start: 5, end: 6 };
        let result = 2;

        assert_eq!(calculate_duplicates(range1.clone(), range2.clone()), result);
        assert_eq!(calculate_duplicates(range2, range1), result);
    }

    #[test]
    fn test8() {
        let range1 = IDRange { start: 98, end: 98 };
        let range2 = IDRange { start: 99, end: 100 };
        let result = 0;

        assert_eq!(calculate_duplicates(range1.clone(), range2.clone()), result);
        assert_eq!(calculate_duplicates(range2, range1), result);
    }
    #[test]

    fn test7() {
        let range1 = IDRange { start: 0, end: 10 };
        let range2 = IDRange { start: 10, end: 10 };
        let result = 1;

        assert_eq!(calculate_duplicates(range1.clone(), range2.clone()), result);
        assert_eq!(calculate_duplicates(range2, range1), result);
    }
}
