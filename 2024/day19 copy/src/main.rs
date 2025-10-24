
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

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
struct VisitedNodes {
    index:usize,
    substr:String,
}

struct Towel {
    towel:String,
    arrangements:i128,
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



fn is_possible_design (towels:&Vec<String>, design:&str, towels_vec:&Vec<Towel>) -> (bool,i128){
    
    let start_i = 0;
    let mut found = false;
    let mut visited_nodes = Vec::new();
    let mut arrcount = 0;

    next_pos(towels, design, start_i, &mut found, &mut visited_nodes, towels_vec, &mut arrcount);

    // println!("{}",arrcount);

    // if found && arrcount == 0 {
    //     arrcount = 1;
    // }
unsafe{
    (found,COUNTER)
}
}

static mut COUNTER:i128 = 0;

fn next_pos(towels: &Vec<String>, design: &str, start_index: usize, found:&mut bool, visited_nodes:&mut Vec<VisitedNodes>, towels_vec:&Vec<Towel>, arrcount:&mut i128) {

    let mut x = "".to_string();
    if start_index == design.len() {
        *found = true;
        unsafe {
        if *arrcount == 0 {
                COUNTER += 1;
                println!("{}",COUNTER);
            
        } else {
            COUNTER += *arrcount;
            println!("{}",COUNTER);
        }
        }
        *arrcount=0;
    }
    
    // else if !*found {
        for j in start_index..design.len() {
            let test = design.chars().nth(j).unwrap();
            x.push(test);
            let node = VisitedNodes{index:j.clone(),substr:x.clone()};
            if towels.contains(&x) && !visited_nodes.contains(&node){

                    visited_nodes.push(node);
                    *arrcount += calculate_arr_inc(&x, towels_vec);
                    next_pos(towels, design,j + 1, found, visited_nodes,towels_vec, arrcount);
                    *arrcount -= calculate_arr_inc(&x, towels_vec);
                    // visited_nodes.pop();
                }
            }
    // }
}

fn calculate_arr_inc(part:&str, towels: &Vec<Towel>) -> i128 {
    let mut count = 0;

    for t in towels{
        if t.towel == part {
            if t.arrangements > 1 {
                count = t.arrangements;
            } 
        }
    }

    count
}
// fn part1 (input:&str) -> i32 {
//     let arrangement = parse_data(input);

//     let mut pos_designs = 0;
//     for d in arrangement.designs {
//         if is_possible_design(&arrangement.towels, &d) {
//             pos_designs += 1;
//         }
//     }

//     pos_designs
// }

fn calculate_arrangements(towels:&Vec<String>, design:&str) -> i128
{
    let start_i = 0;
    let mut found = 0;

    next_total_arrangements(towels, design, start_i, &mut found);

    found
}

fn next_total_arrangements(towels: &Vec<String>, design: &str, start_index: usize, found:&mut i128) {

    let mut x = "".to_string();
    if start_index == design.len() {
        *found += 1;
    } 

    for j in start_index..design.len() {
            let test = design.chars().nth(j).unwrap();
            x.push(test);
            if towels.contains(&x) {
                    next_total_arrangements(towels, design,j + 1, found);
                }
            }

}

fn create_towel_arrangements (arrangement:&Arrangement) -> Vec<Towel> {
    let mut towels_vec = Vec::new();

    for towel in &arrangement.towels {
        let count = calculate_arrangements(&arrangement.towels, &towel);
        towels_vec.push(Towel{towel:towel.clone(),arrangements:count});
    }
    towels_vec
}

fn part2 (input:&str) -> i128 {
    let arrangement = parse_data(input);

    calculate_total_arrangements(arrangement)
}

fn calculate_total_arrangements(arrangement: Arrangement) -> i128 {
    let towel_vec = create_towel_arrangements(&arrangement);
    let mut total_arrangements = 0;
    for d in arrangement.designs {
        let output = is_possible_design(&arrangement.towels, &d, &towel_vec);
        if  output.0 {
            total_arrangements += output.1;
            // println!("{}",total_arrangements);
        }
    }

    total_arrangements
}

//62711
//62712
//74674 too low

fn main() {
    let start = Instant::now();

    // let part1 = part1(INPUT);
    // println!("{}",part1);

    // assert_eq!(part1,304);
    
    let part2 = part2(INPUT);
    println!("{}",part2);
    // assert_eq!(part2,"38,63");

    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
}

#[cfg(test)]
mod tests {
    static TESTINPUT: &str = "test1";
    use std::vec;

    use super::*;

    // #[test]
    // fn test1() {
    //     let part1 = part1(TESTINPUT);
    //     assert_eq!(part1, 6);
    // }

    //  #[test]
    // fn test2() {
    //     let towels = vec!["b","r","rw"];
    //     let towels: Vec<String> = towels.into_iter().map(String::from).collect();
    //     let design = "brwrr";
    //     assert_eq!(is_possible_design(&towels, design), true);
    // }

    // #[test]
    // fn test3() {
    //     let towels = vec!["b","r"];
    //     let towels: Vec<String> = towels.into_iter().map(String::from).collect();
    //     let design = "brwrr"; 
    //     assert_eq!(is_possible_design(&towels, design), false);
    // }

    // #[test]
    // fn test4() {
    //     let towels = vec!["b"];
    //     let towels: Vec<String> = towels.into_iter().map(String::from).collect();
    //     let design = "bbbbbbn"; 
    //     assert_eq!(is_possible_design(&towels, design), false);
    // }

    // #[test]
    // fn test5() {
    //     let towels = vec!["b","r"];
    //     let towels: Vec<String> = towels.into_iter().map(String::from).collect();
    //     let design = "br"; 
    //     assert_eq!(is_possible_design(&towels, design), true);
    // }

    // #[test]
    // fn test6() {
    //     let towels = vec!["b","rn"];
    //     let towels: Vec<String> = towels.into_iter().map(String::from).collect();
    //     let design = "bbbbbrn"; 
    //     assert_eq!(is_possible_design(&towels, design), true);
    // }
    
    #[test]
    fn test6() {
        let towels = vec!["bw", "wr", "bwr","lwr","rl"];
        let towels: Vec<String> = towels.into_iter().map(String::from).collect();
        let design = "bwrlwr".to_string(); 
        let designs = vec![design];

        let arrangement = Arrangement{towels:towels, designs:designs};

        assert_eq!(calculate_total_arrangements(arrangement), 2);
    }

    #[test]
    fn test7() {
        let towels = vec!["b","r", "wr", "b", "g", "bwu", "rb", "gb", "br"];
        let towels: Vec<String> = towels.into_iter().map(String::from).collect();
        let design = "brwrr".to_string();
        let designs = vec![design];
        
        let arrangement = Arrangement{towels:towels, designs:designs};

        assert_eq!(calculate_total_arrangements(arrangement), 2);
    }

    #[test]
    fn test8() {
        let part1 = part2(TESTINPUT);
        assert_eq!(part1, 16);
    }

    #[test]
    fn test9() {
        let part2 = part2("test2");
        assert_eq!(part2, 2);
    }
}
