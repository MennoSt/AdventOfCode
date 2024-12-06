use lib::filereader;

fn main()
{
    let (ordering_vec,pages_vec) = parse_data("../input/day05");

    let part1 = part1(&ordering_vec, &pages_vec);
    println!("{}", part1);

    let part2 = part2(&ordering_vec, &pages_vec);
    println!("{}", part2);

    assert_eq!(part1, 4959);
    assert_eq!(part2, 4655);
}

fn part2(ordering_vec: &Vec<(i32,i32)>, pages_vec: &Vec<Vec<i32>>) ->i32 {
    let mut sum = 0;
    for page in pages_vec {
        if !has_correct_order(&ordering_vec, &page) {
            let ordered_page = order_incorrect_page(ordering_vec, page);
            sum += get_middle_value(&ordered_page);
        } 
    }
    sum
}

fn order_incorrect_page(ordering_vec: &Vec<(i32,i32)>, page:&Vec<i32>) -> Vec<i32> {
    let mut corrected_page = Vec::new();
    let mut found_page = Vec::new();
    let possible_pairs = calculate_possible_pairs(page, ordering_vec);
    let mut result = Vec::new();

    for pair in &possible_pairs {
        corrected_page.push(*pair);
        calc_chain(&possible_pairs, pair, &mut corrected_page, &mut found_page,page.len() as i32);
        corrected_page.pop();
    }  

    result.push(found_page[0].0);
    for pair in found_page {
        result.push(pair.1);
    }

    return result;
}

fn calc_chain(possible_pairs: &Vec<(i32, i32)>, pair_prev: &(i32, i32), corrected_page: &mut Vec<(i32,i32)>, found_page: &mut Vec<(i32,i32)>, length:i32) {
    for pair in possible_pairs {
        if pair_prev.1 == pair.0 {
            corrected_page.push(*pair);
            if corrected_page.len() as i32 == (length-1) {
                *found_page = corrected_page.clone();
            }
            calc_chain(possible_pairs, pair, corrected_page,found_page,length) ;
            corrected_page.pop();
        }
    }
}

fn calculate_possible_pairs(page: &Vec<i32>, ordering_vec: &Vec<(i32, i32)>) -> Vec<(i32, i32)> {

    let mut possible_pairs:Vec<(i32,i32)> = Vec::new();
    for vec in ordering_vec {
        if page.contains(&vec.0) && page.contains(&vec.1) {
            possible_pairs.push(*vec);
        }
    }

    possible_pairs.dedup();
    possible_pairs
}

fn part1(ordering_vec: &Vec<(i32,i32)>, pages_vec: &Vec<Vec<i32>>) ->i32 {
    let mut sum = 0;
    for page in pages_vec {
        if has_correct_order(&ordering_vec, &page) {
            sum += get_middle_value(&page);
        }
    }
    sum
}

fn get_middle_value(page:&Vec<i32>) -> i32{
    return page[page.len()/2];
}

fn has_correct_order(ordering_vec:&Vec<(i32,i32)>, page:&Vec<i32>) -> bool {
    for i in 0..(page.len()-1) {
        let vec = (page[i],page[i+1]);
        if !ordering_vec.contains(&vec) {
            return false;
        }
    }
    return true;
}

fn parse_data(contents: &str) -> (Vec<(i32,i32)>, Vec<Vec<i32>>)
{
    let contents = filereader::_input(&contents);
    let mut vec1:Vec<(i32,i32)> = Vec::new();
    let mut vec2:Vec<Vec<i32>> = Vec::new();

    for content in contents.lines() {
        if content.contains('|') {
            let numbers: Vec<i32> = content
            .split('|')    
            .map(|s| s.parse::<i32>().unwrap()) 
            .collect();
            let test = (numbers[0],numbers[1]);
            vec1.push(test);
        } else if content.contains(',') {
            let numbers: Vec<i32> = content
            .split(',')
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
            vec2.push(numbers);
        }
    }
    vec1.sort_by_key(|k| k.0);
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
        let invalid_page: Vec<i32> = vec![75,97,47,61,53];
        let page = order_incorrect_page(&data.0, &invalid_page);
        assert_eq!(page, vec![97,75,47,61,53]);
    }

    #[test]
    fn test5() {
        let data = parse_data("test1");
        let part1 = part2(&data.0, &data.1);
        assert_eq!(part1,123);
    }
}