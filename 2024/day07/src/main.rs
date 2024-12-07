use itertools::Position;
use lib::filereader;
use lib::grid::Grid;
use lib::utils::*;

struct Calculation {
    answer:i128,
    values:Vec<i128>,
}

fn main()
{
    let part1 = part1("../input/day07");
    
    println!("{}",part1);
    assert_eq!(part1, 1260333054159);
    
    // let part2 = part2(grid);
    // println!("{}",part2);
    // assert_eq!(part2, 1503);
}
fn part1(input:&str) -> i128{
    let calculations = parse_data(input);
    let mut sum = 0;
    for calculation in &calculations {
        if calculate_configurations(calculation) > 0 {
            sum += calculation.0;
        }
    }
    sum
}

fn calculate_configurations(calculation:&(i128,Vec<i128>)) ->i128 {

    let mut values = calculation.1.clone();
    values.reverse();
    let answer = calculation.0;
    let mut configurations = 0;

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
        }
        it_vec = it_vec_copy;
        i += 1;
    }

    let vec_size = values.len();
    for value in it_vec {
        if value == values[vec_size-1] {
            configurations += 1;
        }
    }

    configurations
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let calculation = (190, vec![10,19]);
        let answer = calculate_configurations(&calculation);
        assert_eq!(answer,1);
    }
    
    #[test]
    fn test2() {
        let calculation = (3267, vec![81,40,27]);
        let answer = calculate_configurations(&calculation);
        assert_eq!(answer,2);
    }
    #[test]
    fn test3() {
        let calculation = (292, vec![11,6,16,20]);
        let answer = calculate_configurations(&calculation);
        assert_eq!(answer,1);
    }
    
    #[test]
    fn test4() {
        let answer = part1("test1");
        assert_eq!(answer, 3749);
    }
}