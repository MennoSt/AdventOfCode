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

    let content = filereader::_input(TESTINPUT);

    let (filled, empty) = parse_data(content);
    println!("{:?}",filled);
    println!("{:?}",empty);

    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
}

fn move_file_blocks(disk_map:(Vec<i32>, Vec<i32>)) -> Vec<FileBlock> {
    let filled = disk_map.0.clone();
    let mut empty = disk_map.1.clone();
    let mut fileblock = Vec::new();

    for i in 0..filled.len() {
        fileblock.push(FileBlock {value:i as i32, count:filled[i]});
    }

    let mut i_em = 0;
    let mut insert_index = 1;
    while i_em < empty.len() {
        
        let fb_index = fileblock.len() -1;
        if fileblock[fb_index].count < empty[i_em] {
            fileblock.insert(insert_index,FileBlock{value:fileblock[fb_index].value, count:fileblock[fb_index].count});
            fileblock.pop();
            empty[i_em] -= fileblock[fb_index].count;
            insert_index+=1
        } else if  fileblock[fb_index].count == empty[i_em]{
            fileblock.insert(insert_index,FileBlock{value:fileblock[fb_index].value, count:empty[i_em]});
            fileblock.pop();
            i_em += 1;
            insert_index+=2;
        } else {
            fileblock.insert(insert_index,FileBlock{value:fileblock[fb_index].value, count:empty[i_em]});
            fileblock[fb_index+1].count = fileblock[fb_index+1].count - empty[i_em];
            i_em += 1;
            insert_index+=2;
        }


    }
    for i in 0..fileblock.len() {
        if fileblock[i].count == 0 {
            fileblock.remove(i);
        }
    }

    // let fileblock = FileBlock {value:0,count:0};
    fileblock
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
    fn test1() {
        let diskmap = (vec![1,3,5],vec![2,4]);
        let fileblock = move_file_blocks(diskmap);
        let expected_fileblock = vec![FileBlock{value:0,count:1}, FileBlock{value:2,count:2},
                                                      FileBlock{value:1,count:3}, FileBlock{value:2,count:3}];
        
        assert_eq!(fileblock, expected_fileblock);
    }
}