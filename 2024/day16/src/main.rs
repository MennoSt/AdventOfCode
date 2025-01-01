use lib::filereader;
use lib::utils;
use lib::utils::*;
use std::time::Instant;
use lib::grid::*;
use std::collections::VecDeque;

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

fn iterate_maze(grid:&mut Grid, score_grid:&mut Gridi32) {

    let mut pos = find_single_elem(grid, "S");
    let mut dir = Direction::Right;


    let mut score = 0;

    next_it(grid, score_grid, Movement{coordinate:pos, direction:dir}, &mut score);

    score_grid._print();
    score_grid._print_special();
}

fn next_it(grid: &mut Grid, score_grid: &mut Gridi32, mut pos:Movement, score: &mut i32) {

    let x = pos.coordinate.x;
    let y = pos.coordinate.y;

    let movements = vec![Movement{coordinate:Coordinate{x:x-1,y:y}, direction:Direction::Left}, 
        Movement{coordinate:Coordinate{x:x+1,y:y}, direction:Direction::Right},
        Movement{coordinate:Coordinate{x:x,y:y-1}, direction:Direction::Up}, 
        Movement{coordinate:Coordinate{x:x,y:y+1},direction:Direction::Down}];

    for movement in movements {

        let grid_elem = grid._elem(movement.coordinate.x, movement.coordinate.y);
        if grid_elem == "." || grid_elem == "E"{

            let current_pos = pos.clone();
            pos = movement.clone();
            grid._set_str(x, y, "x".to_string());
            
            let score_increase= calculate_score_increase(&current_pos.direction, &movement.direction);

            *score += score_increase;
            
            //update score
            let score_elem = score_grid._elem(pos.coordinate.x, pos.coordinate.y);
            if *score <= score_elem || score_elem == 0 {
                score_grid._set(pos.coordinate.x, pos.coordinate.y, *score);
                if grid_elem != "E"{
                    next_it(grid, score_grid, pos, score);
                }
            }


            pos = current_pos;
            grid._set_str(x, y, ".".to_string());
            *score -= score_increase;
        }
    }
}

fn calculate_score_increase(first: &Direction, second: &Direction) -> i32 {
    let mut score_increase = 1;

    if *first != *second {
        score_increase += 1000;
    }

    score_increase
}

fn part1(input:&str) -> i32 {
    let mut grid = filereader::_input_into_grid(input);
    let end = find_single_elem(&grid,"E");

    let rows = grid._width(); 
    let cols = grid._height();
    let mut score_grid = Gridi32 { grid_vec: vec![vec![0; rows]; cols]};

    iterate_maze(&mut grid, &mut score_grid);

    grid._print();
    
    score_grid._print();


    score_grid._elem(end.x,end.y)
}
// 134596 too high
fn main() {
    let start = Instant::now();

    // utils::answer((part1(INPUT), 1538871),(part2(INPUT), 1543338));
    let answer = part1(INPUT);
    println!("{}",answer);
    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
}

#[cfg(test)]
mod tests {
    static TESTINPUT: &str = "test1";
    use super::*;

    #[test]
    fn test1() {
        let part1 = part1(TESTINPUT);
        assert_eq!(part1, 7036);
    }
    
    #[test]
    fn test2() {
        let part1 = part1("test2");
        assert_eq!(part1, 7036);
    }
}