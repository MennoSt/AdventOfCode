use lib::filereader;
use lib::utils;
use lib::utils::Coordinate;
use std::time::Instant;
use regex::Regex;
use lib::grid::Grid;
use lib::grid::Gridi32;
use std::thread;
use std::time::Duration;

static INPUT: &str = "../input/day15";

fn parse_data(file:&str) -> (Grid, Vec<String>) 
{
    let content = filereader::_input(file);
    let mut grid_vec = Vec::new();
    let mut sequence = Vec::new();
    let mut first_part = true;

    for line in content.lines() {
        if line.is_empty() {
            first_part = false;
        }

        let test:Vec<char> = line.chars().collect();
        let strings = test
        .iter()
        .map(|c| String::from(c.to_string()))
        .collect::<Vec<String>>();
        if first_part {
            grid_vec.push(strings);
        } else {
            sequence.extend(strings);
        }
    }

    (Grid {grid_vec:grid_vec}, sequence)
}

fn start_pos(grid:&Grid) -> Coordinate {
    for i in 0..grid._width() as i32{
        for j in 0..grid._height() as i32 {
            if grid._elem(i,j) == "@" {
                return Coordinate {x:i, y:j};
            }
        }
    }
    Coordinate{x:0,y:0}
}
fn move_robot(robot_pos:&mut Coordinate, grid:&mut Grid, movement:&str) {
    let mut depth = 0;
    let mut it_pos = robot_pos.clone();

    change_pos(movement, &mut it_pos);
    try_move(robot_pos, &mut it_pos, movement, &mut depth, grid)
}

fn change_pos(movement: &str, it_pos: &mut Coordinate) {
    match movement {
        ">" => it_pos.x += 1,
        "<" => it_pos.x -= 1,
        "^" => it_pos.y -= 1,
        "v" => it_pos.y += 1,
        _ => println!("Unknown command"),
    }
}
fn try_move(robot_pos:&mut Coordinate, it_pos:&mut Coordinate, movement:&str, depth:&mut i32,  grid:&mut Grid) {
    *depth += 1;

    if grid._elem(it_pos.x, it_pos.y) == "." && *depth == 1 {
        update_robot_pos(robot_pos, movement, grid);
    } else if grid._elem(it_pos.x, it_pos.y) == "O" {
        change_pos(movement, it_pos);
        try_move(robot_pos, it_pos, movement, depth, grid);
    } else if grid._elem(it_pos.x, it_pos.y) == "." {
        update_robot_pos(robot_pos, movement, grid);
        grid._set_str(it_pos.x, it_pos.y,"O".to_string());
    } 
}

fn update_robot_pos(robot_pos: &mut Coordinate, movement: &str, grid: &mut Grid) {
    grid._set_str(robot_pos.x, robot_pos.y,".".to_string());
    change_pos(movement, robot_pos);
    grid._set_str(robot_pos.x, robot_pos.y,"@".to_string());
}

fn part1(input:&str) -> i32 {
    let (mut grid, moves) = parse_data(input);
    let mut rp = start_pos(&grid);

    for m in moves {
        move_robot(&mut rp, &mut grid, m.as_str());
    }

    sum_gps_boxes(grid)
}

fn sum_gps_boxes(grid: Grid) -> i32 {
    let mut sum_gps_boxes = 0;

    for i in 0..grid._width() as i32 {
        for j in 0..grid._height() as i32 {
           if grid._elem(i, j) == "O" {
            sum_gps_boxes += 100 * j + i;
           }
        }
    }
    sum_gps_boxes
}

fn main() {
    let start = Instant::now();

    let part1 = part1(INPUT);
    println!("{}", part1);
    // utils::answer((part1(INPUT, Area {width:101, height:103}), 232589280),(part2(INPUT, Area {width:101, height:103}), 7596));

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
        assert_eq!(part1, 2028);
    }
 
     #[test]
    fn test2() {
        let part1 = part1("test2");
        assert_eq!(part1, 10092);
    }
    
}