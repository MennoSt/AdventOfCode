use lib::filereader;
use lib::utils;
use lib::utils::*;
use std::time::Instant;
use lib::grid::*;
use std::collections::HashSet;

static INPUT: &str = "../input/day16";

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
struct Movement {
    coordinate: Coordinate,
    direction: Direction,
}

fn find_single_elem(grid:&Grid, elem:&str) -> Coordinate {
    for i in 0..grid._width() as i32{
        for j in 0..grid._height() as i32 {
            if grid._elem(i,j) == elem {
                return Coordinate {x:i, y:j};
            }
        }
    }
    Coordinate{x:0,y:0}
}

fn iterate_maze(grid:&mut Grid, score_grid:&mut Gridi32, shortest_paths:&mut HashSet<Coordinate>, lowest_score:i32) {
    let start_pos = find_single_elem(grid, "S");
    let mut score = 0;
    let pos = start_pos.clone();
    grid._set_str(start_pos.x, start_pos.y ,"x".to_string());

    next_it(grid, score_grid, Movement{coordinate:pos, direction:Direction::Right}, &mut score, shortest_paths, lowest_score);

    grid._set_str(start_pos.x, start_pos.y ,"S".to_string());
}

fn next_it(grid: &mut Grid, score_grid: &mut Gridi32, mut pos:Movement, score: &mut i32, shortest_paths:&mut HashSet<Coordinate>, lowest_score:i32) {

    let x = pos.coordinate.x.clone();
    let y = pos.coordinate.y.clone();

    let movements = vec![Movement{coordinate:Coordinate{x:x-1,y:y}, direction:Direction::Left}, 
        Movement{coordinate:Coordinate{x:x+1,y:y}, direction:Direction::Right},
        Movement{coordinate:Coordinate{x:x,y:y-1}, direction:Direction::Up}, 
        Movement{coordinate:Coordinate{x:x,y:y+1},direction:Direction::Down}];

    for movement in movements {
        let grid_elem = grid._elem(movement.coordinate.x, movement.coordinate.y);

        if grid_elem == "." || grid_elem == "E"{
                
            let current_pos = pos.clone();
            let score_increase= calculate_score_increase(&current_pos.direction, &movement.direction);
            
            pos = movement.clone();
            *score += score_increase;
            grid._set_str(x, y ,"x".to_string());
            
            let score_elem = score_grid._elem(pos.coordinate.x, pos.coordinate.y);
            if *score <= score_elem || score_elem == 0 || is_cross_or_t(grid, &pos.coordinate) {
                score_grid._set(pos.coordinate.x, pos.coordinate.y, *score);
                if grid_elem != "E"{
                    next_it(grid, score_grid, pos, score, shortest_paths, lowest_score);
                }
            }
            
            update_shortest_path(grid, score, shortest_paths, lowest_score, grid_elem);

            pos = current_pos;
            *score -= score_increase;
            grid._set_str(x, y, ".".to_string());
        }
    }
}

fn update_shortest_path(grid: &mut Grid, score: &mut i32, lowest_score_coord: &mut HashSet<Coordinate>, lowest_score: i32, grid_elem: String) {
    if grid_elem == "E" && *score == lowest_score {
        for i in 0..grid._width() as i32{
            for j in 0..grid._height() as i32 {
                if grid._elem(i, j) ==  "x" || grid._elem(i, j) ==  "E"{
                    lowest_score_coord.insert(Coordinate { x: i , y: j });
                }
            }
        }
    }
}

fn calculate_score_increase(first: &Direction, second: &Direction) -> i32 {
    if first == second {
        1
    } else {
        1001
    }
}

fn is_cross_or_t(grid:&Grid, coordinate:&Coordinate) -> bool {
    if n_directions(grid, coordinate) > 2 {
        true
    } else {
        false
    }
}

fn n_directions(grid: &Grid, coordinate: &Coordinate) -> i32 {
    let x=coordinate.x;
    let y = coordinate.y;
    let vec_elems = vec![grid._elem(x+1,y), grid._elem(x-1,y), grid._elem(x,y+1), grid._elem(x,y-1)];

    let mut directions = 0;
    for elem in vec_elems {
        if elem == "." || elem == "x"
        {
            directions += 1;
        }
    }
    directions
}

fn solve(input:&str, lowest_score:i32) -> (i32,i32) {
    let mut grid = filereader::_input_into_grid(input);
    let end = find_single_elem(&grid,"E");
    let mut lowest_score_coord = HashSet::new();
    let rows = grid._width(); 
    let cols = grid._height();
    let mut score_grid = Gridi32 { grid_vec: vec![vec![0; rows]; cols]};

    iterate_maze(&mut grid, &mut score_grid, &mut lowest_score_coord, lowest_score);

    (score_grid._elem(end.x,end.y), lowest_score_coord.len() as i32)
}

fn main() {
    let start = Instant::now();

    let solution = solve(INPUT,134588);
    utils::answer((solution.0, 134588),(solution.1, 631));

    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
}

#[cfg(test)]
mod tests {
    static TESTINPUT: &str = "test1";
    use super::*;

    #[test]
    fn test1() {
        let part1 = solve(TESTINPUT, 0);
        assert_eq!(part1.0, 7036);
    }
    
    #[test]
    fn test2() {
        let part1= solve("test2", 0);
        assert_eq!(part1.0, 11048);
    }

    #[test]
    fn test3() {
        let part2 = solve(TESTINPUT,7036);
        assert_eq!(part2.1, 45);
    }

    #[test]
    fn test4() {
        let part2 = solve("test2",11048);
        assert_eq!(part2.1, 64);
    }
}