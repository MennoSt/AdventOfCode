use lib::filereader;
use lib::utils;
use lib::utils::Coordinate;
use std::time::Instant;

static INPUT: &str = "../input/day07";

fn p1(input: &str) -> i32 {
    let grid_init = filereader::_input_into_grid(input);
    let mut grid = grid_init.clone();
    let splits = iterate_manifold(&mut grid);

    grid._print();
    splits
}

fn iterate_manifold(grid: &mut lib::grid::Grid) -> i32 {
    let mut splits = 0;
    for y in 0..grid._height() as i32 {
        for x in 0..grid._width() as i32 {
            let current_elem = grid._elem(x, y);
            let up_elem = grid._up(x, y);

            if current_elem == "^" && up_elem == "|" {
                grid._set_str(x - 1, y, "|");
                grid._set_str(x + 1, y, "|");
                splits += 1;
            }
            if current_elem == "." && up_elem == "|" || up_elem == "S" {
                grid._set_str(x, y, "|");
            }
        }
    }
    splits
}

fn p2(input: &str) -> i128 {
    let grid_init = filereader::_input_into_grid(input);
    let mut grid = grid_init.clone();
    let splits = iterate_manifold(&mut grid);
    let mut count_grid = grid_init._create_visiter_grid();

    for y in 0..grid._height() as i32 {
        for x in 0..grid._width() as i32 {
            let current_elem = grid._elem(x, y);
            let up_elem = grid._up(x, y);
            let x = x as i128;
            let y = y as i128;

            if current_elem == "^" && up_elem == "|" {
                let count = count_grid._elem(x, y-1);

                let countnew = count_grid._elem(x+1,y);
                let new_val = count+countnew;
                count_grid._set(x+1, y, new_val);


                let countnew = count_grid._elem(x-1,y);
                let new_val = count+countnew;
                count_grid._set(x-1, y, new_val);
            }
            if up_elem == "|" {
                let count = count_grid._elem(x, y-1);
                let countnew = count_grid._elem(x,y);
                let new_val = count + countnew;
                count_grid._set(x, y, new_val);
            }
            
            if up_elem == "S"
            {
                count_grid._set(x, y, 1);
            }
        }
    }

    count_grid._print();
    grid._print();

    let vectors = count_grid.get_vec();
    let sum:i128 = vectors.last().unwrap().iter().sum();
    sum
}

fn main() {
    let start = Instant::now();

    let part1 = p1(INPUT);
    println!("{}", part1);
    assert_eq!(part1, 1541);
    let part2 = p2(INPUT);
    println!("{}", part2);

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
        assert_eq!(sum, 21);
    }
    #[test]
    fn test2() {
        let sum = p2(INPUT_EXAMPLE);
        assert_eq!(sum, 40);
    }
}
