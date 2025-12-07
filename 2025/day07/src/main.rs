use lib::filereader;
use lib::utils;
use lib::utils::Coordinate;
use std::time::Instant;

static INPUT: &str = "../input/day07";

fn p1(input: &str) -> i32 {
    let grid_init = filereader::_input_into_grid(input);
    let mut grid = grid_init.clone();
    for y in 0..grid._height() as i32 {
        for x in 0..grid._width() as i32 {
            let current_elem = grid._elem(x, y);

            if current_elem == "^" && grid._up(x, y) == "|" {
                grid._set_str(x - 1, y, "|");
                grid._set_str(x + 1, y, "|");
            }
            if current_elem == "." && grid._up(x, y) == "|" || grid._up(x,y) == "S" {
                grid._set_str(x, y, "|");
            }
        }
    }

    grid._print();

    let mut sum = 0;

    sum
}

fn main() {
    let start = Instant::now();

    let part1 = p1(INPUT);
    // let part2 = p2(INPUT);

    // utils::answer((part1,1363),(part2, 8184));

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
