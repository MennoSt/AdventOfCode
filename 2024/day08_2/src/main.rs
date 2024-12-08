use lib::filereader;
use lib::utils::*;
use lib::grid::Grid;
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
    
    let part1 = part2(INPUT);
    
    println!("{}",part1);
    // assert_eq!(part1, 1260333054159);
    
    // let part2 = sum(INPUT, true);
    // println!("{}",part2);
    // assert_eq!(part2,162042343638683);
    
    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
}

fn part2(input: &str) -> usize{
    let grid = filereader::_input_into_grid(input);
    let frequencies = create_frequencies(&grid);

    let mut antinodes_vec:Vec<Coordinate> =Vec::new();
    for freq in frequencies {
        let coordinates = freq.coordinates;
        let antinodes = calculate_antinodes(coordinates,&grid);
        antinodes_vec.extend(antinodes);
    }
    antinodes_vec.sort();
    antinodes_vec.dedup();
    let alen = antinodes_vec.len();
    let mut out_of_grid =0;
    for i in 0..alen {
        if antinodes_vec[i].x < 0 || antinodes_vec[i].y < 0 || 
            antinodes_vec[i].x >= grid._width() as i32 || antinodes_vec[i].y >= grid._height() as i32 {
                out_of_grid+=1;
        } 
    }


    antinodes_vec.len()-out_of_grid
}

fn calculate_antinodes (frequencies: Vec<Coordinate>, grid:&Grid) -> Vec<Coordinate> {
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
        
        let mut antinode1 = coordinate1.clone();
        let mut antinode2 = coordinate2.clone();
        
        antinodes.push(antinode1.clone());
        antinodes.push(antinode2.clone());
        let xdiff = (antinode1.x -antinode2.x).abs();
        let ydiff = (antinode1.y - antinode2.y).abs();
        while antinode1.x >= 0 && antinode1.x < grid._width() as i32 && antinode1.y >= 0 && antinode1.y < grid._height() as i32{
            let (antinodex1, antinodex2) = calc_antinode(antinode1.x, antinode2.x, xdiff);
            let (antinodey1, antinodey2) = calc_antinode(antinode1.y, antinode2.y, ydiff);
            antinode1 = Coordinate{x:antinodex1,y:antinodey1};
            antinode2 = Coordinate{x:antinodex2,y:antinodey2};
            antinodes.push(antinode1.clone());
            antinodes.push(antinode2.clone());
        }
        
        let mut antinode1 = coordinate1.clone();
        let mut antinode2 = coordinate2.clone();
        while antinode2.x >= 0 && antinode2.x < grid._width() as i32 && antinode2.y >= 0 && antinode2.y < grid._height() as i32{
            let (antinodex1, antinodex2) = calc_antinode(antinode1.x, antinode2.x, xdiff);
            let (antinodey1, antinodey2) = calc_antinode(antinode1.y, antinode2.y, ydiff);
            antinode1 = Coordinate{x:antinodex1,y:antinodey1};
            antinode2 = Coordinate{x:antinodex2,y:antinodey2};
            antinodes.push(antinode1.clone());
            antinodes.push(antinode2.clone());
        }
    }
    antinodes 
}

fn calc_antinode(coordinate1: i32, coordinate2: i32, diff:i32) -> (i32, i32) {
    let antinode1;
    let antinode2;
    if coordinate1 > coordinate2 {
        antinode1 = coordinate1 + diff;
        antinode2 = coordinate2 - diff;
    } else {
        antinode1 = coordinate1 - diff;
        antinode2 = coordinate2 + diff;
    }
    (antinode1, antinode2)
}

fn create_frequencies(grid: &lib::grid::Grid) -> Vec<Frequency> {
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

        // let coordinates = vec![Coordinate { x: 8, y: 8 }, Coordinate { x: 9, y: 9 }];
        // let antinodes = calculate_antinodes(coordinates);
        // assert_eq!(antinodes, vec![Coordinate { x: 7, y: 7 }, Coordinate { x: 10, y: 10 }]);
    }
    
    #[test]
    fn test2() {
        let part2 = part2("test2");
        assert_eq!(part2, 4);
    }

    #[test]
    fn test3() {
        let part2 = part2(TESTINPUT);
        assert_eq!(part2, 34);
    }
}