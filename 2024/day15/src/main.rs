use lib::filereader;
use lib::utils;
use lib::utils::Coordinate;
use std::time::Instant;
use lib::grid::Grid;

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

fn move_robot_p1(robot_pos:&mut Coordinate, grid:&mut Grid, movement:&str) {
    let mut depth = 0;
    let mut it_pos = robot_pos.clone();

    change_pos(movement, &mut it_pos);
    try_move_p1(robot_pos, &mut it_pos, movement, &mut depth, grid)
}

fn try_move_p1(robot_pos:&mut Coordinate, it_pos:&mut Coordinate, movement:&str, depth:&mut i32,  grid:&mut Grid) {
    *depth += 1;

    if grid._elem(it_pos.x, it_pos.y) == "." && *depth == 1 {
        update_robot_pos(robot_pos, movement, grid);
    } else if grid._elem(it_pos.x, it_pos.y) == "O" {
        change_pos(movement, it_pos);
        try_move_p1(robot_pos, it_pos, movement, depth, grid);
    } else if grid._elem(it_pos.x, it_pos.y) == "." {
        update_robot_pos(robot_pos, movement, grid);
        grid._set_str(it_pos.x, it_pos.y,"O".to_string());
    } 
}

fn move_robot_p2(robot_pos:&mut Coordinate, grid:&mut Grid, movement:&str) {
    let mut depth = 0;
    let mut vec_elem = Vec::new();
    let mut it_pos = robot_pos.clone();
    let mut blocked = true;
    let mut linked_boxes = Vec::new();

    change_pos(movement, &mut it_pos);
    if movement == ">" || movement == "<" {
        try_move_hor(robot_pos, &mut it_pos, movement, &mut depth, grid, &mut vec_elem);
    } else {
        let elems_to_check=vec![Coordinate{x:it_pos.x, y:it_pos.y}];
        is_blocked(&mut it_pos, movement,grid, &elems_to_check, &mut blocked, &mut linked_boxes);
        linked_boxes.sort(); // Sort the vector first
        linked_boxes.dedup();
        if !blocked {

            update_boxes(grid, movement, &mut linked_boxes);
            update_robot_pos(robot_pos, movement, grid);
            remove_brackets(grid);
        }
    }
}

fn update_boxes(grid:&mut Grid, movement:&str, linked_boxes:&mut Vec<Coordinate>) {
    if movement == "^" {
        for coor in linked_boxes {
            let elem = grid._elem(coor.x, coor.y+1);
            if elem == "[" || elem =="]"{
                grid._set_str(coor.x, coor.y, elem);
            }
        }
    }
    else if movement == "v" 
        {
        linked_boxes.sort_by(|a, b| b.y.cmp(&a.y));
        for coor in linked_boxes {
            let elem = grid._elem(coor.x, coor.y-1);
            if elem == "[" || elem =="]"{
                grid._set_str(coor.x, coor.y, elem);
            }
        }
    }    
}

fn remove_brackets(grid: &mut Grid) {
    for i in 0..grid._width() as i32{
        for j in 0..grid._height() as i32 {
            if grid._elem(i,j) == "[" && grid._elem(i+1,j) != "]" {
                grid._set_str(i, j,".".to_string());  
            }
            if grid._elem(i,j) == "]" && grid._elem(i-1,j) != "["{
                grid._set_str(i, j,".".to_string());
            }
        }
    }
}

fn is_blocked(it_pos:&mut Coordinate, movement:&str, grid:&mut Grid, coordinates: &Vec<Coordinate>, blocked:&mut bool, linked_boxes:&mut Vec<Coordinate>)
{
    let mut blocked_by_hash = false;
    if is_free(grid, coordinates) {
        *blocked = false;
    }

    let mut new_coordinates = Vec::new();

    if *blocked {
        for coor in coordinates{
            let yup = coor.y + 1;
            let ydown = coor.y - 1;
            let elem = grid._elem(coor.x, coor.y);

            if movement == "^" {
                if elem == "#" {
                    blocked_by_hash = is_blocked_by_hash(grid,Coordinate{x:coor.x,y:yup});
                }
                fill_new_coordinates(&mut new_coordinates, coor, elem, ydown);
            } else if movement=="v"{
                if elem == "#" {
                    blocked_by_hash = is_blocked_by_hash(grid,Coordinate{x:coor.x,y:ydown});
                }
                fill_new_coordinates(&mut new_coordinates, coor, elem, yup);
                }
            }
        }

   if new_coordinates.len()>0 && !blocked_by_hash {
      linked_boxes.extend(new_coordinates.clone());
      is_blocked(it_pos, movement, grid, &new_coordinates, blocked, linked_boxes);
   }
   
}

fn is_blocked_by_hash(grid: &mut Grid, coor: Coordinate) -> bool {
    
    if grid._elem(coor.x, coor.y) != "." {
        return true;
    }
    false
}

fn fill_new_coordinates(new_coordinates: &mut Vec<Coordinate>, coor: &Coordinate, elem: String, ynew:i32) {
    if elem == "[" {
        let xnew=coor.x+1;
        new_coordinates.push(Coordinate{x:coor.x, y:ynew});
        new_coordinates.push(Coordinate{x:xnew, y:ynew});
    }
    if elem == "]" {
        let xnew=coor.x-1;
        new_coordinates.push(Coordinate{x:coor.x, y:ynew});
        new_coordinates.push(Coordinate{x:xnew, y:ynew});
    }
}

fn is_free(grid: &mut Grid, elems_to_check:&Vec<Coordinate>) -> bool{
    let mut is_free = true;
    for elem in elems_to_check {
        if grid._elem(elem.x, elem.y) != "." {
            is_free = false;
        }
    }
    is_free
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

fn try_move_hor(robot_pos:&mut Coordinate, it_pos:&mut Coordinate, movement:&str, depth:&mut i32, grid:&mut Grid, vec_elem:&mut Vec<String>) {
    *depth += 1;
    let elem = grid._elem(it_pos.x, it_pos.y);

    if elem == "." && *depth == 1 {
        update_robot_pos(robot_pos, movement, grid);
    } else if elem == "[" || elem == "]" {
        change_pos(movement, it_pos);
        vec_elem.push(elem);
        try_move_hor(robot_pos, it_pos, movement, depth, grid, vec_elem);
    } else if elem== "."  {
        let mut it = 0;
        while let Some(last) = vec_elem.pop() {
            if movement == ">"{
                grid._set_str(it_pos.x-it as i32, it_pos.y, last);
            } else if movement == "<"{
                grid._set_str(it_pos.x+it as i32, it_pos.y, last);
            } 
            it += 1;
        }
        update_robot_pos(robot_pos, movement, grid);
    }
}

fn update_robot_pos(robot_pos: &mut Coordinate, movement: &str, grid: &mut Grid) {
    grid._set_str(robot_pos.x, robot_pos.y,".".to_string());
    change_pos(movement, robot_pos);
    grid._set_str(robot_pos.x, robot_pos.y,"@".to_string());
}

fn extend_grid (grid:&Grid) -> Grid {
    let grid_vec = grid.grid_vec();
    let mut grid_vec_extended = Vec::new();

    for vec in grid_vec{

        let mut new_string = Vec::new();
        for s in vec {
            if s == "#" {
                new_string.push("#");
                new_string.push("#");
            } else if s == "O" {
                new_string.push("[");
                new_string.push("]");
            } else if s == "." {
                new_string.push(".");
                new_string.push(".");
            } else if s == "@" {
                new_string.push("@");
                new_string.push(".");
            }
        }
        grid_vec_extended.push(new_string.iter().map(|&s| s.to_string()).collect());
    }

    Grid{grid_vec:grid_vec_extended}
}

fn sum_gps_boxes(grid: Grid, elem:&str) -> i32 {
    let mut sum_gps_boxes = 0;

    for i in 0..grid._width() as i32 {
        for j in 0..grid._height() as i32 {
           if grid._elem(i, j) == elem {
            sum_gps_boxes += 100 * j + i;
           }
        }
    }
    sum_gps_boxes
}

fn part1(input:&str) -> i32 {
    let (mut grid, moves) = parse_data(input);
    let mut rp = start_pos(&grid);

    for m in moves {
        move_robot_p1(&mut rp, &mut grid, m.as_str());
    }

    sum_gps_boxes(grid, "O")
}

fn part2(input:&str) -> i32 {
    let (grid, moves) = parse_data(input);
    let mut grid = extend_grid(&grid);
    let mut rp = start_pos(&grid);

    for m in moves {
        move_robot_p2(&mut rp, &mut grid, m.as_str());
    }

    sum_gps_boxes(grid,"[")
}

fn main() {
    let start = Instant::now();

    utils::answer((part1(INPUT), 1538871),(part2(INPUT), 1543338));

    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
}

#[cfg(test)]
mod tests {
    static TESTINPUT: &str = "test1";
    use super::*;

    #[test]
    fn test1_p1() {
        let part1 = part1("test1_p1");
        assert_eq!(part1, 2028);
    }
 
     #[test]
    fn test2_p1() {
        let part1 = part1("test2");
        assert_eq!(part1, 10092);
    }

    #[test]
    fn test1() {
        let part1 = part2(TESTINPUT);
        assert_eq!(part1, 618);
    }
 
     #[test]
    fn test2() {
        let part2 = part2("test2");
        assert_eq!(part2, 9021);
    }

    #[test]
    fn test3() {
        let part1 = part2("test3");
        assert_eq!(part1, 312);
    }
    
    #[test]
    fn test4() {
        let part1 = part2("test4");
        assert_eq!(part1, 912);
    }
    
     #[test]
    fn test5() {
        let mut grid = filereader::_input_into_grid("test5");
        let  m = "^";
        
        let mut rp = start_pos(&grid);
        move_robot_p2(&mut rp, &mut grid, m);
        let sum = sum_gps_boxes(grid.clone(), "[");
        assert_eq!(sum,511);
    }

     #[test]
    fn test6() {
        let mut grid = filereader::_input_into_grid("test6");
        let  m = "v";
        
        let mut rp = start_pos(&grid);
        move_robot_p2(&mut rp, &mut grid, m);
        let sum = sum_gps_boxes(grid.clone(), "[");
        grid._print();
        assert_eq!(sum,711);
    }

    #[test]
    fn test7() {
        let mut grid = filereader::_input_into_grid("test7");
        let  m = "v";
        
        grid._print();
        let mut rp = start_pos(&grid);
        move_robot_p2(&mut rp, &mut grid, m);
        let sum = sum_gps_boxes(grid.clone(), "[");
        assert_eq!(sum,1124);
    }
}