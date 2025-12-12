use lib::filereader;
use lib::utils;
use std::time::Instant;

static INPUT: &str = "../input/dayxx";

fn parse_data(input: &str) {}

fn p1(input: &str) -> i128 {
    0
}

fn p2(input: &str) -> i128 {
    0
}

fn main() {
    let start = Instant::now();

    let part1 = p1(INPUT);
    let part2 = p2(INPUT);

    utils::answer((part1, 0), (part2, 0));

    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT_EXAMPLE: &str = "example";

    #[test]
    fn test1() {
        let result = p1(0);
        assert_eq!(result, 0);
    }
}
