
use std::fs;
use std::env;

fn main() {
    let current_dir = env::current_dir().unwrap();
    let file_path = current_dir.join("../input/day01");
    let contents = fs::read_to_string(file_path).unwrap();
    
    let filepath_example = current_dir.join("exampleinput2023day1");
    let contents_example = fs::read_to_string(filepath_example).unwrap();

    let sum = calculate_sum_part1(contents.clone());
    assert_eq!(sum, 55621);
    println!("Answer part 1 {}",sum);

    let sum_example = calculate_sum_part2(contents_example.clone());
    assert_eq!(sum_example,281);
    let sum_part2 = calculate_sum_part2(contents.clone());
    println!("Answer part 2 {}",sum_part2);
}

fn find_first_digit_left(number_string: &str) -> String {
    let vector_of_tuples: Vec<(&str, &str)> = vec![("1", "one"), ("2", "two"),("3", "three"),("4", "four"), ("5", "five"),
        ("6", "six"), ("7","seven"), ("8","eight"), ("9","nine")];

    for i in 0..number_string.len() 
    {
        for vec in vector_of_tuples.clone() {        
            if number_string[0..i].contains(vec.1)
            {
                return vec.0.to_string();
            }
        }

        let number = number_string.chars().nth(i).unwrap();
        if number.is_numeric() 
        {
            return number.to_string();
        }
    }
    return "".to_string();
}

fn find_first_digit_right(number_string: &str) -> String {
    let vector_of_tuples: Vec<(&str, &str)> = vec![("1", "one"), ("2", "two"),("3", "three"),("4", "four"), ("5", "five"),
        ("6", "six"), ("7","seven"), ("8","eight"), ("9","nine")];

    let str_length = number_string.len();
    for i in 0..str_length
    {
        for vec in vector_of_tuples.clone() {        
            if number_string[str_length-i..str_length].contains(vec.1)
            {
                return vec.0.to_string();
            }
        }

        let number = number_string.chars().nth(str_length-1-i).unwrap();
        if number.is_numeric() 
        {
            return number.to_string();
        }
    }
    return "".to_string();
}

fn calculate_sum_part2(contents: String) -> i32 {
    let mut sum = 0;
    for line in contents.lines() {
        let digit_left = find_first_digit_left(line);
        let digit_right = find_first_digit_right(line);
        let left_and_right = digit_left + &digit_right;

        sum += left_and_right.parse::<i32>().unwrap();
    }
    sum
}

fn calculate_sum_part1(contents: String) -> i32 {
    let mut sum = 0;
    for line in contents.lines() {
        sum += get_firstlast_digit(line.to_string());
    }
    sum
}

fn get_firstlast_digit(input_string: String) -> i32{

    let mut numeric_str = String::new();
    for s in input_string.chars() {
        if s.is_numeric() {
            numeric_str.push(s);
        }
    }

    let first_numeric = numeric_str.chars().next().unwrap_or_default();
    let last_numeric = numeric_str.chars().last().unwrap_or_default();
    let mut digits = String::new();
    digits.push(first_numeric);
    digits.push(last_numeric);
    let num:i32 = digits.parse().unwrap();
    return num;
}
