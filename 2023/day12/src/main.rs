use lib::filereader;

use itertools::Itertools;

struct Spring {
    record: String,
    numbers: Vec<i128>,
}

fn main()
{
    // let part1 = part1("../input/day12");
    // assert_eq!(part1, 7857);
    // println!("{}",part1);

    let part2 = part2("../input/day12");
    assert_eq!(part2, 7857);
    println!("{}",part2);

}

fn part1(input:&str) -> i128{
    let springs = parse_data(input);
    
    let mut arrangement_sum = 0;
    for spring in springs {
        arrangement_sum += calculate_arrangements(&spring);
    }

    arrangement_sum
}

fn part2(input:&str) -> i128{
    let springs = parse_data(input);
    
    let mut arrangement_sum = 0;
    let mut it =0;
    for spring in springs {
        let arrangements = calculate_arrangements_extended(&spring);
        println!("{} {}",it, arrangements);
        it+=1;
        arrangement_sum += arrangements;
    }

    arrangement_sum
}

fn calculate_arrangements_extended(spring: &Spring) -> i128 {
    let mut arrangements = 0;
    let mut extended_record= spring.record.clone();
    let mut extended_spring = Spring{record:extended_record, numbers:spring.numbers.clone()};

    let part1 = calculate_arrangements(spring);

    if spring.record.chars().last() != Some('#') {
        extended_record = format!("{}{}{}", spring.record, '?', spring.record);
        let mut extended_numbers = spring.numbers.clone();
        extended_numbers.extend(extended_numbers.clone());
        extended_spring = Spring{record:extended_record, numbers:extended_numbers};

        let extended = calculate_arrangements(&extended_spring)/part1;
        arrangements = part1 * extended.pow(4);
    } else {
        arrangements = part1 * calculate_arrangements(&extended_spring).pow(4);
    }

    arrangements
}

fn parse_data(input:&str) -> Vec<Spring> {
    let contents = filereader::_input(input);
    let mut spring_collection:Vec<Spring> = Vec::new();
    
    for line in contents.lines() {
        let text:Vec<&str> =line.split_whitespace().collect();
        let record = text[0];
        let numbers:Vec<i128>= text[1]
            .split(',')
            .map(|s| s.parse::<i128>().expect("Not a valid integer"))
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

fn calculate_arrangements(spring: &Spring) -> i128 {
    let sum:i128 = spring.numbers.iter().sum();
    let hashes:Vec<usize> = spring.record.chars()
    .enumerate()
    .filter(|&(_, c)| c == '#')
    .map(|(i, _)| i)
    .collect();
    let unknowns: Vec<usize>  = spring.record.chars()
                                  .enumerate()
                                  .filter(|&(_, c)| c == '?')
                                  .map(|(i, _)| i) 
                                  .collect();

    // let hash_or_unknown: Vec<usize>  = spring.record.chars()
    //     .enumerate()
    //     .filter(|&(_, c)| c == '?'|| c == '#')
    //     .map(|(i, _)| i) 
    //     .collect();

    let hashes_to_place = sum - hashes.len() as i128;

    let mut arrangements = 0;
    let combinations = unknowns.iter().combinations(hashes_to_place as usize);
    // for combination in combinations.clone() {
    //     println!("{:?}",combination);
    // }

    for combination in combinations {
        let mut mutated_record = spring.record.clone();
        for i in 0..hashes_to_place as usize {
            let one =combination[i];
            replace_nth_character(&mut mutated_record, *one, '#');
        }
            let mut counter = 0;
            let mut count_vec:Vec<i128> = Vec::new();

        for char in mutated_record.chars() {
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
                record:"???????##?????#?#?".to_string(),
                numbers:vec![9,6]};
    
            // assert_eq!(calculate_arrangements(&spring), 5);
            assert_eq!(calculate_arrangements_extended(&spring), 20480);
        }


        #[test]
        fn test2() {
            let spring = Spring {
                record:"#??#???????????.???".to_string(),
                numbers:vec![14,2]};
            
            assert_eq!(calculate_arrangements(&spring), 2);
        }

        #[test]
        fn test3() {
            let spring = Spring {
                record:"#??#???????????.???".to_string(),
                numbers:vec![14,2]};
            
            assert_eq!(calculate_arrangements(&spring), 2);
        }


        
    }
    
    mod part1 {
        use super::*;
        #[test]
        fn test1() {
            let spring = Spring {
                record:"???.###".to_string(),
                numbers:vec![1,1,3]};
    
            assert_eq!(calculate_arrangements(&spring), 1);
            // assert_eq!(calculate_arrangements_extended(&spring), 1);
        }
    
        #[test]
        fn test2() {
            let spring = Spring {
                record:".??..??...?##.".to_string(),
                numbers:vec![1,1,3]};
    
            assert_eq!(calculate_arrangements(&spring), 4);
            assert_eq!(calculate_arrangements_extended(&spring), 16384);
        }
    
        #[test]
        fn test3() {
            let spring = Spring {
                record:"?#?#?#?#?#?#?#?".to_string(),
                numbers:vec![1,3,1,6]};
    
            assert_eq!(calculate_arrangements(&spring), 1);
            assert_eq!(calculate_arrangements_extended(&spring), 1);
        }
    
        #[test]
        fn test4() {
            let spring = Spring {
                record:"????.#...#...".to_string(),
                numbers:vec![4,1,1]};
    
            assert_eq!(calculate_arrangements(&spring), 1);
            assert_eq!(calculate_arrangements_extended(&spring), 16);
        }
    
        #[test]
        fn test5() {
            let spring = Spring {
                record:"????.######..#####.".to_string(),
                numbers:vec![1,6,5]};
    
            assert_eq!(calculate_arrangements(&spring), 4);
            assert_eq!(calculate_arrangements_extended(&spring), 2500);
        }
    
        #[test]
        fn test6() {
            let spring = Spring {
                record:"?###????????".to_string(),
                numbers:vec![3,2,1]};
    
            // assert_eq!(calculate_arrangements(&spring), 10);
            assert_eq!(calculate_arrangements_extended(&spring), 506250);
        }

        #[test]
        fn test8() {
    
            assert_eq!(part1("testinput/test1"), 21);
        }
    }

}

// .??..??...?##.?.??..??...?##.?.??..??...?##.?.??..??...?##.?.??..??...?##.  