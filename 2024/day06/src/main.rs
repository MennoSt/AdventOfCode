use lib::filereader;
use lib::grid::Grid;
use lib::utils::*;

fn part1(grid: Grid) -> usize{
    
    let mut direction = Direction::Up;
    let start_pos = start_pos(&grid);
    let mut current_pos = start_pos.clone();
    let mut vec_coord:Vec<Coordinate> = Vec::new();
    
    vec_coord.push(start_pos.clone());

    while (current_pos.x < (grid._width()-1) as i32)&& current_pos.x > 0 && 
        (current_pos.y < (grid._height()-1) as i32) && current_pos.y > 0 
        {

        if next_element(&direction, &grid, &current_pos) == "#" {
            change_direction(&mut direction, &grid, &current_pos);
        }

        update_current_position(&direction, &mut current_pos);

        if !vec_coord.contains(&current_pos) {
            vec_coord.push(current_pos.clone());
        }
    }
    vec_coord.len()
}

fn part2(grid: Grid) -> usize{
  
    let mut number_of_obstructions = 0;

    let mut direction = Direction::Up;
    let start_pos = start_pos(&grid);
    let mut current_pos = start_pos.clone();
    let mut vec_coord:Vec<(Coordinate,Direction)> = Vec::new();
    vec_coord.push((start_pos.clone(), Direction::Up));

    let mut grid_mut = grid.clone();
    
    for i in 0..grid_mut._width() {
        for j in 0..grid_mut._height(){
            grid_mut._set_str(i as i32, j as i32, "#".to_string());
            if is_loop(&mut current_pos, &grid_mut, &mut direction, &mut vec_coord) {
                number_of_obstructions +=1;
            }
            current_pos = start_pos.clone();
            vec_coord.clear();
            direction = Direction::Up;
            grid_mut = grid.clone();
        }
    }
    number_of_obstructions
}

fn is_loop(current_pos: &mut Coordinate, grid_mut: &Grid, direction: &mut Direction, 
    vec_coord: &mut Vec<(Coordinate, Direction)>) -> bool {
    let mut is_loop = false;
    while (current_pos.x < (grid_mut._width()-1) as i32)&& current_pos.x >= 0 && 
        (current_pos.y < (grid_mut._height()-1) as i32) && current_pos.y >= 0 
        {
        if next_element(&direction, &grid_mut, &current_pos) == "#" {
            change_direction(direction, &grid_mut,&current_pos);
        }
        update_current_position(&direction, current_pos);
        
        let tuple = (current_pos.clone(),direction.clone());
        if vec_coord.contains(&tuple) {
            is_loop = true;
            break;
        }
        vec_coord.push((current_pos.clone(), direction.clone()));
    }
    is_loop
}

fn change_direction(direction: &mut Direction, grid: &Grid, current_pos: &Coordinate) {
    if *direction == Direction::Up {
        *direction = Direction::Right;
        if grid._elem(current_pos.x+1, current_pos.y)=="#"
        {
            change_direction(direction, grid, current_pos);
        }
    } else if *direction == Direction::Right {
        *direction = Direction::Down;
        if grid._elem(current_pos.x, current_pos.y+1)=="#"
        {
            change_direction(direction, grid, current_pos);
        }
    } else if *direction == Direction::Down {
        *direction = Direction::Left;
        if grid._elem(current_pos.x-1, current_pos.y)=="#"
        {
            change_direction(direction, grid, current_pos);
        }
    } else {
        *direction = Direction::Up;
        if grid._elem(current_pos.x, current_pos.y-1)=="#"
        {
            change_direction(direction, grid, current_pos);
        }
    }
}


fn update_current_position(direction: &Direction, current_pos: &mut Coordinate) {
    if *direction == Direction::Up {
        current_pos.y = current_pos.y-1;
    } else if *direction == Direction::Right {
        current_pos.x = current_pos.x+1;
    } else if *direction == Direction::Down {
        current_pos.y = current_pos.y+1;
    } else {
        current_pos.x = current_pos.x-1;
    }
}

fn next_element(direction: &Direction, grid: &Grid, current_pos: &Coordinate) -> String {
    let next_elem;
    if *direction == Direction::Up {
        next_elem = grid._elem(current_pos.x, current_pos.y-1);
    } else if *direction == Direction::Right {
        next_elem = grid._elem(current_pos.x+1, current_pos.y);
    } else if *direction == Direction::Down {
        next_elem = grid._elem(current_pos.x, current_pos.y+1);
    } else {
        next_elem = grid._elem(current_pos.x-1, current_pos.y);
    }
    next_elem
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

fn main()
{
    let grid= filereader::_input_into_grid("../input/day06");
    let part1 = part1(grid.clone());
    println!("{}",part1);
    assert_eq!(part1, 4454);

    let part2 = part2(grid);
    println!("{}",part2);
    assert_eq!(part2, 1503);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn isloop_test(str:&str) {
        let grid= filereader::_input_into_grid(str);
        let start_pos = start_pos(&grid);
        let mut direction = Direction::Up;
        let mut current_pos = start_pos.clone();
        let mut vec_coord:Vec<(Coordinate,Direction)> = Vec::new();
        vec_coord.push((start_pos.clone(), Direction::Up));
    
        let answer = is_loop(&mut current_pos, &grid, &mut direction, &mut vec_coord);
        assert_eq!(answer,true);
    }

    #[test]
    fn test1() {
        let grid= filereader::_input_into_grid("test1");
        let answer = part1(grid);
        assert_eq!(answer,41);
    }

    #[test]
    fn test2() {
        isloop_test("test2");
        isloop_test("test3");
        isloop_test("test4");
        isloop_test("test5");
    }

    #[test]
    fn test3() {
        let grid= filereader::_input_into_grid("test1");
        let answer = part2(grid);
        assert_eq!(answer,6);
    }
}