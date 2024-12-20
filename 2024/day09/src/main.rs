use lib::filereader;
use lib::utils;
use std::time::Instant;

static INPUT: &str = "../input/day09";
static TESTINPUT: &str = "test1";

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
struct FileBlock {
    value:i32,
    count:i32
}

fn main() {
    let start = Instant::now();

    utils::answer((calculate_checksum_p1(INPUT), 6398252054886),
        (calculate_checksum_p2(INPUT), 6415666220005));

    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
}

fn calculate_checksum_p1(input:&str) -> i128 {
    let content = filereader::_input(input);
    let diskmap = parse_data(content);
    let fileblocks = move_file_blocks_p1(diskmap);

    let mut position:i128 = 0;
    let mut checksum:i128 = 0;
    for block in fileblocks {
        for _ in 0..block.count {
            checksum += block.value as i128 * position as i128;
            position += 1;
        }
    }
    checksum
}

fn move_file_blocks_p1(disk_map:(Vec<i32>, Vec<i32>)) -> Vec<FileBlock> {
    let filled = disk_map.0.clone();
    let mut empty = disk_map.1.clone();
    let mut fileblock = Vec::new();

    for i in 0..filled.len() {
        fileblock.push(FileBlock {value:i as i32, count:filled[i]});
    }

    let mut i_em = 0;
    let mut insert_index = 1;
    
    while i_em < (empty.len() -1) && insert_index < (fileblock.len()-1) {
        
        let fb_index = fileblock.len() -1;
        if fileblock[fb_index].count < empty[i_em] {
            fileblock.insert(insert_index,FileBlock{value:fileblock[fb_index].value, count:fileblock[fb_index].count});
            empty[i_em] -= fileblock[fb_index+1].count;
            fileblock.pop();
            insert_index+=1
        } else if  fileblock[fb_index].count == empty[i_em] {
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

    let mut index = 0;
    while index < fileblock.len(){
        if fileblock[index].count == 0 {
            fileblock.remove(index);
        }
        index+=1;
    }

    // let fileblock = FileBlock {value:0,count:0};
    fileblock
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
    fn test1() {
        let diskmap = (vec![1,3,5],vec![2,4]);
        let fileblock = move_file_blocks_p1(diskmap);
        let expected_fileblock = vec![FileBlock{value:0,count:1}, FileBlock{value:2,count:2},
                                                      FileBlock{value:1,count:3}, FileBlock{value:2,count:3}];
        
        assert_eq!(fileblock, expected_fileblock);
    }
    
    #[test]
    fn test2() {
        let checksum = calculate_checksum_p1(TESTINPUT); 
        assert_eq!(checksum, 1928);
    }

    #[test]
    fn test3() {
        let checksum = calculate_checksum_p2(TESTINPUT); 
        assert_eq!(checksum, 2858);
    }
    
}
