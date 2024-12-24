use lib::filereader;
use lib::utils;
use std::time::Instant;
use regex::Regex;

static INPUT: &str = "../input/day13";

#[derive(Debug, Default, Clone)]
struct Button {
    x : i128,
    y : i128
}

#[derive(Debug, Default, Clone)]
struct Machine {
    button_a: Button,
    button_b: Button,
    prize_x: i128,
    prize_y: i128
}

fn parse_data(file:&str) -> Vec<Machine>
{
    let mut machines = Vec::new();
    let content = filereader::_input(file);

    let re = Regex::new(r"\d+").unwrap();
    let mut it = 1;
    let mut machine = Machine::default();


    for line in content.lines() {
        let numbers: Vec<i128> = re
        .find_iter(line)
        .filter_map(|mat| mat.as_str().parse::<i128>().ok())
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

    machines
}

fn increase_input(machines: &mut Vec<Machine>) {
    for machine in machines {
        machine.prize_x += 10000000000000;
        machine.prize_y += 10000000000000;
    }
}
fn solve_linear_equations(
    xa: i128, xb: i128, ya: i128, yb: i128, px: i128, py: i128,
) -> Option<(i128, i128)> {
    // Calculate the determinant
    let determinant = xa * yb - xb * ya;

    // Check if the determinant is zero (no unique solution)
    if determinant == 0 {
        return None; // No solution or infinite solutions
    }

    // Calculate the numerator for i and j
    let i_numerator = px * yb - py * xb;
    let j_numerator = xa * py - ya * px;

    // Check if the solutions are integers
    if i_numerator % determinant != 0 || j_numerator % determinant != 0 {
        return None; // No integer solution exists
    }

    // Compute i and j
    let i = i_numerator / determinant;
    let j = j_numerator / determinant;

    Some((i, j)) // Return the solution
}

fn calculate_posibilities(machine: &Machine) -> Vec<Vec<i128>> {
    let xa = machine.button_a.x;
    let xb = machine.button_b.x;
    let px = machine.prize_x;
    let ya = machine.button_a.y;
    let yb = machine.button_b.y;
    let py = machine.prize_y;

    let mut posibilities = Vec::new();

    match solve_linear_equations(xa, xb, ya, yb, px, py) {
        Some((i, j)) => {
            posibilities.push(vec![i,j]);
        }
        None => {
        }
    }
    posibilities
}

fn cheapest_win(machine: &Machine) -> i128
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

fn part1 (input: &str) -> i128 {
    let machines = parse_data(input);
    let mut total_tokens = 0;

    for machine in &machines {
        total_tokens += cheapest_win(&machine);
    }
    total_tokens
}

fn part2 (input: &str) -> i128 {
    let mut machines = parse_data(input);
    increase_input(&mut machines);

    let mut total_tokens = 0;
    for machine in &machines {
        total_tokens += cheapest_win(&machine);
    }
    total_tokens
}

fn main() {
    let start = Instant::now();
    
    utils::answer((part1(INPUT), 26005),(part2(INPUT), 105620095782547));

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
    
     #[test]
    fn test2() {
        let tokens = part1(&TESTINPUT);
        assert_eq!(tokens, 480);
    }
       
}
