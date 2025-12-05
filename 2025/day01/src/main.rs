use lib::filereader;

struct Rotation {
    direction: char,
    distance: i32,
}

fn calculate_password(input: &str) -> i32 {
    let rotations = parse_data(input);
    let mut start_pos = 50;
    let mut result = 0;
    for rot in rotations {
        if rot.direction == 'L' {
            start_pos -= rot.distance;
        } else {
            start_pos += rot.distance;
        }

        start_pos %= 100;
        if start_pos == 0 {
            result += 1;
        }
    }

    result
}

fn calculate_password_p2(input: &str) -> i32 {
    let rotations = parse_data(input);
    let result = calcuate_zeros_p2(rotations, 50);
    result
}

fn calcuate_zeros_p2(rotations: Vec<Rotation>, start: i32) -> i32 {
    let mut start_pos = start;
    let mut result = 0;

    for rot in rotations {
        let current_pos = start_pos;
        if rot.direction == 'L' {
            start_pos -= rot.distance;
        } else {
            start_pos += rot.distance;
        }

        if start_pos < 0 && current_pos != 0 {
            result += 1;
        }

        let abs_start_pos = start_pos.abs();
        let mut zero_crossings = abs_start_pos / 100;

        if start_pos == 0 {
            result += 1;
            if zero_crossings >= 1 {
                zero_crossings -= 1;
            }
        }
        result += zero_crossings;

        start_pos %= 100;
        if start_pos < 0 {
            start_pos += 100;
        }
    }
    result
}

fn parse_data(input: &str) -> Vec<Rotation> {
    let contents = filereader::_input(input);
    let mut operations: Vec<Rotation> = Vec::new();
    for content in contents.lines() {
        let letter = content.chars().next().unwrap();
        let number: i32 = content[1..].parse().unwrap();
        operations.push(Rotation {
            direction: letter,
            distance: number,
        });
    }
    operations
}

fn main() {
    let part1 = calculate_password("../input/day01");
    let part2 = calculate_password_p2("../input/day01");

    assert_eq!(part1, 1021);
    assert_eq!(part2, 5933);
    println!("{} {}", part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = calculate_password("example");
        assert_eq!(result, 3);
    }

    #[test]
    fn test2() {
        let result = calculate_password_p2("example");
        assert_eq!(result, 6);
    }

    #[test]
    fn test3() {
        let rotations = vec![
            Rotation {
                direction: 'L',
                distance: 51,
            },
            Rotation {
                direction: 'R',
                distance: 1,
            },
            Rotation {
                direction: 'R',
                distance: 1,
            },
        ];

        let result = calcuate_zeros_p2(rotations, 50);
        assert_eq!(result, 2);
    }

    #[test]
    fn test4() {
        let rotations = vec![
            Rotation {
                direction: 'R',
                distance: 51,
            },
            Rotation {
                direction: 'L',
                distance: 1,
            },
        ];

        let result = calcuate_zeros_p2(rotations, 50);
        assert_eq!(result, 2);
    }

    #[test]
    fn test5() {
        let rotations = vec![Rotation {
            direction: 'R',
            distance: 350,
        }];

        let result = calcuate_zeros_p2(rotations, 50);
        assert_eq!(result, 4);
    }

    #[test]
    fn test6() {
        let rotations = vec![Rotation {
            direction: 'L',
            distance: 5,
        }];

        let result = calcuate_zeros_p2(rotations, 0);
        assert_eq!(result, 0);
    }
}
