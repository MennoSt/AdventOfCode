use lib::filereader;

fn main()
{
    let part1 = part1();
    assert_eq!(part1, 279);
}

fn part1() -> i32 {
    let vec_numbers = parse_data("../input/day2");

    let mut counter = 0;
    for vec in vec_numbers {
        if is_safe(vec) {
            counter+=1;
        }
    }

    println!("{:?}",counter);
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

fn is_only_increase_or_decrease(input:&Vec<i32>) -> bool{
    
    let mut increased = false;
    let mut decreased = false;
    for i  in 1..input.len() {
        if input[i-1] >= input[i] {
            decreased = true;
        }
        if input[i-1] <= input[i] {
            increased = true;
        }
    }

    if increased && decreased {
        return false;
    }
    true
}

fn has_correct_difference(input:&Vec<i32>) -> bool {

    for i  in 1..input.len() {
        let diff = (input[i-1]-input[i]).abs();
        if diff > 3 || diff == 0 {
            return false;
        }        
    }
    true
}

fn is_safe(input:Vec<i32>) -> bool {
    return is_only_increase_or_decrease(&input) && has_correct_difference(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let numbers = vec![7,6,4,2,1];
        let is_safe=is_safe(numbers);
        assert_eq!(is_safe, true);
    }

    #[test]
    fn test2() {
        let numbers = vec![1,2,7,8,9];
        let is_safe=is_safe(numbers);
        assert_eq!(is_safe, false);
    }

    #[test]
    fn test3() {
        let numbers = vec![9,7,6,2,1];
        let is_safe=is_safe(numbers);
        assert_eq!(is_safe, false);
    }


}