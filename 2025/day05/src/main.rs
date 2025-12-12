use lib::filereader;
use lib::utils;
use std::sync::MutexGuard;
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

fn has_duplicates(r1: IDRange, r2: IDRange) -> bool {
    if (calculate_duplicates(r1, r2)) > 0 {
        return true;
    } else {
        return false;
    }
}

fn merge_ranges(ranges: Vec<IDRange>) -> Vec<IDRange> {
    let current_vec = ranges.clone();
    let mut mut_vec = Vec::new();

    let mut in_range_vecs = Vec::new();
    for i in 0..current_vec.len() {
        for j in 0..current_vec.len() {
            in_range_vecs.push(current_vec[i].clone());
            if has_duplicates(current_vec[i].clone(), current_vec[j].clone()) {
                in_range_vecs.push(current_vec[j].clone());
            }
        }
        let mut min = in_range_vecs[0].start;
        let mut max = in_range_vecs[0].end;
        for range in &in_range_vecs {
            if range.start < min {
                min = range.start;
            }
            if range.end > max {
                max = range.end
            }
        }

        mut_vec.push(IDRange {
            start: min,
            end: max,
        });
        in_range_vecs.clear();
    }

    mut_vec.sort_by(|a, b| a.start.cmp(&b.start).then(a.end.cmp(&b.end)));
    mut_vec.dedup();

    mut_vec
}

fn p2(input: &str) -> i128 {
    let ingredients = parse_data(input);

    let merged_ranges = merge_ranges(ingredients.id_ranges.clone());
    let merged_ranges = merge_ranges(merged_ranges.clone());
    let merged_ranges = merge_ranges(merged_ranges.clone());

    let mut sum = 0;
    for range in merged_ranges {
        let diff = range.end - range.start + 1;
        sum += diff;
    }
    sum
}


fn main() {
    let start = Instant::now();
    let part1 = p1(INPUT);
    println!("{:?}", part1);
    assert_eq!(part1, 694);
    let part2 = p2(INPUT);
    println!("{:?}", part2);
    assert_eq!(part2, 352716206375547);
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
    fn test7() {
        let range1 = IDRange { start: 0, end: 10 };
        let range2 = IDRange { start: 10, end: 10 };
        let result = 1;

        assert_eq!(calculate_duplicates(range1.clone(), range2.clone()), result);
        assert_eq!(calculate_duplicates(range2, range1), result);
    }

    #[test]
    fn test8() {
        let range1 = IDRange { start: 98, end: 98 };
        let range2 = IDRange {
            start: 99,
            end: 100,
        };
        let result = 0;

        assert_eq!(calculate_duplicates(range1.clone(), range2.clone()), result);
        assert_eq!(calculate_duplicates(range2, range1), result);
    }

    #[test]
    fn test9() {
        let ranges: Vec<IDRange> = vec![IDRange { start: 5, end: 6 }, IDRange { start: 6, end: 7 }];

        assert_eq!(merge_ranges(ranges), vec![IDRange { start: 5, end: 7 }]);
    }

    #[test]
    fn test10() {
        let ranges: Vec<IDRange> = vec![IDRange { start: 2, end: 3 }, IDRange { start: 4, end: 7 }];

        assert_eq!(
            merge_ranges(ranges),
            vec![IDRange { start: 2, end: 3 }, IDRange { start: 4, end: 7 }]
        );
    }

    #[test]
    fn test11() {
        let ranges: Vec<IDRange> = vec![IDRange { start: 2, end: 3 }, IDRange { start: 2, end: 3 }];

        assert_eq!(merge_ranges(ranges), vec![IDRange { start: 2, end: 3 }]);
    }

    #[test]
    fn test12() {
        let ranges: Vec<IDRange> = vec![
            IDRange { start: 2, end: 3 },
            IDRange { start: 2, end: 4 },
            IDRange { start: 2, end: 5 },
        ];

        assert_eq!(merge_ranges(ranges), vec![IDRange { start: 2, end: 5 }]);
    }
}
