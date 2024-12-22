use lib::filereader;
use lib::utils;
use lib::utils::*;
use lib::grid::Gridi32;
use std::time::Instant;

static INPUT: &str = "../input/day11";
static TESTINPUT: &str = "test1";

fn main() {
    let start = Instant::now();
    
    // utils::answer((part1(INPUT), 794),(part2(INPUT), 1706));

    assert_eq!(part1(INPUT),193899);

    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
}

fn part1(file:&str) -> usize
{
    let stones = parse_data(file);
    
    calculate_stone_length(&stones, 25)
}

fn part2(file:&str) -> usize
{
    let stones = parse_data(file);
    
    calculate_stone_length(&stones, 75)
}

fn calculate_stone_length(vector:&Vec<i128>, blinks:i32) -> usize
{   
    let mut it =0;
    let mut vec_mutated = vector.clone();
    while it < blinks {
        vec_mutated = blink(&vec_mutated);
        it+=1;
        println!("{}",it);
        println!("{:?}", vec_mutated);
    }

    vec_mutated.len()
}

fn parse_data(string:&str) -> Vec<i128>
{
    let input = filereader::_input(string);
    input
    .split_whitespace() 
    .map(|s| s.parse::<i128>().unwrap())    
    .collect()
}

fn even_digits(number:i128) -> bool
{
    let s = number.to_string();
    let length = s.len();
    if utils::is_odd(length as i128)
    {
        return false
    }
    true
}

fn split_number(number:i128) ->(i128,i128)
{
    let s = number.to_string();
    let mid = s.len()/2;
    let values = s.split_at(mid);
    let value1 = values.0.parse::<i128>().unwrap();
    let value2 = values.1.parse::<i128>().unwrap();
    (value1,value2)
}
fn blink(vec_ints:&Vec<i128>) -> Vec<i128>
{
    let mut vec_mutated:Vec<i128> = Vec::new();
    for val in vec_ints {
        if *val == 0 {
            vec_mutated.push(1);
        } else if even_digits(*val) {
            let values = split_number(*val);
            vec_mutated.push(values.0);
            vec_mutated.push(values.1);
        } else {
            let new = *val*2024;
            vec_mutated.push(new);
        }
    }
    vec_mutated
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() 
    {
        let vector: Vec<i128> = vec![125,17];
        let vector_blinked = blink(&vector);
        assert_eq!(vector_blinked, vec![253000, 1, 7]);
        let vector_blinked = blink(&vector_blinked);
        assert_eq!(vector_blinked, vec![253, 0, 2024,14168]);

    }

    #[test]
    fn test2() 
    {
        let part1 = part1(TESTINPUT);
        assert_eq!(part1, 55312);
    }

    #[test]
    fn test3() 
    {
        let stones = vec![0];
        let stone_length = calculate_stone_length(&stones, 40);

        assert_eq!(stone_length, 55312);
    }
}
