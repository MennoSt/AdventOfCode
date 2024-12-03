use lib::filereader;

fn main()
{
    let part1 = calculate_safe_reports(false);
    let part2 = calculate_safe_reports(true);
    println!("{}",part1);
    println!("{}",part2);
    assert_eq!(part1, 279);
    assert_eq!(part2, 343);
}

fn calculate_safe_reports(tolerate_single_bad:bool) -> i32 {
    let vec_numbers = parse_data("../input/day2");
    let mut counter = 0;
    for vec in &vec_numbers {
        if is_safe(&vec, tolerate_single_bad) {
            counter+=1;
        }
    }
    counter
}

fn parse_data(input: &str) -> Vec<Vec<i32>>  {
    let contents = filereader::_input(input);
    let mut vec1: Vec<Vec<i32>> = Vec::new();
    
    for content in contents.lines() {
        let numbers: Vec<i32> = content
        .split_whitespace() // Split by whitespace
        .filter_map(|s| s.parse::<i32>().ok()) // Parse each piece to an integer
        .collect();
        vec1.push(numbers);
    }
    vec1
}

fn is_only_increase_or_decrease(input:&Vec<i32>, tolerate_single_bad:bool) -> bool{
    
    let mut increased = false;
    let mut decreased = false;
    for i  in 1..input.len() {
        if input[i-1] > input[i] {
            decreased = true;
        }
        if input[i-1] < input[i] {
            increased = true;
        }
    }

    if ((increased && decreased) || (!increased && !decreased)) && tolerate_single_bad {
        for i  in 1..input.len() {
            let mut vec_copy1 = input.clone();
            let mut vec_copy2 = input.clone();
            vec_copy1.remove(i);
            vec_copy2.remove(i-1);
            if is_only_increase_or_decrease(&vec_copy1, false) && has_correct_difference(&vec_copy1, false)|| 
            is_only_increase_or_decrease(&vec_copy2, false) && has_correct_difference(&vec_copy2, false) {
                return true;
            }
        }
    }
    
    if (increased && decreased) || (!increased && !decreased) {
        return false
    }
    true
}

fn has_correct_difference(input:&Vec<i32>, tolerate_single_bad:bool) -> bool {

    for i  in 1..input.len() {
        let diff = (input[i-1]-input[i]).abs();
        if diff > 3 || diff < 1{
            if tolerate_single_bad {
                let mut vec_copy1 = input.clone();
                let mut vec_copy2 = input.clone();
                vec_copy1.remove(i);
                vec_copy2.remove(i-1);
                
                if has_correct_difference(&vec_copy1, false) && is_only_increase_or_decrease(&vec_copy1, false)  || 
                has_correct_difference(&vec_copy2, false) && is_only_increase_or_decrease(&vec_copy2, false) {
                    return true;
                }
            }
            return false;
        }        
    }
    true
}

fn is_safe(input:&Vec<i32>, tolerate_single_bad:bool) -> bool {
    return is_only_increase_or_decrease(&input, tolerate_single_bad) && 
        has_correct_difference(&input,tolerate_single_bad)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let numbers = vec![7,6,4,2,1];
        let is_safe=is_safe(&numbers,true);
        assert_eq!(is_safe, true);
    }

    #[test]
    fn test2() {
        let numbers = vec![1,2,7,8,9];
        let is_safe=is_safe(&numbers,true);
        assert_eq!(is_safe, false);
    }

    #[test]
    fn test3() {
        let numbers = vec![9,7,6,2,1];
        let is_safe=is_safe(&numbers, true);
        assert_eq!(is_safe, false);
    }

    #[test]
    fn test4() {
        let numbers = vec![1,3,2,4,5];
        let is_safe=is_safe(&numbers, true);
        assert_eq!(is_safe, true);
    }

    #[test]
    fn test5() {
        let numbers = vec![8,6,4,4,1];
        let is_safe=is_safe(&numbers,true);
        assert_eq!(is_safe, true);
    }

    #[test]
    fn test6() {
        let numbers = vec![1,3,6,7,9];
        let is_safe=is_safe(&numbers,true);
        assert_eq!(is_safe, true);
    }
}