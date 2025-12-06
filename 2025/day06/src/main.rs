use lib::{filereader, utils};
use std::time::Instant;

static INPUT: &str = "../input/day06";

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
enum Operate {
    Added,
    Multiplied,
}

struct Operation {
    values: Vec<i128>,
    operation: Operate,
}

fn parse_data(input: &str) -> Vec<Operation> {
    let contents = filereader::_input(input);
    let mut data: Vec<Vec<i128>> = Vec::new();
    let mut operations: Vec<char> = Vec::new();

    for line in contents.lines() {
        let numbers: Option<Vec<i128>> = {
            let v: Vec<_> = line
                .split_whitespace()
                .filter_map(|s| s.parse::<i128>().ok())
                .collect();

            if v.is_empty() {
                None
            } else {
                Some(v)
            }
        };

        if let Some(nums) = numbers {
            data.push(nums);
        } else {
            // Fallback: parse chars
            let chars: Vec<char> = line
                .split_whitespace()
                .filter_map(|s| s.chars().next())
                .collect();

            operations = chars;
        }
    }

    let mut operations_vec: Vec<Operation> = Vec::new();

    for i in 0..operations.len() {
        let mut values_op: Vec<i128> = Vec::new();
        for vec in &data {
            values_op.push(vec[i]);
        }

        let mut operate = Operate::Added;
        if operations[i] == '*' {
            operate = Operate::Multiplied;
        } else {
            operate = Operate::Added;
        }

        operations_vec.push(Operation {
            values: values_op,
            operation: operate,
        })
    }

    operations_vec
}

fn p1(input: &str) -> i128 {
    let operations = parse_data(input);
    let mut sum = 0;
    for o in operations {
        if o.operation == Operate::Added {
            let answer: i128 = o.values.iter().sum();
            sum += answer;
        } else {
            let answer: i128 = o.values.iter().product();
            sum += answer;
        }
    }
    sum
}

fn p2(input: &str) -> i128 {
    let operations = parse_data(input);
    let mut sum = 0;
    for o in operations {

        if o.operation == Operate::Added {
            let new_vec = transpose_vec(o.values);
            let answer: i128 = new_vec.iter().sum();
            // println!("{}", answer);
            sum += answer;
        } else {
            let new_vec = transpose_vec_mult(o.values);
            let answer: i128 = new_vec.iter().product();
            // println!("{}", answer);
            sum += answer;
        }
    }
    sum
}

fn transpose_vec(values: Vec<i128>) -> Vec<i128> {
    let mut transposed_values: Vec<i128> = Vec::new();

    let max = values.iter().max().unwrap().to_string().len();

    for i in 0..max +2 {
        let mut new_str: String = "".to_string();
        for val in &values {
            let str = val.to_string();
            if let Some(c) = str.chars().nth(i) {
                new_str.push(c);
                // println!("The {}-th character is '{}'", i, c);
            } else {
                // println!("String is too short");
            }
        }
        if !new_str.is_empty()
        {
            let newstr_val = new_str.parse::<i128>().unwrap();
            transposed_values.push(newstr_val);
        }
    }

    transposed_values
}

fn transpose_vec_mult(values: Vec<i128>) -> Vec<i128> {
    let mut transposed_values: Vec<i128> = Vec::new();

    let max = values.iter().max().unwrap().to_string().len();

    for i in 0..max+2 {
        let mut new_str: String = "".to_string();
        for val in &values {
            let mut str = val.to_string();
            if let Some(c) = str.chars().rev().nth(i) {
                new_str.push(c);
                // println!("The {}-th character is '{}'", i, c);
            } else {
                // println!("String is too short");
            }
        }
        if !new_str.is_empty()
        {
            let newstr_val = new_str.parse::<i128>().unwrap();
            transposed_values.push(newstr_val);
        }
    }

    transposed_values
}

fn main() {
    let start = Instant::now();

    let answerp1 = p1(INPUT);
    println!("{}", answerp1);
    let answerp2 = p2(INPUT);
    println!("{}", answerp2);
    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
}

#[cfg(test)]
mod tests {
    use std::f32::consts::E;

    use super::*;
    static INPUT_EXAMPLE: &str = "example";

    #[test]
    fn test1() {
        let sum = p1(INPUT_EXAMPLE);
        assert_eq!(sum, 4277556);
    }

    #[test]
    fn test2() {
        let sum = p2(INPUT_EXAMPLE);
        assert_eq!(sum, 3263827);
    }
    #[test]
    fn test3() {
        let vec: Vec<i128> = vec![64, 23, 314];

        let mut transposed = transpose_vec(vec);
        let mut expected_vec = vec![4, 431, 623];

        transposed.sort();
        expected_vec.sort();

        assert_eq!(transposed, expected_vec);
    }

    #[test]
    fn test4() {
        let vec: Vec<i128> = vec![123, 45, 6];

        let mut transposed = transpose_vec_mult(vec);
        let mut expected_vec = vec![1, 24, 356];

        transposed.sort();
        expected_vec.sort();

        assert_eq!(transposed, expected_vec);
    }
}
