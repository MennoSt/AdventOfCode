use lib::filereader;
use lib::grid::Grid;
use std::time::Instant;

static INPUT: &str = "../input/day04";

fn parse_data(input: &str) -> Grid {
    let grid = filereader::_input_into_grid(input);
    grid._print();
    grid
}

fn p1(input: &str) -> i32 {
    let grid = parse_data(input);
    let mut sum = 0;

    for i in 0..grid._width() as i32 {
        for j in 0..grid._height() as i32 {
            let surroundings = grid._get_surroundings(i, j);
            let count = surroundings.iter().filter(|s| s == &"@").count();
            if count < 4 && grid._elem(i, j) == "@" {
                sum += 1;
            }
        }
    }
    sum
}

fn p2(input: &str) -> i32 {
    let grid = parse_data(input);
    let mut grid_mut = grid.clone();
    let mut sum = 0;

    loop {
        let mut removed_rolls = 0;
        for i in 0..grid._width() as i32 {
            for j in 0..grid._height() as i32 {
                let surroundings = grid_mut._get_surroundings(i, j);
                let count = surroundings.iter().filter(|s| s == &"@").count();
                if count < 4 && grid_mut._elem(i, j) == "@" {
                    removed_rolls += 1;
                    grid_mut._set_str(i, j, "x".to_string());
                }
            }
        }
        sum += removed_rolls;
        grid_mut._print();
        if removed_rolls == 0
        {
            break;
        }
    }

    sum
}
fn main() {
    let start = Instant::now();

    let part1 = p1(INPUT);
    let part2 = p2(INPUT);

    assert_eq!(part1, 1363);
    assert_eq!(part2, 8184);
    println!("{}", part1);
    println!("{}", part2);

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

    #[test]
    fn test2() {
        let sum = p2(INPUT_EXAMPLE);
        assert_eq!(sum, 43);
    }
}
