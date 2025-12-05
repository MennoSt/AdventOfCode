use lib::{filereader, utils};

fn parse_data(input: &str) -> Vec<String> {
    let contents = filereader::_input(input);
    let mut data: Vec<String> = Vec::new();

    for c in contents.lines() {
        data.push(c.to_string());
    }
    data
}

fn largest_joltage(
    input: &str,
    added_indices: &mut Vec<usize>,
    joltage: &mut String,
    length: usize,
) {
    for i in ('1'..='9').rev() {
        if joltage.len() == length {
            return;
        }

        let positions: Vec<usize> = input
            .char_indices() // iterate over (index, char)
            .filter(|(_, c)| *c == i) // keep only i
            .map(|(i, _)| i) // take the index
            .collect();

        if positions.len() > 0 {
            for p in positions {
                if !added_indices.contains(&p) && added_indices.iter().all(|&n| p > n) {
                    let last_index_and_size_zero = p == input.len() - 1 && joltage.len() == 0;
                    if !last_index_and_size_zero {
                        joltage.push(i);
                        added_indices.push(p);
                        largest_joltage(input, added_indices, joltage, length);
                        break;
                    }
                }
            }
        }
    }
}

fn p1(input: &str) -> i64 {
    let data = parse_data(input);
    let mut sum = 0;

    for d in data {
        let mut joltage = "".to_string();
        let mut added_indices: Vec<usize> = Vec::new();
        largest_joltage(&d, &mut added_indices, &mut joltage, 2);
        let j: i64 = joltage.parse().unwrap();
        println!("{}", j);
        sum += j;
    }
    sum
}

fn p2(input: &str) -> i64 {
    let data = parse_data(input);
    let mut sum = 0;

    for d in data {
        let mut joltage = "".to_string();
        let mut added_indices: Vec<usize> = Vec::new();
        largest_joltage(&d, &mut added_indices, &mut joltage, 12);
        let j: i64 = joltage.parse().unwrap();
        println!("{}", j);
        sum += j;
    }
    sum
}

fn main() {
    let part1 = p1("../input/day03");
    let part2 = p2("../input/day03");

    // let p1 = answerp1("../input/day02");
    // let p2 = answerp2("../input/day02");
    assert_eq!(part1, 17087);
    println!("{}", part1);
    println!("{}", part2);
    // println!("{}", p2);
    // println!("{}", p2);
    // assert_eq!(p1, 54641809925);
    // assert_eq!(p2, 73694270688);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut joltage = "".to_string();
        let mut added_indices: Vec<usize> = Vec::new();
        largest_joltage("987654321111111", &mut added_indices, &mut joltage, 2);
        assert_eq!(joltage, "98");
    }

    // #[test]
    // fn test2() {
    //     let mut joltage = "".to_string();
    //     let mut added_indices:Vec<i64> = Vec::new();
    //     largest_joltage("811111111111119", &mut added_indices, &mut joltage);
    //     assert_eq!(joltage, "89");
    // }

    // #[test]
    // fn test3() {
    //     let mut joltage = "".to_string();
    //     largest_joltage("234234234234278", &mut -1, &mut joltage);
    //     assert_eq!(joltage, "78");
    // }

    #[test]
    fn test4() {
        let mut joltage = "".to_string();
        let mut added_indices: Vec<usize> = Vec::new();
        largest_joltage("811111111111119", &mut added_indices, &mut joltage, 12);
        assert_eq!(joltage, "811111111119");
    }

    // #[test]
    // fn test5() {
    //     let mut joltage = "".to_string();
    //     largest_joltage("9191", &mut -1, &mut joltage);
    //     assert_eq!(joltage, "99");
    // }

    //     #[test]
    // fn test7() {
    //     let mut joltage = "".to_string();
    //     largest_joltage("1188", &mut -1, &mut joltage);
    //     assert_eq!(joltage, "88");
    // }

    #[test]
    fn test6() {
        let sum = p1("example");
        assert_eq!(sum, 357);
    }
    #[test]
    fn test7() {
        let sum = p2("example");
        assert_eq!(sum, 3121910778619);
    }
}
