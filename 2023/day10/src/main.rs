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
    }
}

fn main()
{
    let grid_init = parse_data("exampleinput2023day10");
    let grid_mut_vec:Vec<Vec<i32>> = vec![vec![0; grid_init._width()]; grid_init._height()];
    let mut grid_mut = Gridi32{grid_vec:grid_mut_vec};
    
    let vec_dir:Vec<Direction> = vec![Direction::Left, Direction::Right, Direction::Up, Direction::Down];
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
            grid_mut._print();
            println!("{}", "");
            step += 1;
        }
    }
}

fn mutate(current_step: Direction, next_elem: &mut String, 
    grid_init: &Grid, current_position: &mut Coordinate, next_step: &mut Direction) {

    // update current position
    if current_step == Direction::Right {
        *next_elem = grid_init._elem(current_position.x+1, current_position.y);
        current_position.x +=1;
    } else if current_step == Direction::Left {
        *next_elem = grid_init._elem(current_position.x-1, current_position.y);
        current_position.x -=1;
    } else if current_step == Direction::Up {
        *next_elem = grid_init._elem(current_position.x, current_position.y-1);
        current_position.y -=1;
    } else if current_step == Direction::Down {
        *next_elem = grid_init._elem(current_position.x, current_position.y+1);
        current_position.y +=1;
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
    } else if *next_elem == "|" {
        if current_step == Direction::Down {
          *next_step = Direction::Down;
        } else {
          *next_step = Direction::Up;
        }
    } else if *next_elem == "-" {
        *next_step = Direction::Left;
    } else {
        *next_step = Direction::None;
    }
}

fn start_coord(grid_init: &Grid) -> Coordinate {
    let mut start_coord= Coordinate{x:0,y:0};
    for x in 0..grid_init._height() {
        for y in 0..grid_init._width() {
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
