use lib::filereader;
use itertools::Itertools;
use std::collections::HashSet;

fn unique_numbers(input: Vec<i32>) -> Vec<i32> {
    let mut seen = HashSet::new();
    input
        .into_iter()
        .filter(|x| seen.insert(*x)) // Insert into the HashSet, and keep only unique values
        .collect()
}


fn main()
{
    let (ordering_vec,pages_vec) = parse_data("../input/day05");

    let part1 = part1(&ordering_vec, &pages_vec);
    println!("{}", part1);

    let part2 = part2(&ordering_vec, &pages_vec);
    println!("{}", part2);
    // println!("{}", part2);

    // assert_eq!(part1, 2500);
    // assert_eq!(part2, 1933);
}

fn part2(ordering_vec: &Vec<Vec<i128>>, pages_vec: &Vec<Vec<i128>>) ->i128 {
    let mut sum = 0;
    for page in pages_vec {
        if !has_correct_order(&ordering_vec, &page) {
            println!("{:?}", &page);
            let ordered_page = order_incorrect_page(ordering_vec, page);
            sum += get_middle_value(&ordered_page);
        } 
    }
    sum
}

fn order_incorrect_page(ordering_vec: &Vec<Vec<i128>>, page:&Vec<i128>) -> Vec<i128> {
    let corrected_page = Vec::new();
    let page_cp = page.clone();
    let length = page_cp.len();

    get_incorrect_indices(ordering_vec, page);
    // for perm in page_cp.into_iter().permutations(length) {
    //     if has_correct_order(&ordering_vec, &perm) {
    //         println!("{:?}",perm);
    //         return perm;
    //     }
    // }
    corrected_page
}

fn part1(ordering_vec: &Vec<Vec<i128>>, pages_vec: &Vec<Vec<i128>>) ->i128 {
    let mut sum = 0;
    for page in pages_vec {
        if has_correct_order(&ordering_vec, &page) {
            sum += get_middle_value(&page);
        }
    }
    sum
}

fn get_middle_value(page:&Vec<i128>) -> i128{
    return page[page.len()/2];
}

fn get_incorrect_indices(ordering_vec:&Vec<Vec<i128>>, page:&Vec<i128>) {
    let mut wrong_indices:Vec<i32> = Vec::new();
    for i in 0..(page.len()-1) {
        let vec = vec![page[i],page[i+1]];
        if !ordering_vec.contains(&vec) {
            wrong_indices.push(i as i32);
            wrong_indices.push((i+1) as i32);
        }
    }
    let unique_numbers = unique_numbers(wrong_indices);
    println!("{:?}", unique_numbers);
}

fn has_correct_order(ordering_vec:&Vec<Vec<i128>>, page:&Vec<i128>) -> bool {
    for i in 0..(page.len()-1) {
        let vec = vec![page[i],page[i+1]];
        if !ordering_vec.contains(&vec) {
            return false;
        }
    }
    return true;
}

fn parse_data(contents: &str) -> (Vec<Vec<i128>>, Vec<Vec<i128>>)
{
    let contents = filereader::_input(&contents);
    let mut vec1:Vec<Vec<i128>> = Vec::new();
    let mut vec2:Vec<Vec<i128>> = Vec::new();

    for content in contents.lines() {
        if content.contains('|') {
            let numbers: Vec<i128> = content
            .split('|')    
            .map(|s| s.parse::<i128>().unwrap()) 
            .collect();
            vec1.push(numbers);
        } else if content.contains(',') {
            let numbers: Vec<i128> = content
            .split(',')
            .map(|s| s.parse::<i128>().unwrap())
            .collect();
            vec2.push(numbers);
        }
    }
    (vec1,vec2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = parse_data("test1");
        let page = vec![75,47,61,53,29];
        let valid_order = has_correct_order(&data.0, &page);
        assert!(valid_order);
    }

    #[test]
    fn test2() {
        let data = parse_data("test1");
        let page = vec![97,13,75,29,47];
        let valid_order = has_correct_order(&data.0, &page);
        assert!(!valid_order);
    }

    #[test]
    fn test3() {
        let data = parse_data("test1");
        let part1 = part1(&data.0, &data.1);
        assert_eq!(part1,143);
    }

    #[test]
    fn test4() {
        let data = parse_data("test1");
        let invalid_page: Vec<i128> = vec![75,97,47,61,53];
        let page = order_incorrect_page(&data.0, &invalid_page);
        assert_eq!(page, vec![97,75,47,61,53]);
    }

    #[test]
    fn test5() {
        let data = parse_data("test1");
        let part1 = part2(&data.0, &data.1);
        assert_eq!(part1,123);
    }

    #[test]
    fn test6() {
        let data = parse_data("../input/day05");
        let vec = vec![56, 85, 42, 32, 48, 43, 62, 57, 11, 74, 18, 81, 24, 39, 31, 86, 16, 19, 38];
        let part1 = order_incorrect_page(&data.0, &vec);
        let vec = vec![82,32,83,18,92,98,53,26,17,86,38,47,94,58,42,56,12,89,54,29,77,87,13];
        let part1 = order_incorrect_page(&data.0, &vec);
        // assert_eq!(part1,123);
    }
}