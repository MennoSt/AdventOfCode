use lib::{filereader, utils};
use std::time::Instant;

static INPUT: &str = "../input/day06";

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
enum Operation {
    Added,
    Multiplied,
}

struct MathProblem {
    values: Vec<i128>,
    operation: Operation,
}

fn parse_data(input: &str) -> Vec<MathProblem> {
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
            let chars: Vec<char> = line
                .split_whitespace()
                .filter_map(|s| s.chars().next())
                .collect();

            operations = chars;
        }
    }

    let mut math_problems: Vec<MathProblem> = Vec::new();

    for i in 0..operations.len() {
        let mut values_op: Vec<i128> = Vec::new();
        for vec in &data {
            values_op.push(vec[i]);
        }

        let operate;
        if operations[i] == '*' {
            operate = Operation::Multiplied;
        } else {
            operate = Operation::Added;
        }

        math_problems.push(MathProblem {
            values: values_op,
            operation: operate,
        })
    }

    math_problems
}

fn parse_data_p2(input: &str) -> Vec<MathProblem> {
    let contents = filereader::_input(input);
    let mut data: Vec<String> = Vec::new();
    let mut operations: Vec<char> = Vec::new();
    let mut return_value: Vec<MathProblem> = Vec::new();

    for line in contents.lines() {
        if line.chars().any(|c| c.is_ascii_digit()) {
            data.push(line.to_string());
        } else {
            let chars: Vec<char> = line
                .split_whitespace()
                .filter_map(|s| s.chars().next())
                .collect();

            operations = chars;
        }
    }

    let mut values_vec_ints: Vec<Vec<i128>> = Vec::new();
    let mut values: Vec<i128> = Vec::new();
    for i in 0..data[0].len() {
        let mut value = "".to_string();
        for d in &data {
            let val = d.chars().nth(i).unwrap();
            if val != ' ' {
                value.push(val);
            }
        }
        if value.is_empty() {
            values_vec_ints.push(values.clone());
            values.clear();
        } else {
            let val_int: i128 = value.parse().unwrap();
            values.push(val_int);
        }
    }
    values_vec_ints.push(values.clone());

    for i in 0..operations.len() {
        let vec = values_vec_ints[i].clone();

        let operate;
        if operations[i] == '*' {
            operate = Operation::Multiplied;
        } else {
            operate = Operation::Added;
        }

        let op = MathProblem {
            values: vec,
            operation: operate,
        };
        return_value.push(op);
    }

    return_value
}

fn calculate_sum(operations: Vec<MathProblem>) -> i128 {
    let mut sum = 0;
    for o in operations {
        if o.operation == Operation::Added {
            let answer: i128 = o.values.iter().sum();
            sum += answer;
        } else {
            let answer: i128 = o.values.iter().product();
            sum += answer;
        }
    }
    sum
}

fn p1(input: &str) -> i128 {
    let operations = parse_data(input);
    calculate_sum(operations)
}

fn p2(input: &str) -> i128 {
    let operations = parse_data_p2(input);
    calculate_sum(operations)
}

fn main() {
    let start = Instant::now();

    let part1 = p1(INPUT);
    let part2 = p2(INPUT);
    utils::answer((part1, 7644505810277), (part2, 12841228084455));

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
        assert_eq!(sum, 4277556);
    }

    #[test]
    fn test2() {
        let sum = p2(INPUT_EXAMPLE);
        assert_eq!(sum, 3263827);
    }
}
