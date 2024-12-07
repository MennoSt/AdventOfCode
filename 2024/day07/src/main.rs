use lib::filereader;
use std::time::Instant;

static INPUT: &str = "../input/day07";

fn remove_last_digits_if_match(mut num: i128, x: i128) -> (i128, bool) {
    let mut match_found = false;
    let value = x.to_string();
    let n = value.len();
    let divisor = 10_i32.pow(n as u32) as i128;

    if num % divisor == x {
        num /= divisor;
        match_found=true;
    }
    (num, match_found)
}

fn parse_data(input:&str) -> Vec<(i128,Vec<i128>)> {
    let mut calculations = Vec::new();
    let content= filereader::_input(input);
    for line in content.lines()
    {
        let test:Vec<&str> = line.split(": ").collect();   
        let answer:i128 = test[0].parse().unwrap();
        let values:Vec<i128> = test[1].split_whitespace()
                                      .filter_map(|s| s.parse::<i128>().ok())
                                      .collect();
                                    calculations.push((answer,values));
                                }
                                calculations
                            }
                            
                            fn sum(input:&str, concatenation_enabled:bool) -> i128{
    let calculations = parse_data(input);
    let mut sum = 0;
    for calculation in &calculations {
        if solve(calculation, concatenation_enabled) {
            sum += calculation.0;
        }
    }
    sum
}

fn solve(calculation:&(i128,Vec<i128>), concatenation_enabled:bool ) -> bool {
    let mut values = calculation.1.clone();
    values.reverse();
    let answer = calculation.0;
    let mut it_vec = vec![answer];
    let mut i = 0;
    
    while i < (values.len()-1){
        let mut it_vec_copy = Vec::new();
        for it in &it_vec {
            if it%values[i] == 0 {
                let it1 = it / values[i];
                it_vec_copy.push(it1);
            }
            
            let it2 = it - values[i];
            it_vec_copy.push(it2);
            
            if concatenation_enabled {
                let it3 = remove_last_digits_if_match(it.clone(), values[i]);
                if it3.1 {
                    it_vec_copy.push(it3.0);
                }
            }
        }
        it_vec = it_vec_copy;
        i += 1;
    }
    
    let vec_size = values.len();
    for value in it_vec {
        if value == values[vec_size-1] {
            return true;
        }
    }
    false
}

fn main()
{
    let start = Instant::now();

    let part1 = sum(INPUT, false);
    println!("{}",part1);
    assert_eq!(part1, 1260333054159);
    
    let part2 = sum(INPUT, true);
    println!("{}",part2);
    assert_eq!(part2,162042343638683);
    
    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let calculation = (190, vec![10,19]);
        let answer = solve(&calculation,false);
        assert_eq!(answer,true);
    }
    
    #[test]
    fn test2() {
        let calculation = (3267, vec![81,40,27]);
        let answer = solve(&calculation, false);
        assert_eq!(answer,true);
    }
    #[test]
    fn test3() {
        let calculation = (292, vec![11,6,16,20]);
        let answer = solve(&calculation, false);
        assert_eq!(answer,true);
    }
    
    #[test]
    fn test4() {
        let answer = sum("test1",false);
        assert_eq!(answer, 3749);
    }
    
    #[test]
    fn test5() {
        let calculation = (156, vec![15,6]);
        let answer = solve(&calculation, true);
        assert_eq!(answer,true);
    }

    #[test]
    fn test6() {
        let calculation = (7290, vec![6,8,6,15]);
        let answer = solve(&calculation,true);
        assert_eq!(answer,true);
    }

    #[test]
    fn test7() {
        let answer = sum("test1",true);
        assert_eq!(answer, 11387);
    }
    #[test]
    fn test8() {
        let calculation = (1491, vec![1,491]);
        let answer = solve(&calculation,true);
        assert_eq!(answer, true);
    }
    
    #[test]
    fn test9() {
        let answer = remove_last_digits_if_match(1491,491);
        assert_eq!(answer,(1,true));
    }

    #[test]
    fn test10() {
        let answer = remove_last_digits_if_match(12,4);
        assert_eq!(answer,(12,false));
    }
}