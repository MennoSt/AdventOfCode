use lib::filereader;
use lib::utils::*;
use std::time::Instant;
use itertools::Itertools;

static INPUT: &str = "../input/day08";
static TESTINPUT: &str = "test1";

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
struct Frequency {
    value:String,
    coordinates: Vec<Coordinate>
}

fn main() {
    let start = Instant::now();
    
    let part1 = part1();
    
    // println!("{}",part1);
    // assert_eq!(part1, 1260333054159);
    
    // let part2 = sum(INPUT, true);
    // println!("{}",part2);
    // assert_eq!(part2,162042343638683);
    
    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
}

fn part1() -> i32{
    let grid = filereader::_input_into_grid(TESTINPUT);
    let frequencies = create_frequencies(grid);
    println!("{:?}",frequencies);
    0
}

fn calculate_antinodes (frequencies: Vec<Coordinate>) -> Vec<Coordinate> {
    let mut antinodes = Vec::new();    
    
    let pairs: Vec<(Coordinate, Coordinate)> = frequencies
    .iter()
    .cloned()
    .combinations(2)
    .map(|combo| (combo[0].clone(), combo[1].clone()))
    .collect();

    for pair in &pairs {
        let coordinate1 = &pair.0;
        let coordinate2 = &pair.1;

        let diffx = (coordinate1.x-coordinate2.x).abs();
        let diffy = (coordinate1.y-coordinate2.y).abs();
        
        let mut antinodex1 =0;
        let mut antinodex2 =0;
        let mut antinodey1 =0;
        let mut antinodey2 =0;
        if coordinate1.x > coordinate2.x {
            antinodex1 = coordinate1.x + diffx;
            antinodex2 = coordinate2.x - diffx;
        } else {
            antinodex1 = coordinate1.x - diffx;
            antinodex2 = coordinate2.x + diffx;
        }

        if coordinate1.y > coordinate2.y {
            antinodey1 = coordinate1.y + diffy;
            antinodey2 = coordinate2.y - diffy;
        } else {
            antinodey1 = coordinate1.y - diffy;
            antinodey2 = coordinate2.y + diffy;
        }
        
        let antinode1 = Coordinate{x:antinodex1,y:antinodey1};
        let antinode2 = Coordinate{x:antinodex2,y:antinodey2};
        antinodes.push(antinode1);
        antinodes.push(antinode2);
    }
    println!("Unique pairs: {:?}", pairs);
    antinodes 
}

fn create_frequencies(grid: lib::grid::Grid) -> Vec<Frequency> {
    let mut frequencies:Vec<Frequency> = Vec::new();
    for i in 0..grid._width() as i32 {
        for j in 0..grid._height() as i32{
            let elem = grid._elem(i,j);
            if elem != "." {
                let mut contains_elem = false;
                for  frequency in &mut frequencies {
                    if frequency.value == elem {
                        contains_elem = true;
                        let coord = Coordinate{x:i,y:j};
                        frequency.coordinates.push(coord);
                    }
                }
                if !contains_elem {
                    let coord_vec = vec![Coordinate{x:i,y:j}];
                    let frequency = Frequency{value:grid._elem(i, j),coordinates: coord_vec};
                    frequencies.push(frequency);                
                }
            }
        }
    }
    frequencies
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {

        let coordinates = vec![Coordinate { x: 8, y: 8 }, Coordinate { x: 9, y: 9 }];
        let antinodes = calculate_antinodes(coordinates);
        assert_eq!(antinodes, vec![Coordinate { x: 7, y: 7 }, Coordinate { x: 10, y: 10 }]);
    }
}