use lib::filereader;
use lib::utils;
use lib::utils::*;
use std::time::Instant;
use lib::grid::*;
use std::collections::HashSet;
use std::collections::VecDeque;
use regex::Regex;

static INPUT: &str = "../input/day18";

#[derive(Clone)]
struct SearchCollection {
    grid: Grid,
    score_grid: Gridi32,
    visited_nodes: VecDeque::<Coordinate>,
    target: Coordinate,
}

fn parse_data (input:&str) -> Vec<Coordinate> {
    let content = filereader::_input(input);
    let mut coordinates = Vec::new();

        let re = Regex::new(r"\d+").unwrap();

        for line in content.lines() {
            let line_numbers:Vec<i32> = re
            .find_iter(line)
            .filter_map(|mat| mat.as_str().parse::<i32>().ok())
            .collect();
    
            coordinates.push(Coordinate{x:line_numbers[0], y:line_numbers[1]});
        }

    coordinates
}

fn iterate_grid(collection: &mut SearchCollection) {

    let mut pos = Coordinate{x:0,y:0};
    let mut score = 0;

    next_it(collection, &mut pos, &mut score);

}
fn next_it(collection: &mut SearchCollection, pos: &mut Coordinate, score: &mut i32) {

    let current_pos = pos.clone();
    let x = pos.x.clone();
    let y = pos.y.clone();

    let movements = vec![Coordinate{x:x-1,y:y}, 
                    Coordinate{x:x+1,y:y},
                    Coordinate{x:x,y:y-1}, 
                    Coordinate{x:x,y:y+1}];

    for movement in movements {
        let grid_elem = collection.grid._elem(movement.x, movement.y);

        if grid_elem == "." && !collection.visited_nodes.contains(&current_pos) && 
            collection.grid._is_within_grid(movement.x, movement.y) {
            
            *pos = movement.clone();
            *score += 1;
            collection.visited_nodes.push_back(Coordinate { x: x, y: y });

            let score_elem = collection.score_grid._elem(pos.x, pos.y);
            if *score < score_elem || score_elem == 0 {
                collection.score_grid._set(pos.x, pos.y, *score);
                if *pos !=  collection.target {
                    next_it(collection, pos, score);
                }
            }
            collection.visited_nodes.pop_back();

            *pos = current_pos.clone();
            *score -=  1;
        }
    }
}

fn part1 (input:&str, grid_size:usize, n_bytes:usize) -> i32 {
    let coordinates = parse_data(input);
    let grid = initiate_grid(coordinates, grid_size, n_bytes);
    let target = (grid_size-1) as i32;
    let score_grid = Gridi32 { grid_vec: vec![vec![0; grid._height()]; grid._width()]};
    let mut collection = SearchCollection{grid:grid, 
        score_grid:score_grid, visited_nodes:VecDeque::new(), target:Coordinate{x:target,y:target}};

    
    iterate_grid(&mut collection);

    collection.score_grid._elem(target, target)
}

fn main() {
    let start = Instant::now();

    let part1 = part1(INPUT, 71, 1024);
    println!("{}",part1);
    // utils::answer((solution.0, 134588),(solution.1, 631));

    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
}

fn initiate_grid(coordinates: Vec<Coordinate>, size:usize, bytes:usize) -> Grid {
    let rows = size;
    let cols = size;
    let mut grid = Grid { grid_vec: vec![vec![".".to_string(); rows]; cols]};
    
    let n_bytes = bytes;
    let mut it = 0;
    for c in coordinates {
        if it < n_bytes {
            grid._set_str(c.x, c.y,"#".to_string());
        }
        it += 1;
    }

    grid
}

#[cfg(test)]
mod tests {
    static TESTINPUT: &str = "test1";
    use super::*;

    #[test]
    fn test1() {
        let part1 = part1(TESTINPUT, 7, 12);
        assert_eq!(part1, 22);
    }
    
}