use lib::filereader;
use lib::utils;
use std::time::Instant;

static INPUT: &str = "../input/day01";

struct Rotation {
    direction: char,
    distance: i32,
}

fn parse_data(input: &str) -> Vec<Rotation> {
    filereader::_input(input)
        .lines()
        .map(|line| Rotation {
            direction: line.as_bytes()[0] as char,
            distance: line[1..].parse().unwrap(),
        })
        .collect()
}

fn update_position(position: &mut i32, rotation: Rotation) {
    if rotation.direction == 'L' {
        *position -= rotation.distance;
    } else {
        *position += rotation.distance;
    }
}

fn calculate_crossings(rotations: Vec<Rotation>, start: i32) -> i32 {
    let mut pos = start;
    let mut result = 0;

    for rot in rotations {
        let prev_pos = pos;
        update_position(&mut pos, rot);

        if pos < 0 && prev_pos != 0 {
            result += 1;
        }

        let mut crossings = pos.abs() / 100;

        if pos == 0 {
            result += 1;
            if crossings >= 1 {
                crossings -= 1;
            }
        }

        result += crossings;

        pos %= 100;
        if pos < 0 {
            pos += 100;
        }
    }
    result
}

fn p1(input: &str) -> i32 {
    let rotations = parse_data(input);
    let mut start_pos = 50;
    let mut result = 0;

    for rot in rotations {
        update_position(&mut start_pos, rot);

        start_pos %= 100;

        if start_pos == 0 {
            result += 1;
        }
    }

    result
}

fn p2(input: &str) -> i32 {
    let rotations = parse_data(input);
    let result = calculate_crossings(rotations, 50);
    result
}

fn main() {
    let start = Instant::now();

    let part1 = p1(INPUT);
    let part2 = p2(INPUT);

    utils::answer((part1, 1021), (part2, 5933));

    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT_EXAMPLE: &str = "example";

    #[test]
    fn test1() {
        let result = p1(INPUT_EXAMPLE);
        assert_eq!(result, 3);
    }

    #[test]
    fn test2() {
        let result = p2(INPUT_EXAMPLE);
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

        let result = calculate_crossings(rotations, 50);
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

        let result = calculate_crossings(rotations, 50);
        assert_eq!(result, 2);
    }

    #[test]
    fn test5() {
        let rotations = vec![Rotation {
            direction: 'R',
            distance: 350,
        }];

        let result = calculate_crossings(rotations, 50);
        assert_eq!(result, 4);
    }

    #[test]
    fn test6() {
        let rotations = vec![Rotation {
            direction: 'L',
            distance: 5,
        }];

        let result = calculate_crossings(rotations, 0);
        assert_eq!(result, 0);
    }
}
