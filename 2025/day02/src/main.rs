use lib::filereader;

fn parse_data(input: &str) -> Vec<(i32, i32)> {
    let contents = filereader::_input(input);
    let mut vec: Vec<(i32, i32)> = Vec::new();

    let mut it = contents.split('-').map(|p| p.trim().parse::<i32>());
    let a = it.next();
    let b = it.next();
    vec
}

fn main() {
    let pair = parse_data("../input/day02");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        // let result = calculate_password("/home/menno/Alten/AdventOfCode/2025/day01/example");
        // assert_eq!(result, 3);
    }
}
