use lib::filereader;
use lib::utils;
use lib::utils::*;
use lib::grid::Gridi32;
use std::time::Instant;

static INPUT: &str = "../input/day10";

fn part1(input: &str) -> i32 {
    let mut grid = filereader::_input_into_grid_i32(input);

    let rows = grid._width(); 
    let cols = grid._height();
    let mut visited_grid = Gridi32 { grid_vec: vec![vec![0; rows]; cols]};
    let mut trailheads = 0;
    let mut depth = 0;
    for i in 0..grid._width() as i32 {
        for j in 0..grid._height() as i32 {
            if grid._elem(i,j) == 0 {
                find_trailheads(i, j, &mut grid, &mut visited_grid, &mut trailheads, &mut depth,false);
            }
            grid = filereader::_input_into_grid_i32(input);
        }
    }
    trailheads
}

fn part2(input: &str) -> i32 {
    let mut grid = filereader::_input_into_grid_i32(input);

    let rows = grid._width(); 
    let cols = grid._height();
    let mut visited_grid = Gridi32 { grid_vec: vec![vec![0; rows]; cols]};

    let mut trailheads = 0;
    let mut depth = 0;
    for i in 0..grid._width() as i32 {
        for j in 0..grid._height() as i32 {
            if grid._elem(i,j) == 0 {
                find_trailheads(i, j, &mut grid, &mut visited_grid, &mut trailheads, &mut depth,true);
            }
            grid = filereader::_input_into_grid_i32(input);
        }
    }
    trailheads
}

fn find_trailheads(x:i32, y:i32, grid: &mut Gridi32, visited_grid: &mut Gridi32, trailheads: &mut i32, depth: &mut i32, unique: bool) {
    let current_elem = grid._elem(x,y);
    visited_grid._set(x, y,1);
    *depth+=1;
    let directions = vec![Coordinate{x:x-1,y:y}, 
                                            Coordinate{x:x+1,y:y}, 
                                            Coordinate{x:x,y:y-1}, 
                                            Coordinate{x:x,y:y+1}];

    for dir in directions {
        if is_within_grid(grid, &dir) {
            if grid._elem(dir.x,dir.y) == current_elem+1 && visited_grid._elem(dir.x,dir.y) != 1 {
                if *depth < 10 {
                    find_trailheads(dir.x, dir.y, grid, visited_grid, trailheads, depth, unique);
                }
            }
            if *depth == 10 {
                *trailheads +=1 ;

                if !unique {
                    grid._set(x,y,-1);
                }
                break;
            }
        }
    }
    visited_grid._set(x, y,0);
    *depth -= 1;
}

fn is_within_grid(grid: &mut Gridi32, dir: &Coordinate) -> bool {
    dir.x < grid._width() as i32 && dir.x >= 0 && dir.y < grid._height() as i32 && dir.y >= 0
}

fn main() {
    let start = Instant::now();
    
    utils::answer((part1(INPUT), 794),(part2(INPUT), 1706));

    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
}

#[cfg(test)]
mod tests {

    static TESTINPUT: &str = "test1";
    use super::*;

    #[test]
    fn test1_p1() {
        let part1 = part1(TESTINPUT);
        assert_eq!(part1, 36);
    }

    #[test]
    fn test1_p2() {
        let part1 = part2(TESTINPUT);
        assert_eq!(part1, 81);
    }

    #[test]
    fn test2() {
        let answer = part1("test2");
        assert_eq!(answer, 1);
    }

    #[test]
    fn test3() {
        let answer = part1("test3");
        assert_eq!(answer, 3);
    }
    
    #[test]
    fn test4() {
        let answer = part1("test4");
        assert_eq!(answer, 4);
    }
    
    #[test]
    fn test5() {
        let answer = part1("test5");
        assert_eq!(answer, 1);
    }
}
