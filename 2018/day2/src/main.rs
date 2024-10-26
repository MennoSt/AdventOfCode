
use std::fs;
use std::env;

fn main() {
    let current_dir = env::current_dir().unwrap();
    let file_path = current_dir.join("input");
    let contents = fs::read_to_string(file_path).unwrap();

    let checksum = calculate_checksum(contents.clone());
    println!("answer part one: {}",checksum);
    assert_eq!(checksum, 4920);

    let common_characters = common_characters(contents);
    println!("answer part two {}",common_characters);
    assert_eq! ("fonbwmjquwtapeyzikghtvdxl", common_characters);
}

fn common_characters(contents: String) -> String {
    let mut common_characters = String::from("");
    let mut distance_found = false;
    
    for line1 in contents.lines() {
        for line2 in contents.lines() 
        {
            if hamming_distance(line1, line2) == 1 && !distance_found {
                distance_found = true;
                for i in 0..line1.len() {
                    if line1.chars().nth(i) == line2.chars().nth(i) 
                        {
                        let commonchar = line1.chars().nth(i).unwrap();
                        common_characters.push(commonchar);
                        }
                    }
            }
        }
    }
    return common_characters;
}

fn hamming_distance(str1: &str, str2: &str) -> usize {
    if str1.len() != str2.len() {
        panic!("Strings must be of equal length");
    }

    let mut distance = 0;

    for i in 0..str1.len() {
        if str1.chars().nth(i) != str2.chars().nth(i) {
            distance += 1;
        }
    }
    distance
}

fn calculate_checksum(contents: String) -> i32 {
    let mut threes =0;
    let mut twos = 0;
    for line in contents.lines() {
        let hasmap = char_occurrences(line);
        if is_char_there(hasmap.clone(), 3) {
            threes += 1;
        }
        if is_char_there(hasmap.clone(), 2) {
            twos += 1;
        }
    }
    let checksum = twos * threes;
    checksum
}

fn is_char_there(counts: std::collections::HashMap<char, usize>, n: usize) -> bool{
    for (_ch, count) in counts.into_iter() {
        if count == n {
            return true;
        }
    }
    return false;
}

fn char_occurrences(s: &str) -> std::collections::HashMap<char, usize> {
    let mut counts = std::collections::HashMap::new();
    for ch in s.chars() {
        *counts.entry(ch).or_insert(0) += 1;
    }
    counts
}

