use lib::filereader;

fn parse_data(input: &str) -> Vec<(i64, i64)> {
    let contents = filereader::_input(input);
    let pairs: Vec<(i64, i64)> = contents
        .split(',')
        .map(|p| {
            let nums: Vec<i64> = p.split('-').map(|x| x.trim().parse().unwrap()).collect();
            (nums[0], nums[1])
        })
        .collect();

    println!("{:?}", pairs);

    pairs
}

fn calculate_invalid_ids(start: i64, end: i64) -> i64 {
    let mut invalid_ids_sum = 0;
    for n in start..=end {
        let s = n.to_string();
        let len = s.len();
        let half = len / 2;
        let left = s[..half].to_string();
        let right = s[half..].to_string();

        if left.len() != right.len() {
            continue;
        }

        if left == right {
            invalid_ids_sum += n;
            println!("invalid id: {}", n);
        }
    }
    invalid_ids_sum
}

fn answerp1(input: &str) -> i64 {
    let pairs = parse_data(input);
    let mut sum = 0;
    for pair in pairs {
        sum += calculate_invalid_ids(pair.0, pair.1);
    }
    sum
}
fn main() {
    let p1 = answerp1("../input/day02");
    println!("{}", p1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let invalid_ids = calculate_invalid_ids(11, 22);
        assert_eq!(invalid_ids, 33);
    }

    #[test]
    fn test2() {
        let invalid_ids = calculate_invalid_ids(38593856, 38593862);
        assert_eq!(invalid_ids, 38593859);
    }

    #[test]
    fn test3() {
        let invalid_ids = calculate_invalid_ids(95, 115);
        assert_eq!(invalid_ids, 99);
    }

    #[test]
    fn test4() {
        let invalid_ids = answerp1("example");
        assert_eq!(invalid_ids, 1227775554);
    }
}
