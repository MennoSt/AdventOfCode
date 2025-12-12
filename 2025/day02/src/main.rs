use lib::{filereader, utils};
use std::time::Instant;

static INPUT: &str = "../input/day02";

fn parse_data(input: &str) -> Vec<(i64, i64)> {
    let contents = filereader::_input(input);
    let pairs: Vec<(i64, i64)> = contents
        .split(',')
        .map(|p| {
            let nums: Vec<i64> = p.split('-').map(|x| x.trim().parse().unwrap()).collect();
            (nums[0], nums[1])
        })
        .collect();
    pairs
}

fn calculate_invalid_ids_p1(start: i64, end: i64) -> i64 {
    let mut invalid_ids_sum = 0;
    for n in start..=end {
        if is_invalid_id_p1(n) {
            invalid_ids_sum += n;
        }
    }
    invalid_ids_sum
}

fn is_invalid_id_p1(number: i64) -> bool {
    let n = number;
    let s = n.to_string();
    let len = s.len();
    let half = len / 2;
    let left = &s[..half];
    let right = &s[half..];

    if left.len() != right.len() {
        return false;
    }
    if left == right {
        return true;
    }
    return false;
}

fn calculate_invalid_ids_p2(start: i64, end: i64) -> i64 {
    let mut invalid_ids_sum = 0;
    for n in start..=end {
        if is_invalid_id_p2(n) {
            invalid_ids_sum += n;
        }
    }
    invalid_ids_sum
}

fn split_into_equal_parts(s: &str, n: usize) -> Vec<&str> {
    let len = s.len();
    if len % n != 0 {
        panic!("String length is not divisible by n");
    }
    let part_len = len / n;

    (0..n)
        .map(|i| &s[i * part_len..(i + 1) * part_len])
        .collect()
}

fn is_invalid_id_p2(number: i64) -> bool {
    let n = number as usize;
    let s = n.to_string();
    let len = s.len();

    if is_invalid_id_p1(number) {
        return true;
    }

    if len == 1 {
        return false;
    }

    if utils::is_odd(len as i128) {
        let v: Vec<&str> = split_into_equal_parts(&s, len);
        let all_equal = v.iter().all(|s| s == &v[0]);
        if all_equal {
            return true;
        }
        if len % 3 == 0 && len >= 6 {
            let part_len = len / 3;
            let v: Vec<&str> = split_into_equal_parts(&s, part_len);
            let all_equal = v.iter().all(|s| s == &v[0]);
            if all_equal {
                return true;
            }
        }
    } else {
        let mut parts = len;
        while parts >= 2 {
            let v: Vec<&str> = split_into_equal_parts(&s, parts);
            let all_equal = v.iter().all(|s| s == &v[0]);
            if all_equal {
                return true;
            } else {
                parts /= 2;
            }
        }
    }

    return false;
}

fn p1(input: &str) -> i64 {
    let pairs = parse_data(input);
    let mut sum = 0;
    for pair in pairs {
        sum += calculate_invalid_ids_p1(pair.0, pair.1);
    }
    sum
}
fn p2(input: &str) -> i64 {
    let pairs = parse_data(input);
    let mut sum = 0;
    for pair in pairs {
        sum += calculate_invalid_ids_p2(pair.0, pair.1);
    }
    sum
}

fn main() {
    let start = Instant::now();

    let part1 = p1(INPUT);
    let part2 = p2(INPUT);

    utils::answer((part1, 54641809925), (part2, 73694270688));

    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT_EXAMPLE: &str = "example";

    #[test]
    fn test1() {
        let invalid_ids = calculate_invalid_ids_p1(11, 22);
        assert_eq!(invalid_ids, 33);
    }

    #[test]
    fn test2() {
        let invalid_ids = calculate_invalid_ids_p1(38593856, 38593862);
        assert_eq!(invalid_ids, 38593859);
    }

    #[test]
    fn test3() {
        let invalid_ids = calculate_invalid_ids_p1(95, 115);
        assert_eq!(invalid_ids, 99);
    }

    #[test]
    fn test4() {
        let invalid_ids = p1(INPUT_EXAMPLE);
        assert_eq!(invalid_ids, 1227775554);
    }

    #[test]
    fn test5() {
        let invalid = is_invalid_id_p2(102);
        assert_eq!(invalid, false);
    }

    #[test]
    fn test6() {
        let invalid = is_invalid_id_p2(446446);
        assert_eq!(invalid, true);
    }

    #[test]
    fn test7() {
        let invalid = is_invalid_id_p2(2);
        assert_eq!(invalid, false);
    }

    #[test]
    fn test8() {
        let invalid_ids = p2("example");
        assert_eq!(invalid_ids, 4174379265);
    }
}
