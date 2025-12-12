use lib::filereader;
use lib::utils;
use std::time::Instant;

static INPUT: &str = "../input/day07";

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

fn p1(input: &str) -> i128 {
    let grid_init = filereader::_input_into_grid(input);
    let mut grid = grid_init.clone();
    let splits = iterate_manifold(&mut grid);
    splits as i128
}

fn p2(input: &str) -> i128 {
    let mut grid = filereader::_input_into_grid(input);
    let mut count_grid = grid._create_visiter_grid();
    iterate_manifold(&mut grid);

    for y in 0..grid._height() as i32 {
        for x in 0..grid._width() as i32 {
            let current_elem = grid._elem(x, y);
            let up_elem = grid._up(x, y);
            let x = x as i128;
            let y = y as i128;

            if current_elem == "^" && up_elem == "|" {
                let count = count_grid._elem(x, y - 1);

                let countnew = count_grid._elem(x + 1, y);
                let new_val = count + countnew;
                count_grid._set(x + 1, y, new_val);

                let countnew = count_grid._elem(x - 1, y);
                let new_val = count + countnew;
                count_grid._set(x - 1, y, new_val);
            }
            if up_elem == "|" {
                let count = count_grid._elem(x, y - 1);
                let countnew = count_grid._elem(x, y);
                let new_val = count + countnew;
                count_grid._set(x, y, new_val);
            }

            if up_elem == "S" {
                count_grid._set(x, y, 1);
            }
        }
    }

    let vectors = count_grid.get_vec();
    let sum: i128 = vectors.last().unwrap().iter().sum();
    sum
}

fn main() {
    let start = Instant::now();

    utils::answer((p1(INPUT), 1541), (p2(INPUT), 80158285728929));

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
