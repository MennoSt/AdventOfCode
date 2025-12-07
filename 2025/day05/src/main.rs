use lib::filereader;
use lib::utils;
use std::time::Instant;

static INPUT: &str = "../input/day05";

struct IDRange
{
    start:i128,
    end:i128,   
}

struct Ingredients
{
    id_ranges:Vec<IDRange>,
    ingredient_ids : Vec<i128>,
}

fn parse_data(input: &str) -> i32 {
    let content = filereader::_input(input);
    filereader::_print_content(&content);


    let mut sum = 0;

    sum
}

fn main() {
    let start = Instant::now();
    parse_data(INPUT);
    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT_EXAMPLE: &str = "example";

    #[test]
    fn test1() {
        // let sum = p1(INPUT_EXAMPLE);
        // assert_eq!(sum, 13);
    }

}
