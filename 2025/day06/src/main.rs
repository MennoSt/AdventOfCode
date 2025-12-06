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

        let operate;
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

fn parse_data_p2(input: &str)   {
    let contents = filereader::_input(input);
    let mut data: Vec<String> = Vec::new();
    let mut operations: Vec<char> = Vec::new();

    let mut returnValue: Vec<Operate> = Vec::new();

    for line in contents.lines() {
        if line.chars().any(|c| c.is_ascii_digit()) {
            data.push(line.to_string());
            println!("String contains digits!");
        } else {
            println!("No digits.");
            // Fallback: parse chars
            let chars: Vec<char> = line
                .split_whitespace()
                .filter_map(|s| s.chars().next())
                .collect();

            operations = chars;
        }
    }

    let mut values_vec_ints:Vec<Vec<i32>> = Vec::new();
    let mut values:Vec<i32> = Vec::new();
    for i in 0..data[0].len() {
        let mut value = "".to_string();
        for d in &data {
            let val = d.chars().nth(i).unwrap();
            if val != ' '
            {
                value.push(val);
            }
        }
        if value.is_empty()
        {
            println!("{:?}",values);
            values_vec_ints.push(values.clone());
            values.clear();
            // let values:Vec<i32> = Vec::new();
        }
        else{
            let valInt:i32 = value.parse().unwrap();
            values.push(valInt);
        }
    }
    println!("{:?}",values);
    values_vec_ints.push(values.clone());

}

// 12866637663746 too high
fn main() {
    let start = Instant::now();

    let answerp1 = p1(INPUT);
    println!("{}", answerp1);
    assert_eq!(answerp1, 7644505810277);
    // let answerp2 = p2(INPUT);
    // println!("{}", answerp2);
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
        parse_data_p2(INPUT_EXAMPLE);
        // assert_eq!(sum, 3263827);
    }
}
