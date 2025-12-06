use lib::{filereader};
use std::time::Instant;
use lib::grid::Grid;

static INPUT: &str = "../input/day04";

fn parse_data(input: &str) -> Grid {

    let grid = filereader::_input_into_grid(input);
    grid._print();
    grid
}


fn main() {
    
    let start = Instant::now();

    parse_data(INPUT);
    // let part1 = p1(INPUT);
    // let part2 = p2(INPUT);

    // assert_eq!(part1, 17087);
    // assert_eq!(part2, 169019504359949);
    // println!("{}", part1);
    // println!("{}", part2);
    
    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT_EXAMPLE: &str = "example";

    #[test]
    fn test6() {
        let sum = p1(INPUT_EXAMPLE);
        assert_eq!(sum, 357);
    }
    #[test]
    fn test7() {
        let sum = p2(INPUT_EXAMPLE);
        assert_eq!(sum, 3121910778619);
    }
}
