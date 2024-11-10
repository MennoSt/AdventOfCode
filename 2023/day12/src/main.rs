use lib::filereader;

use itertools::Itertools;

struct Spring {
    record: String,
    numbers: Vec<i32>,
}

fn main()
{
    let part1 = part1("../input/day12");
    assert_eq!(part1, 7857);
    println!("{}",part1);

}

fn part1(input:&str) -> i32{
    let springs = parse_data(input);
    
    let mut arrangement_sum = 0;
    for spring in springs {
        arrangement_sum += calculate_arrangements(&spring);
    }

    arrangement_sum
}

fn parse_data(input:&str) -> Vec<Spring> {
    let contents = filereader::_input(input);
    let mut spring_collection:Vec<Spring> = Vec::new();
    
    for line in contents.lines() {
        let text:Vec<&str> =line.split_whitespace().collect();
        let record = text[0];
        let numbers:Vec<i32>= text[1]
            .split(',')
            .map(|s| s.parse::<i32>().expect("Not a valid integer"))
            .collect();

        spring_collection.push(Spring{ record:record.to_string(), numbers:numbers});
    }
    spring_collection
}

fn replace_nth_character(s: &mut String, n: usize, new_char: char) {
    if n < s.len() {
        let char_len = new_char.len_utf8();  // Account for multi-byte characters
        s.replace_range(n..n + char_len, &new_char.to_string());
    }
}

fn calculate_arrangements(spring: &Spring) -> i32 {
    let sum:i32 = spring.numbers.iter().sum();
    let hashes = spring.record.chars().filter(|&c| c == '#').count() as i32;
    let unknowns: Vec<usize>  = spring.record.chars()
                                  .enumerate()
                                  .filter(|&(_, c)| c == '?')
                                  .map(|(i, _)| i) 
                                  .collect();
    let hashes_to_place = (sum - hashes) as usize;

    let mut arrangements = 0;
    let combination = unknowns.iter().combinations(hashes_to_place);
    for combination in combination {
        let mut mutated_record = spring.record.clone();
        for i in 0..hashes_to_place {
            let one =combination[i];
            replace_nth_character(&mut mutated_record, *one, '#');
        }
        let mut counter = 0;
        let mut count_vec:Vec<i32> = Vec::new();

        for char in mutated_record.chars(){
            if char == '#' {
                counter += 1;
            } else if counter != 0 {
                count_vec.push(counter);
                counter = 0;
            }
        }

        if counter > 0 {
            count_vec.push(counter);
        }

        if count_vec == spring.numbers {
            arrangements += 1;
        }
    }

    return arrangements;
}

#[cfg(test)]
mod tests {
    use super::*;

    mod part2 {
        use super::*;
        #[test]
        fn test1() {
            let spring = Spring {
                record:"???.###".to_string(),
                numbers:vec![1,1,3]};
    
            assert_eq!(calculate_arrangements(&spring), 1);
        }
    }
    
    mod part1 {
        use super::*;
        #[test]
        fn test1() {
            let spring = Spring {
                record:"???.### 1,1,3".to_string(),
                numbers:vec![1,1,3]};
    
            assert_eq!(calculate_arrangements(&spring), 1);
        }
    
        #[test]
        fn test2() {
            let spring = Spring {
                record:".??..??...?##.".to_string(),
                numbers:vec![1,1,3]};
    
            assert_eq!(calculate_arrangements(&spring), 4);
        }
    
        #[test]
        fn test3() {
            let spring = Spring {
                record:"?#?#?#?#?#?#?#?".to_string(),
                numbers:vec![1,3,1,6]};
    
            assert_eq!(calculate_arrangements(&spring), 1);
        }
    
        #[test]
        fn test4() {
            let spring = Spring {
                record:"????.#...#...".to_string(),
                numbers:vec![4,1,1]};
    
            assert_eq!(calculate_arrangements(&spring), 1);
        }
    
        #[test]
        fn test5() {
            let spring = Spring {
                record:"????.######..#####.".to_string(),
                numbers:vec![1,6,5]};
    
            assert_eq!(calculate_arrangements(&spring), 4);
        }
    
        #[test]
        fn test6() {
            let spring = Spring {
                record:"?###????????".to_string(),
                numbers:vec![3,2,1]};
    
            assert_eq!(calculate_arrangements(&spring), 10);
        }
        
        #[test]
        fn test7() {
    
            assert_eq!(part1("testinput/test1"), 21);
        }
    }

}