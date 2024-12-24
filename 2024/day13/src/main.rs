use lib::filereader;
use lib::utils;
use lib::utils::*;
use lib::grid::*;
use core::num;
use std::time::Instant;
use regex::Regex;
use std::collections::HashSet;

static INPUT: &str = "../input/day13";

#[derive(Debug, Default, Clone)]
struct Button {
    x : i32,
    y : i32
}

#[derive(Debug, Default, Clone)]
struct Machine {
    button_a: Button,
    button_b: Button,
    prize_x: i32,
    prize_y: i32
}

fn parse_data(file:&str) -> Vec<Machine>
{
    let mut machines = Vec::new();
    let content = filereader::_input(file);

    let re = Regex::new(r"\d+").unwrap();
    let mut it = 1;
    let mut machine = Machine::default();


    for line in content.lines() {
        let numbers: Vec<i32> = re
        .find_iter(line)
        .filter_map(|mat| mat.as_str().parse::<i32>().ok())
        .collect();
    
        if it == 1 {
            machine.button_a = Button { x: numbers[0], y:numbers[1]};
        } else if it == 2 {
            machine.button_b = Button { x: numbers[0], y:numbers[1]};
        } else if it == 3 {
            machine.prize_x = numbers[0];
            machine.prize_y = numbers[1];
            machines.push(machine.clone());
        } else if it == 4 {
                it = 0;
            }
        it+=1;
    }
    println!("{:?}",machines);

    machines
}

fn cheapest_win(machine: &Machine) -> i32
{
    let posibilities = calculate_posibilities(machine);
    
    let mut init = true;
    let mut cheapest_win = 0;
    
    for posibility in posibilities {
        let tokens = posibility[0]*3 + posibility[1];
        if init {
            cheapest_win = tokens;
            init = false;
        } else {
            if tokens < cheapest_win {
                cheapest_win = tokens;
            }
        }
    }

    cheapest_win
}

fn calculate_posibilities(machine: &Machine) -> Vec<Vec<i32>> {
    let xa = machine.button_a.x;
    let xb = machine.button_b.x;
    let px = machine.prize_x;
    let ya = machine.button_a.y;
    let yb = machine.button_b.y;
    let py = machine.prize_y;

    let mut tokens_a = Vec::new();
    let mut tokens_b = Vec::new();
    let mut tokens_common = Vec::new();

    for i in 0..100 {
        for j in 0..100 {
            if i*xa + j*xb == px {
                println!("xtokens: xa: {} xb: {}", i, j);
                tokens_a.push(vec![i,j]);
            }
            if i*ya + j*yb == py {
                println!("ytokens: ya: {} yb: {}", i, j);
                tokens_b.push(vec![i,j]);
            }
        }
    }

    for set1 in &tokens_a {
        for set2 in &tokens_b {
            if set1 == set2 {
                tokens_common.push(set1.clone());
            }
        }
    }

    tokens_common
}

fn main() {
    let start = Instant::now();
    
    // utils::answer((part1(INPUT), 1370258),(part2(INPUT), 805814));
    let machines = parse_data(INPUT);
    
    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
}

#[cfg(test)]
mod tests {
    static TESTINPUT: &str = "test1";
    use super::*;

    #[test]
    fn test1() {
        let machine = Machine {button_a: Button { x : 94, y : 34 }, 
                                        button_b: Button { x : 22, y : 67 },
                                        prize_x: 8400,
                                        prize_y: 5400};
                                
        let tokens = cheapest_win(&machine);
        assert_eq!(tokens, 280);
    }
    
}
