
use lib::filereader;
use lib::utils::*;
use std::time::Instant;
use lib::grid::*;
use std::collections::VecDeque;
use regex::Regex;

static INPUT: &str = "../input/day19";

struct Arrangement {
    towels:Vec<String>,
    designs:Vec<String>,
}

fn parse_data (input:&str) -> Arrangement {
    let mut towels = Vec::new();
    let mut designs = Vec::new();
    let content = filereader::_input(&input);

    let mut first = true; 
    for line in content.lines() {
        if first {
            towels = line.split(", ").map(|s| s.to_string()).collect();
            first = false;
        } else {
            if !line.is_empty(){
                designs.push(line.to_string());
            }
        }
    }

    Arrangement{towels:towels, designs:designs}
}

fn part1 (input:&str) -> i32 {
    let arrangement = parse_data(input);

    let mut pos_designs = 0;
    for d in arrangement.designs {
        if is_possible_design(&arrangement.towels, &d) {
            pos_designs += 1;
            println!("found:{} design:{}",pos_designs, d);
        }
    }

    pos_designs
}

fn is_possible_design (towels:&Vec<String>, design:&str) -> bool{
    
    let mut start_i = 0;
    let mut end_i = 0;
    let mut found = false;

    next_pos(towels, design, start_i, &mut found);

    found
}

fn next_pos(towels: &Vec<String>, design: &str, start_index: usize, found:&mut bool) {

    if start_index == design.len()-1 {
        *found = true;
    } else if !*found {
        for j in start_index..design.len() {
            let x = &design[start_index..j].to_string();
            if towels.contains(x) {
                next_pos(towels, design,j, found);
            }
        }
    }
}

fn main() {
    let start = Instant::now();

    let part1 = part1(INPUT);
    println!("{}",part1);

    // assert_eq!(part1,306);
    
    // let part2 = part2(INPUT, 71);
    // println!("{}",part2);
    // assert_eq!(part2,"38,63");

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
        assert_eq!(part1, 6);
    }

     #[test]
    fn test2() {
        let towels = vec!["b","r","rw"];
        let towels: Vec<String> = towels.into_iter().map(String::from).collect();
        let design = "brwrr";
        assert_eq!(is_possible_design(&towels, design), true);
    }

    #[test]
    fn test3() {
        let towels = vec!["b","r"];
        let towels: Vec<String> = towels.into_iter().map(String::from).collect();
        let design = "brwrr"; 
        assert_eq!(is_possible_design(&towels, design), false);
    }


}