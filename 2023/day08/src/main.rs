use std::vec;

use lib::filereader;

#[derive(Clone)]
struct Instruction {
    start : String,
    left : String,
    right: String,
}

fn main() {
    let contents = filereader::_input("exampleinput2023day8");
    let steps = calculate_number_of_steps_part1(contents);
    assert_eq!(steps, 6);

    let contents = filereader::_input("../input/day08");
    let steps = calculate_number_of_steps_part1(contents);
    assert_eq!(steps, 13019);

    let contents = filereader::_input("exampleinput2023day8_2");
    let answer_ex_part2 = calculate_steps_part2(&contents);
    assert_eq!(answer_ex_part2, 6);

    let contents = filereader::_input("../input/day08");
    let answer_part2 = calculate_steps_part2(&contents);
    assert_eq!(answer_part2, 13524038372771);
    println!("answer part two {}",answer_part2);
}

fn calculate_steps_part2(contents: &String) -> i128 {
    let mut vector_steps = calculate_vector_steps(&contents);
    vector_steps.sort();

    let smallest_step = vector_steps[0];
    let mut found = false;
    let mut it:i128 = 1;
    let mut step_mult:i128 = 0;
    while found == false {
        found = true;
        step_mult = smallest_step*it;
        for step in vector_steps.clone(){
            if step_mult % step != 0 {
                found = false;
            }
        }
        it+=1;
    }
    step_mult
}

fn calculate_vector_steps(contents: &String) -> Vec<i128> {
    let ( instruction_vec ,sequence) = parse_data(contents.clone());

    let mut start_nodes:Vec<String> = Vec::new();
    for instruction in instruction_vec.clone() {
        if instruction.start.chars().nth(2).unwrap() == 'A' {
            start_nodes.push(instruction.start);
        }
    }

    println!("start nodes: {:?}", start_nodes);

    let mut vec_steps:Vec<i128> = Vec::new();
    let mut i = 0;
    let mut steps =0;
    let mut finished = false;

    for node in start_nodes.clone() {
        let mut node2 = node.clone();
        finished = false;
        while finished == false {
                for instruction in &instruction_vec {
                    if instruction.start == node2 {
                        if sequence.chars().nth(i).unwrap() == 'L' {
                            node2 = instruction.left.to_string();
                            break;
                        } else {
                            node2 = instruction.right.to_string();
                            break;
                        }
                    }
                }

                steps+=1;
                i+=1;
                if i >= sequence.len() {
                    i =0;
                }

                if is_finished(&node2) {
                    vec_steps.push(steps);
                    steps = 0;
                    finished = true;
                }
            }
    }
    println!("{:?}", vec_steps);
    vec_steps
}

fn is_finished (node: &String) -> bool{
    let char = node.chars().nth(2).unwrap();
    if char != 'Z' {
        return false;
    }
    return true;
}

fn calculate_number_of_steps_part1(contents: String) -> i32 {
    let ( instruction_vec ,sequence) = parse_data(contents.clone());

    let mut start:String = "AAA".to_string();
    let mut i = 0;
    let mut steps =0;
    while start != "ZZZ" {
        for instruction in &instruction_vec {
            if instruction.start == start {
                if sequence.chars().nth(i).unwrap() == 'L' {
                    start = instruction.left.to_string();
                    break;
                } else {
                    start = instruction.right.to_string();
                    break;
                }
            }
        }
        steps+=1;
        i += 1;

        if i >= sequence.len() {
            i =0;
        }

    }
    steps
}

fn parse_data(contents: String) -> (Vec<Instruction>, String )  {
    let mut vect_instruction:Vec<Instruction> = Vec::new();
    let inputs: Vec<&str> = contents.split("\n\n").collect::<Vec<&str>>();
    let sequence = inputs[0].to_string();
    let mut instructions = inputs[1].split("\n").collect::<Vec<&str>>();

    for content in instructions.iter_mut() {
        let content = content.replace("=", "");
        let content = content.replace("(", "");
        let content = content.replace(")", "");
        let content = content.replace(",", "");
        let instruction:Vec<&str> = content.split_whitespace().collect::<Vec<&str>>();
        vect_instruction.push(Instruction { start: instruction[0].to_string(), left: instruction[1].to_string(), right: instruction[2].to_string()})

    }
    (vect_instruction, sequence)
}
