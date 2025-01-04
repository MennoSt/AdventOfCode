use lib::filereader;
use lib::utils;
use lib::utils::*;
use std::time::Instant;
use regex::Regex;

static INPUT: &str = "../input/day17";

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
struct Program {
    register_a: i128,
    register_b: i128,
    register_c: i128,
    instruction: Vec<i128>,
    output: Vec<i128>,
}

fn parse_data(input: &str) -> Program {
    let content = filereader::_input(input);
    let mut numbers = Vec::new();
    let re = Regex::new(r"\d+").unwrap();

    for line in content.lines() {
        let mut line_numbers:Vec<i128> = re
        .find_iter(line)
        .filter_map(|mat| mat.as_str().parse::<i128>().ok())
        .collect();

        numbers.append(&mut line_numbers);
    }
    
    Program{register_a:numbers[0], register_b:numbers[1], register_c:numbers[2], instruction:numbers[3..].to_vec(), output:Vec::new()}
}

fn operate(program:&mut Program) {

        let mut ip = 0;
        while ip < program.instruction.len() {
            let opcode = program.instruction[ip];
            let operand = program.instruction[ip+1];
            let mut combo_operand = 0;
            let mut increase_step = true;
    
            set_combo_operand(program, operand, &mut combo_operand);

            match opcode {
                0 => program.register_a = dv_instruction(program, combo_operand),
                1 => program.register_b ^= operand,
                2 => program.register_b = combo_operand % 8,
                3 => if program.register_a != 0 {
                        increase_step = false; 
                        ip = operand as usize;
                    }
                4 => program.register_b ^= program.register_c,
                5 => program.output.push(combo_operand % 8),
                6 => program.register_b = dv_instruction(program, combo_operand),
                7 => program.register_c = dv_instruction(program, combo_operand),
                _ => println!("Invalid operand"),
            }

            if increase_step {
                ip += 2;
            }
        }

}

fn operate_p2(program:&mut Program) {

    let mut ip = 0;
    let mut pos_it = 0;
    while ip < program.instruction.len() {
        let opcode = program.instruction[ip];
        let operand = program.instruction[ip+1];
        let mut combo_operand = 0;
        let mut increase_step = true;

        set_combo_operand(program, operand, &mut combo_operand);

        match opcode {
            0 => program.register_a >>= combo_operand,
            1 => program.register_b ^= operand,
            2 => program.register_b = combo_operand % 8,
            3 => if program.register_a != 0 {
                    increase_step = false; 
                    ip = operand as usize;
                },
            4 => program.register_b ^= program.register_c,
            5 => { 
                    let new_val = combo_operand % 8;
                    if program.instruction[pos_it] != new_val {
                        break;
                    } else{
                        program.output.push(new_val);
                    }
                    pos_it+= 1;
                },
            6 => program.register_b = program.register_a >> combo_operand,
            7 => program.register_c = program.register_a >> combo_operand,
            _ => println!("Invalid operand"),
        }


        if increase_step {
            ip += 2;
        }
    }

}

fn dv_instruction(program: &mut Program, combo_operand: i128) -> i128{
    program.register_a/(2_i32.pow(combo_operand as u32)) as i128
}

fn set_combo_operand(program: &mut Program, operand: i128, combo_operand: &mut i128) {

    match operand {
        0..=3 => *combo_operand = operand,
        4 => *combo_operand = program.register_a,
        5 => *combo_operand = program.register_b,
        6 => *combo_operand = program.register_c,
        _ => println!("Invalid operand"),
    }
}

fn convert_to_commastr(program: Program) -> String {
    let mut number:String = "".to_string();

    for digit in &program.output {
        let dig = digit.to_string();
        number.push_str(&dig);
        number.push_str(",");
    }
    number.pop();
    number
}

fn part1(input: &str) -> String {
    let mut program = parse_data(input);
    operate(&mut program);

    convert_to_commastr(program)
}

fn part2(input: &str, start_it:i128, increase:i128) -> i128{
    let mut program = parse_data(input);
    let mut program_clone = program.clone();

    let mut it = start_it;
    let mut it_prev = 0;

    loop {
        program_clone = program.clone();
        program_clone.register_a = it;
        
        operate_p2(&mut program_clone);
        
        if program_clone.instruction == program_clone.output {
            println!("{:?}", program_clone.instruction);
            println!("{:?}", program_clone.output);

            println!("start it rega :{:?}",it);
            break;
        }
        
        
        // print values to check the iteration input for the next iteration
        // if program_clone.output.len() >= 11 {
        //     let diff = it -it_prev;
        //     println!("{:?}",it);
        //     println!("diff {:?}", diff);
        //     println!("{:?}",program_clone);
        //     it_prev = it;
        // }

        it += increase;
    }

    it

}

fn main() {
    let start = Instant::now();

    let part1 = part1(INPUT);
    println!("{:?}",part1);
    assert_eq!(part1, "7,3,1,3,6,3,6,0,2");
    let part2 = part2(INPUT,17918362, 262144); //input values found by checking which values/iterations are "hot"/which have a five or more correct elements, this manual step should be automated.
    assert_eq!(part2, 105843716614554);

    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
}

#[cfg(test)]
mod tests {
    static TESTINPUT: &str = "test1";
    use super::*;

    #[test]
    fn test1() {

        let mut program = Program{register_a:0,
            register_b: 0,
            register_c: 9,
            instruction: vec![2,6], 
            output: Vec::new()};

        operate(&mut program);

        assert_eq!(program.register_b, 1);
    }

    #[test]
    fn test2() {
        let mut program = Program{register_a:10,
            register_b: 0,
            register_c: 0,
            instruction: vec![5,0,5,1,5,4], 
            output: Vec::new()};

        operate(&mut program);

        assert_eq!(program.output, vec![0,1,2]);
    }
    
     #[test]
    fn test3() {
        let mut program = Program{register_a:2024,
            register_b: 0,
            register_c: 0,
            instruction: vec![0,1,5,4,3,0], 
            output: Vec::new()};

        operate(&mut program);

        assert_eq!(program.output, vec![4,2,5,6,7,7,7,7,3,1,0]);
        assert_eq!(program.register_a, 0);
    }

    #[test]
    fn test4() {
        let mut program = Program{register_a:0,
            register_b: 29,
            register_c: 0,
            instruction: vec![1,7], 
            output: Vec::new()};

        operate(&mut program);

        assert_eq!(program.register_b, 26);
    }

    #[test]
    fn test5() {
        let mut program = Program{register_a:0,
            register_b: 2024,
            register_c: 43690,
            instruction: vec![4,0], 
            output: Vec::new()};

        operate(&mut program);

        assert_eq!(program.register_b, 44354);
    }

    #[test]
    fn test6() {
        let part1 = part1(TESTINPUT);
        assert_eq!(part1, "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test7() {
        let part2 = part2("test2",0,1);
        assert_eq!(part2, 117440);
    }

}
