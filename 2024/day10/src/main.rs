use lib::filereader;
use lib::utils;
use lib::utils::*;
use lib::grid::Grid;
use std::time::Instant;
use itertools::Itertools;
use std::fmt::Debug;

static INPUT: &str = "../input/day10";
static TESTINPUT: &str = "test1";

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
struct Frequency {
    value:String,
    coordinates: Vec<Coordinate>
}

fn main() {
    let start = Instant::now();
    

    part1(INPUT);
    // utils::answer((part1(INPUT), 311),(part2(INPUT), 1115));


    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
}

fn part1(input: &str) {
    let grid = filereader::_input_into_grid(input);
}


        


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {

        let coordinates = vec![Coordinate { x: 8, y: 8 }, Coordinate { x: 9, y: 9 }];
        let antinodes = calculate_antinodes_p1(coordinates);
        assert_eq!(antinodes, vec![Coordinate { x: 7, y: 7 }, Coordinate { x: 10, y: 10 }]);
    }
    
    #[test]
    fn test2() {
        let coordinates = vec![Coordinate { x: 8, y: 7 }, Coordinate { x: 9, y: 9 }];
        let antinodes = calculate_antinodes_p1(coordinates);
        assert_eq!(antinodes, vec![Coordinate { x: 7, y: 5 }, Coordinate { x: 10, y: 11 }]);
    }
    
    #[test]
    fn test3() {
        let part1 = part1(TESTINPUT);
        assert_eq!(part1, 14);
    }

    #[test]
    fn test4() {
        let part2 = part2("test2");
        assert_eq!(part2, 4);
    }

    #[test]
    fn test5() {
        let part2 = part2(TESTINPUT);
        assert_eq!(part2, 34);
    }
}