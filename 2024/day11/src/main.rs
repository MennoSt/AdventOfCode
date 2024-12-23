use lib::filereader;
use lib::utils;
use std::time::Instant;

static INPUT: &str = "../input/day11";

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
struct Stone {
    value: i128,
    count: i128,
}

fn main() {
    let start = Instant::now();
    
    utils::answer((part1(INPUT), 193899),(part2(INPUT), 229682160383225));

    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
}

fn part1(file:&str) -> usize
{
    let stones_val = parse_data(file);
    let mut stones: Vec<Stone> = Vec::new();
    for val in stones_val {
        stones.push(Stone{value: val, count: 1});
    }
    
    calculate_stone_length(&stones, 25)
}

fn part2(file:&str) -> usize
{
    let stones_val = parse_data(file);
    let mut stones: Vec<Stone> = Vec::new();
    for val in stones_val {
        stones.push(Stone{value: val, count: 1});
    }

    calculate_stone_length(&stones, 75)
}

fn calculate_stone_length(vector:&Vec<Stone>, blinks:i32) -> usize
{   
    let mut it =0;
    let mut vec_mutated = vector.clone();
    while it < blinks {
        vec_mutated = blink(&vec_mutated);
        it+=1;
    }

    let mut length = 0;
    for stone in vec_mutated {
        length += stone.count;
    }
    length as usize
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
fn is_stone_present(new_stone: &Stone, stones: &mut Vec<Stone>) -> bool
{
    for stone in stones {
        if stone.value == new_stone.value {
            stone.count += new_stone.count;
            return true;
        }
    }
    false
}

fn blink(vec_stones:&Vec<Stone>) -> Vec<Stone>
{
    let mut vec_mutated:Vec<Stone> = Vec::new();
    for stone in vec_stones {
        if stone.value == 0 {
            update_stones(&mut vec_mutated, Stone{value:1,count:stone.count});
        } else if even_digits(stone.value) {
            let values = split_number(stone.value);
            update_stones(&mut vec_mutated, Stone{value:values.0,count:stone.count});
            update_stones(&mut vec_mutated, Stone{value:values.1,count:stone.count});
        } else {
            let new = stone.value*2024;
            update_stones(&mut vec_mutated, Stone{value:new,count:stone.count});
        }
    }

    vec_mutated
}

fn update_stones(vec_mutated: &mut Vec<Stone>, new_stone: Stone) {
    if !is_stone_present(&new_stone, vec_mutated) {
        vec_mutated.push(Stone{value:new_stone.value, count:new_stone.count});
    }
}

#[cfg(test)]
mod tests {
    static TESTINPUT: &str = "test1";
    use super::*;

    #[test]
    fn test1() 
    {
        let vector: Vec<Stone> = vec![Stone{value:125,count:1},Stone{value:17, count:1}];
        let vector_blinked = blink(&vector);
        assert_eq!(vector_blinked, vec![Stone{value:253000, count:1},Stone{value:1, count:1}, Stone{value:7,count:1}]);
        
        let vector_blinked = blink(&vector_blinked);
        assert_eq!(vector_blinked, vec![Stone{value:253, count:1},Stone{value:0, count:1}, 
            Stone{value:2024,count:1}, Stone{value:14168,count:1}]);
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
        let stones = vec![Stone{value:125, count:1},Stone{value: 17, count:1}];
        let length = calculate_stone_length(&stones,6);

        assert_eq!(length, 22);
    }
}
