
use std::fs;
use std::env;
use std::vec;
use std::collections::HashMap;

#[derive(Clone, Copy)]
struct Fabric {
    id: i32,
    xmin: i32,
    ymin: i32,
    xmax: i32,
    ymax: i32
}



fn main() {
    let current_dir = env::current_dir().unwrap();
    let file_path_ex = current_dir.join("testinput2018day3");
    let contents_ex = fs::read_to_string(file_path_ex).unwrap();

    let file_path = current_dir.join("input");
    let contents = fs::read_to_string(file_path).unwrap();

    let overlaps = calculate_overlaps(&contents_ex);
    assert_eq!(overlaps,4);

    let answer_part1 = calculate_overlaps(&contents);
    assert_eq!(answer_part1, 107820);
    print!("answer part1 {}",answer_part1);


    let vect_fabric = parse_fabric(&contents_ex);
    let answer_part2_ex = answer_part2(vect_fabric);
    assert_eq!(answer_part2_ex, 3);

    let vect_fabric = parse_fabric(&contents);
    let answer_part2 = answer_part2(vect_fabric);
    // assert_eq!(answer_part2, 661); //answer is guessed from 8 answers
    print!("answer part2 {}",answer_part2);
}

fn answer_part2(vect_fabric: Vec<Fabric>) ->i32 {
    let mut vec_id=0;
    for vec in &vect_fabric{
        if !has_vec_overlaps(&vect_fabric, vec)
        {
            print!("answer part2 {}\n",vec.id);
            vec_id = vec.id;
        }
    }
    return vec_id;
}

fn has_vec_overlaps(vect_fabric: &Vec<Fabric>, vec: &Fabric) -> bool{
    let mut _vec_cmp_copy:Fabric;
    for vec_cmp in vect_fabric{
        _vec_cmp_copy=*vec_cmp;
        if vec.id != vec_cmp.id {
            if in_range(vec, vec_cmp)|| in_range(vec_cmp,vec)
            {
                return true;
            }
        }
    }
    return false;
}

fn in_range(vec: &Fabric, vec_cmp: &Fabric) -> bool {

    if ((vec.xmin >= vec_cmp.xmin && vec.xmin <= vec_cmp.xmax) ||
        (vec.xmax >= vec_cmp.xmin && vec.xmax <= vec_cmp.xmax)) &&
        (vec.ymin >= vec_cmp.ymin && vec.ymin <= vec_cmp.ymax ||
        vec.ymax >= vec_cmp.ymin && vec.ymax <= vec_cmp.ymax) 
    {
        return true;
    } else 
    {
        return false;
    }
}

fn calculate_overlaps(contents: &str) -> i32 {
    let vect_coord = calculate_coordinates(contents);
    let counts = count_occurrences(vect_coord);
    
    let mut overlaps = 0;
    for count in counts {
        if count.1 > 1 {
            overlaps += 1;
        } 
    }
    overlaps
}

fn count_occurrences(data: Vec<String>) -> HashMap<String, u32> {
    let mut counts = HashMap::new();

    for num in data {
        let count = counts.entry(num).or_insert(0);
        *count += 1;
    }

    counts
}


fn calculate_coordinates(contents: &str) -> Vec<String> {
    let vect_fabric = parse_fabric(contents);
    let mut vect_coord = Vec::new();
    for fabric in &vect_fabric{
        for i in fabric.xmin..=fabric.xmax {
            for j in fabric.ymin..=fabric.ymax {
                let string = i.to_string() + "," + &j.to_string();
                vect_coord.push(string);
            }
        }
    }
    vect_coord
}


fn parse_fabric(contents: &str) -> Vec<Fabric> {
    let mut vect_fabric: Vec<Fabric> = Vec::new();
    for line in contents.lines() {
        let line_empty = line.replace("#", "");
        let line_empty = remove_empty_chars(&line_empty);
        let vect: Vec<&str> = line_empty.split(|c| c == '@' || c == ',' || c == ':'|| c == 'x').collect();

        let xmax = vect[1].parse::<i32>().unwrap() + vect[3].parse::<i32>().unwrap()-1;
        let ymax = vect[2].parse::<i32>().unwrap() + vect[4].parse::<i32>().unwrap()-1;
        let fabric = Fabric{
            id: vect[0].parse::<i32>().unwrap(),
            xmin: vect[1].parse::<i32>().unwrap(),
            ymin: vect[2].parse::<i32>().unwrap(),
            xmax: xmax,
            ymax: ymax};
    
        vect_fabric.push(fabric);
    }
    vect_fabric
}

fn remove_empty_chars(input: &str) -> String {
    let mut result = String::new();
    for c in input.chars() {
        if !c.is_whitespace() {
            result.push(c);
        }
    }
    result
}
