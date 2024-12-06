use std::fs;
use std::env;

pub fn input(path: &str) -> String {
    let current_dir = env::current_dir().unwrap();
    let file_path = current_dir.join(path);
    let contents = fs::read_to_string(file_path).unwrap();
    contents
}