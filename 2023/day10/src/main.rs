use lib::filereader;

#[derive(Clone)]
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

#[derive(Clone)]
struct Grid {
    grid_vec: Vec<Vec<String>>,
}

impl Grid {
    fn _height(&self) -> usize {
        return self.grid_vec.len();
    }

    fn _width(&self) -> usize {
        return self.grid_vec[0].len();
    }

    fn _elem(&self, x: i32, y: i32) -> String {
        let point_char = ".";

        if x < 0 || y < 0 {
            return point_char.to_string();
        }

        let x_usize: usize = x as usize;
        let y_usize: usize = y as usize;

        if x_usize > (self._width() -1) || y_usize > (self._height()-1) {
            return point_char.to_string();
        }

        return self.grid_vec[y_usize][x_usize].clone();
    }

    fn _set(&mut self, x: i32, y: i32, value: i32) {
        let x_usize: usize = x as usize;
        let y_usize: usize = y as usize;
        self.grid_vec[y_usize][x_usize] = value.to_string();
    }

    fn _print(&self) {
        for grid in &self.grid_vec {
            println!("{:?}",grid);
        }
        println!("{}","");
    }
}

#[derive(Clone)]
struct Gridi32 {
    grid_vec: Vec<Vec<i32>>,
}

impl Gridi32 {
    fn _height(&self) -> usize {
        return self.grid_vec.len();
    }

    fn _width(&self) -> usize {
        return self.grid_vec[0].len();
    }

    fn _elem(&self, x: i32, y: i32) -> i32 {
        if x < 0 || y < 0 {
            return 0;
        }

        let x_usize: usize = x as usize;
        let y_usize: usize = y as usize;

        if x_usize > (self._width() -1) || y_usize > (self._height()-1) {
            return 0;
        }

        return self.grid_vec[y_usize][x_usize].clone();
    }

    fn _set(&mut self, x: i32, y: i32, value: i32) {
        let x_usize: usize = x as usize;
        let y_usize: usize = y as usize;
        self.grid_vec[y_usize][x_usize] = value;
    }

    fn _print(&self) {
        for grid in &self.grid_vec {
            println!("{:?}",grid);
        }
        println!("{}","");
    }

    fn _max(&self) -> Option<i32> {
        let max_value = self.grid_vec
        .iter()
        .flatten()
        .max()
        .cloned();
        
        return max_value;
    }
}

fn main()
{
    assert_eq!(calculate_largest_path("test1"), 4);
    assert_eq!(calculate_largest_path("test2"), 8);
    assert_eq!(calculate_largest_path("test3"), 0);
    assert_eq!(calculate_largest_path("test4"), 2);
    assert_eq!(calculate_largest_path("input2023day10"), 6806);

    assert_eq!(calculate_n_enclosed("test5"), 2);
    assert_eq!(calculate_n_enclosed("test6"), 4);
    assert_eq!(calculate_n_enclosed("test7"), 2);
    assert_eq!(calculate_n_enclosed("test8"), 3);
    assert_eq!(calculate_n_enclosed("test9"), 8);
    assert_eq!(calculate_n_enclosed("test10"), 10);
    assert_eq!(calculate_n_enclosed("input2023day10"),449);
}



fn calculate_n_enclosed(input_file: &str) ->i32
{
    let grid_init = parse_data(input_file);
    let mut grid_visited = visited_grid(&grid_init);
    let mut n_enclosed =0;

    for x in 0..grid_init._width() as i32 {
        for y in 0..grid_init._height() as i32 {
            if grid_visited._elem(x, y) == 0 {
                if is_enclosed_up(&grid_init,&grid_visited, x, y) &&
                is_enclosed_down(&grid_init,&grid_visited, x, y) &&
                is_enclosed_left(&grid_init,&grid_visited, x, y) &&
                is_enclosed_right(&grid_init,&grid_visited, x, y)
                {
                    n_enclosed +=1;
                    grid_visited._set(x, y, 9);
                }
            }
        }
    }
    return n_enclosed;

}

fn is_enclosed_left(grid_init: &Grid, grid_visited:&Gridi32, x: i32, y: i32) -> bool {
    let mut counter = 0;
    let mut previous_elem = ".".to_string();
    for i in 0..x as i32 {
        let elem = grid_init._elem(i, y);
        if grid_visited._elem(i, y) == 0 || grid_visited._elem(i, y) == 9 {

        } else if elem == "^" && previous_elem == "L"{
            counter +=1;
        } else if elem == "J" && previous_elem == "F" {
            counter +=1;
        } else if elem == "|" {
            counter +=1;
        } else if elem == "-" || elem == "."
        {
        } else {
            previous_elem = elem.clone();
        }
    }
    is_odd(counter)
}

fn is_enclosed_right(grid_init: &Grid,grid_visited:&Gridi32, x: i32, y: i32) -> bool {
    let mut counter = 0;
    let mut previous_elem = ".".to_string();
    let xit = x+1;
    for i in xit..grid_init._width() as i32 {
        let elem = grid_init._elem(i, y);
        if grid_visited._elem(i, y) == 0 || grid_visited._elem(i, y) == 9  {

        } else if elem == "^" && previous_elem == "L"{
            counter +=1;
        } else if elem == "J" && previous_elem == "F" {
            counter +=1;
        } else if elem == "|" {
            counter +=1;
        } else if elem == "-" || elem == "."
        {
        } else {
            previous_elem = elem.clone();
        }
    }
    is_odd(counter)
}

fn is_enclosed_down(grid_init: &Grid, grid_visited:&Gridi32, x: i32, y: i32) -> bool {
    let mut counter = 0;
    let mut previous_elem = ".".to_string();
    let yit = y+1;
    for i in yit..grid_init._height() as i32 {
        let elem = grid_init._elem(x, i);
        if grid_visited._elem(x, i) == 0 || grid_visited._elem(x, i) == 9 {

        } else if elem == "L" && previous_elem == "^"{
            counter +=1;
        } else if elem == "J" && previous_elem == "F" {
            counter +=1;
        } else if elem == "-" {
            counter +=1;
        } else if elem == "|" || elem == "."
        {
        } else {
            previous_elem = elem.clone();
        }
    }
    is_odd(counter)
}

fn is_enclosed_up(grid_init: &Grid, grid_visited:&Gridi32, x: i32, y: i32) -> bool {
    let mut counter = 0;
    let mut previous_elem = ".".to_string();
    for i in 0..y as i32 {
        let elem = grid_init._elem(x, i);
        if grid_visited._elem(x, i) == 0 || grid_visited._elem(x, i) == 9 {

        } else if elem == "L" && previous_elem == "^"{
            counter +=1;
        } else if elem == "J" && previous_elem == "F" {
            counter +=1;
        } else if elem == "-" {
            counter +=1;
        } else if elem == "|" || elem == "."
        {
        } else {
            previous_elem = elem.clone();
        }
    }
    is_odd(counter)
}

fn is_odd(n: i32) -> bool {
    n % 2 != 0
}


fn visited_grid(grid_init: &Grid) -> Gridi32 {
    let grid_mut_vec = vec![vec![0; grid_init._width()]; grid_init._height()];
    let mut grid_mut = Gridi32{grid_vec:grid_mut_vec};
    
    let vec_dir= vec![Direction::Left, Direction::Right, Direction::Up, Direction::Down];
    let start_position = start_coord(&grid_init);
    grid_mut._set(start_position.x, start_position.y,1);

    for dir in vec_dir {
        let mut next_step = Direction::None;
        let mut current_step = dir;
        let mut next_elem: String = "S".to_string();
        let mut current_position = start_position.clone();

        mutate(current_step.clone(), &mut next_elem, &grid_init, &mut current_position, &mut next_step);
        current_step = next_step.clone();
        while current_step != Direction::None && next_elem != "S" {
            let elem = grid_mut._elem( current_position.x, current_position.y);

            if elem == 0 {
                grid_mut._set(current_position.x.clone(), current_position.y,1);
            }
            mutate(current_step.clone(), &mut next_elem, &grid_init, &mut current_position, &mut next_step);
            current_step = next_step.clone();
        }
    }
    grid_mut
}

fn calculate_largest_path(input_file: &str) -> i32 {
    let grid_init = parse_data(input_file);
    let grid_mut_vec = vec![vec![0; grid_init._width()]; grid_init._height()];
    let mut grid_mut = Gridi32{grid_vec:grid_mut_vec};
    
    let vec_dir= vec![Direction::Left, Direction::Right, Direction::Up, Direction::Down];
    let start_position = start_coord(&grid_init);

    for dir in vec_dir {
        let mut next_step = Direction::None;
        let mut current_step = dir;
        let mut next_elem: String = "S".to_string();
        let mut current_position = start_position.clone();
        let mut step = 1;
    
        mutate(current_step.clone(), &mut next_elem, &grid_init, &mut current_position, &mut next_step);
        current_step = next_step.clone();
        while current_step != Direction::None && next_elem != "S" {
            let elem = grid_mut._elem( current_position.x, current_position.y);

            if elem > step || elem == 0 {
                grid_mut._set(current_position.x.clone(), current_position.y,step);
            }
            mutate(current_step.clone(), &mut next_elem, &grid_init, &mut current_position, &mut next_step);
            current_step = next_step.clone();
            step += 1;
        }
    }
    let max = grid_mut._max().unwrap_or(0);
    return max;
}

fn mutate(current_step: Direction, next_elem: &mut String, 
    grid_init: &Grid, current_position: &mut Coordinate, next_step: &mut Direction) {

    // update current position
    if current_step == Direction::Right {
        *next_elem = grid_init._elem(current_position.x+1, current_position.y);
        if next_elem == "^"|| next_elem == "-"|| next_elem == "J" {
            current_position.x +=1;
        }else {
            *next_step = Direction::None;
            return;
        }
    } else if current_step == Direction::Left {
        *next_elem = grid_init._elem(current_position.x-1, current_position.y);
        if next_elem == "L" || next_elem == "F" || next_elem == "-" {
            current_position.x -=1;
        } else {
            *next_step = Direction::None;
            return;
        }
    } else if current_step == Direction::Up {
        *next_elem = grid_init._elem(current_position.x, current_position.y-1);
        if next_elem == "|" || next_elem == "F"||next_elem == "^" {
            current_position.y -=1;
        }else {
            *next_step = Direction::None;
            return;
        }       
    } else if current_step == Direction::Down {
        *next_elem = grid_init._elem(current_position.x, current_position.y+1);
        if next_elem == "L" || next_elem == "J" || next_elem == "|" {
            current_position.y +=1;
        } else {
            *next_step = Direction::None;
            return;
        }
    }

    // determine next step
    if *next_elem == "." {
        *next_step = Direction::None;
    } else if *next_elem == "-" {
        if current_step == Direction::Right {
            *next_step = Direction::Right;
        } else {
            *next_step = Direction::Left;
        }
    } else if *next_elem == "^" {
        if current_step == Direction::Up {
            *next_step = Direction::Left;
        } else {
            *next_step = Direction::Down;
        }
    } else if *next_elem == "J" {
        if current_step == Direction::Right {
            *next_step = Direction::Up
        } else {
            *next_step = Direction::Left;
        }
    } else if *next_elem == "L" {
        if current_step == Direction::Down {
            *next_step = Direction::Right
        } else {
            *next_step = Direction::Up;
        }
    } 
    else if *next_elem == "|" {
        if current_step == Direction::Down {
          *next_step = Direction::Down;
        } else {
          *next_step = Direction::Up;
        }
    } 
    else if *next_elem == "F" 
    {
        if current_step == Direction::Up {
          *next_step = Direction::Right;
        } else {
          *next_step = Direction::Down;
        }
    }
    else {
        *next_step = Direction::None;
    }
}

fn start_coord(grid_init: &Grid) -> Coordinate {
    let mut start_coord= Coordinate{x:0,y:0};
    for y in 0..grid_init._height() {
        for x in 0..grid_init._width() {
            if grid_init._elem(x as i32, y as i32) == "S" {
                start_coord.x = x as i32;
                start_coord.y = y as i32;
            } 
        }
    }
    start_coord
}

fn parse_data(contents: &str)->Grid
{
    let contents = filereader::_input(&contents);
    let contents = contents.replace('7',"^");

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
