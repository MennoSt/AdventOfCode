use lib::{filereader, utils};

fn parse_data(input: &str) -> Vec<String> {
    let contents = filereader::_input(input);
    let mut data: Vec<String> = Vec::new();

    for c in contents.lines() {
        data.push(c.to_string());
    }
    data
}

fn largest_joltage(input: &str, index: &mut i64, joltage: &mut String) {
    for i in ('1'..='9').rev() {
        if joltage.len() == 2 {
            return;
        }

        let positions: Vec<usize> = input
        .char_indices()                  // iterate over (index, char)
        .filter(|(_, c)| *c == i)     // keep only i 
        .map(|(i, _)| i)                 // take the index
        .collect();

        if positions.len() > 0 {
            for p in positions{
                if p as i64 > *index {
                    let last_index_and_size_zero = p == input.len() - 1 && joltage.len() == 0;
                    if !last_index_and_size_zero {
                        joltage.push(i);
                        *index = p as i64;
                        largest_joltage(input, index, joltage);
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
        largest_joltage(&d, &mut -1, &mut joltage);
        let j:i64 = joltage.parse().unwrap();
        sum += j;
    }
    sum
}

fn main() {
    let part1 = p1("../input/day03");

    // let p1 = answerp1("../input/day02");
    // let p2 = answerp2("../input/day02");
    assert_eq!(part1,17087);
    println!("{}", part1);
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
        largest_joltage("987654321111111", &mut -1, &mut joltage);
        assert_eq!(joltage, "98");
    }

    #[test]
    fn test2() {
        let mut joltage = "".to_string();
        largest_joltage("811111111111119", &mut -1, &mut joltage);
        assert_eq!(joltage, "89");
    }

    #[test]
    fn test3() {
        let mut joltage = "".to_string();
        largest_joltage("234234234234278", &mut -1, &mut joltage);
        assert_eq!(joltage, "78");
    }

    #[test]
    fn test4() {
        let mut joltage = "".to_string();
        largest_joltage("818181911112111", &mut -1, &mut joltage);
        assert_eq!(joltage, "92");
    }
    
    #[test]
    fn test5() {
        let mut joltage = "".to_string();
        largest_joltage("9191", &mut -1, &mut joltage);
        assert_eq!(joltage, "99");
    }

        #[test]
    fn test7() {
        let mut joltage = "".to_string();
        largest_joltage("1188", &mut -1, &mut joltage);
        assert_eq!(joltage, "88");
    }
    
    #[test]
    fn test6() {
        let sum = p1("example");
        assert_eq!(sum, 357);
    }
}

