use itertools::rev;
use lib::filereader;
use lib::utils;
use lib::utils::*;
use lib::grid::Grid;
use std::f32::consts::E;
use std::fs::File;
use std::time::Instant;
use itertools::Itertools;
use std::fmt::Debug;

static INPUT: &str = "../input/day09";
static TESTINPUT: &str = "test1";

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
struct FileBlock {
    value:i32,
    count:i32
}

fn main() {
    let start = Instant::now();

    let part1 = calculate_checksum_p2(INPUT);
    println!("{}",part1);
    assert_eq!(part1,6398252054886);

    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
}

fn move_file_blocks_p2(disk_map:(Vec<i32>, Vec<i32>)) -> Vec<FileBlock> {
    let filled = disk_map.0.clone();
    let empty = disk_map.1.clone();
    let mut fileblock = Vec::new();
    let mut filled_fb = Vec::new();

    for i in 0..filled.len() {
        fileblock.push(FileBlock {value:i as i32, count: filled[i]});
        filled_fb.push(FileBlock {value:i as i32, count: filled[i]});
        if i < empty.len(){
            fileblock.push(FileBlock {value: -1,count: empty[i]});
        }
    }

    filled_fb.reverse();
    for i in 0..filled_fb.len() {
        let mut j = 0;
        while j < fileblock.len(){
            if fileblock[j].value == -1 {
                if filled_fb[i].count <= fileblock[j].count{
                    let count1 = fileblock[j].count;
                    let count2 = filled_fb[i].count; 
                    fileblock[j].count -= filled_fb[i].count;
                    fileblock.insert(j,FileBlock{value:filled_fb[i].value, count:filled_fb[i].count});
                    
                    let mut found_first = false;
                    let mut remove_index=0;
                    for k in 0..fileblock.len() {
                        if fileblock[k].value == filled_fb[i].value {
                            if found_first {
                                remove_index=k;
                            }
                            found_first = true;
                        }
                    }

                    fileblock[remove_index]=FileBlock{value:-1, count:filled_fb[i].count};
                    break;
                }
            }
            j += 1;
        }
    }
    fileblock
}

fn calculate_checksum_p2(input:&str) -> i128 {
    let content = filereader::_input(input);
    let diskmap = parse_data(content);
    let fileblocks = move_file_blocks_p2(diskmap);

    let mut position:i128 = 0;
    let mut checksum:i128 = 0;
    for block in fileblocks {
        for _ in 0..block.count {
            if block.value != -1 {
                checksum += block.value as i128 * position as i128;
            }
            position += 1;
        }
    }
    checksum
}

fn parse_data(content: String) -> (Vec<i32>, Vec<i32>) {
    let mut filled = Vec::new();
    let mut empty = Vec::new();
    let mut toggle = true;
    for content  in content.lines() {
        for char in content.chars() {
            let digit = char.to_digit(10).unwrap() as i32;
            if toggle {
                filled.push(digit);
                toggle = false;
            } else {
                empty.push(digit);
                toggle=true;
            }
        }
    }

    (filled,empty)
}
    
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test3() {
        let checksum = calculate_checksum_p2(TESTINPUT); 
        assert_eq!(checksum, 2858);
    }
}
