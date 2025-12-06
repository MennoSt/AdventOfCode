use lib::filereader;
use lib::grid::Grid;
use std::time::Instant;

static INPUT: &str = "../input/day04";

fn parse_data(input: &str) -> Grid {
    let grid = filereader::_input_into_grid(input);
    grid._print();
    grid
}

fn p1(input: &str) -> i32
{
    let grid = parse_data(input);
    let mut sum = 0;

    for i in 0..grid._width() as i32 {
        for j in 0..grid._height() as i32 {

            let surroundings = grid._get_surroundings(i, j);
            let count = surroundings.iter().filter(|s| s== &"@").count();
            if count < 4 && grid._elem(i, j) == "@"
            {
                sum+=1;
                println!("{} {}", i,j,);
            }
        }
    }
    sum
}

fn main() {
    let start = Instant::now();



    let part1 = p1(INPUT);
    // let part2 = p2(INPUT);

    // assert_eq!(part1, 17087);
    // assert_eq!(part2, 169019504359949);
    println!("{}", part1);
    // println!("{}", part2);

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
        assert_eq!(sum, 13);
    }
}
