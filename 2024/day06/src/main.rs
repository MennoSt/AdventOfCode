use lib::filereader;
use lib::grid::Grid;


#[derive(PartialEq, Eq, PartialOrd, Ord, Debug,Clone)]
struct Coordinate {
    x: i32,
    y: i32,
}

#[derive(PartialEq)]
#[derive(Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

fn main()
{
    let grid= parse_data("../input/day06");
    let part1 = iterate(grid);
    println!("{}",part1);
    assert_eq!(part1, 4454);
    // assert_eq!(part2, 1933);
}

fn iterate(grid: Grid) -> usize{
    let mut visited = Vec::new();
    let mut direction = Direction::Up;
    let start_pos = start_pos(&grid);
    let mut current_pos = start_pos.clone();
    visited.push(start_pos.clone());
    let mut vec_coord:Vec<Coordinate> = Vec::new();
    
    while (current_pos.x < (grid._width()-1) as i32)&& current_pos.x >= 0 && 
        (current_pos.y < (grid._height()-1) as i32) && current_pos.y >= 0 
        {
        let next_elem;
        if direction == Direction::Up {
            next_elem = grid._elem(current_pos.x, current_pos.y-1);
        } else if direction == Direction::Right {
            next_elem = grid._elem(current_pos.x+1, current_pos.y);
        } else if direction == Direction::Down {
            next_elem = grid._elem(current_pos.x, current_pos.y+1);
        } else {
            next_elem = grid._elem(current_pos.x-1, current_pos.y);
        }

        if next_elem == "#" {
            if direction == Direction::Up {
                direction = Direction::Right;
            } else if direction == Direction::Right {
                direction = Direction::Down;
            } else if direction == Direction::Down {
                direction = Direction::Left;
            } else if direction == Direction::Left {
                direction = Direction::Up;
            }
        }
        if direction == Direction::Up {
            current_pos.y = current_pos.y-1;
        } else if direction == Direction::Right {
            current_pos.x = current_pos.x+1;
        } else if direction == Direction::Down {
            current_pos.y = current_pos.y+1;
        } else if direction == Direction::Left {
            current_pos.x = current_pos.x-1;
        }
        vec_coord.push(current_pos.clone());
    }
    vec_coord.sort();
    vec_coord.dedup();
    vec_coord.len()
}

fn start_pos(grid: &Grid) -> Coordinate {
    for i in 0..grid._width() as i32 {
        for j in 0..grid._height() as i32 {
            if grid._elem(i, j) == "^"
            {
                return Coordinate {x:i , y: j};
            }
        }
    }

    Coordinate {x:0, y:0}
}

fn parse_data(contents: &str)-> Grid
{
    let contents = filereader::_input(&contents);
    let grid = read_into_grid(&contents);
    grid
}

fn read_into_grid(contents: &str) -> Grid {
    let mut contents_vector: Vec<Vec<String>> = Vec::new();
    for line in contents.lines() {
        let test:Vec<char> = line.chars().collect();
        let strings = test
        .iter()
        .map(|c| String::from(c.to_string()))
        .collect::<Vec<String>>();
        contents_vector.push(strings);
    }
    let grid = Grid{grid_vec:contents_vector};
    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let grid= parse_data("test1");
        let answer = iterate(grid);
        assert_eq!(answer,41);
    }
}