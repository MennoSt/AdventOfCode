use std::fs;
use std::env;
use grid::Grid;
use grid::Gridi32;

pub fn _input(path: &str) -> String {
    let current_dir = env::current_dir().unwrap();
    let file_path = current_dir.join(path);
    let contents = fs::read_to_string(file_path).unwrap();
    contents
}

pub fn _print_content(content: &str) {
    for line in content.lines() {
        println!("{}", line);
    }
}

pub fn _input_into_grid(contents: &str)-> Grid
{
    let contents = _input(&contents);
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

pub fn _input_into_grid_i32(contents: &str)-> Gridi32
{
    let contents = _input(&contents);
    let grid = read_into_grid_i32(&contents);
    grid
}

fn read_into_grid_i32(contents: &str) -> Gridi32 {
    let mut contents_vector: Vec<Vec<String>> = Vec::new();
    for line in contents.lines() {
        let test:Vec<char> = line.chars().collect();
        let strings = test
        .iter()
        .map(|c| String::from(c.to_string()))
        .collect::<Vec<String>>();
        contents_vector.push(strings);
    }

    let int_vec: Vec<Vec<i32>> = contents_vector
    .into_iter()
    .map(|inner_vec| inner_vec.into_iter().map(|s| s.parse::<i32>().unwrap()).collect())
    .collect();

    Gridi32{grid_vec:int_vec}
}