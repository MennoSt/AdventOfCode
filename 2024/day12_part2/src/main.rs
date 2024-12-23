use lib::filereader;
use lib::utils;
use lib::utils::*;
use lib::grid::*;
use std::time::Instant;

static INPUT: &str = "../input/day12";

struct Plant {
    fence_type:String,
    count: i32,
    fences: i32
}

fn main() {
    let start = Instant::now();
    
    // utils::answer((part1(INPUT), 794),(part2(INPUT), 1706));
    
    let answer = part2(INPUT);
    println!("{:?}", answer);
    assert_eq!(answer, 1370258);
    
    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
}

fn part2(file:&str) -> i32
{
    let grid = filereader::_input_into_grid(file);
    let mut visited_grid = grid._create_visiter_grid();
    let mut total_price=0;
    
    for x in 0..grid._width() as i32 {
        for y in 0..grid._height() as i32 {
            let fence_type = grid._elem(x,y);
            let mut first_iter = true;
            let mut plant = Plant{fence_type:fence_type, count:0, fences:0};
            if visited_grid._elem(x,y) != 1 {
                total_price += calculate_fences(x, y, &grid, &mut visited_grid, &mut plant, &mut first_iter)
            }
        }
    }
    total_price
}

fn calculate_fences(x:i32, y:i32, grid:&Grid, visited_grid: &mut Gridi32, plant:&mut Plant, first_it: &mut bool) -> i32
{
    visited_grid._set(x, y,1);
    let directions = vec![Coordinate{x:x-1,y:y}, 
                                            Coordinate{x:x+1,y:y}, 
                                            Coordinate{x:x,y:y-1}, 
                                            Coordinate{x:x,y:y+1}];

    
    if is_straight(x, y, grid, plant) && !*first_it {
        plant.fences -= 2;
    }

    if is_corner(x,y,grid, plant) {
        plant.fences -= 1;
    }
    
    if *first_it {
        *first_it = false;
    }
    
    plant.fences += 4;
    plant.count += 1;
    for dir in directions {
        if grid._is_within_grid(dir.x,dir.y) {
            if plant.fence_type ==  grid._elem(dir.x, dir.y) {
                plant.fences -= 1;
                if visited_grid._elem(dir.x, dir.y) != 1 {
                    calculate_fences(dir.x, dir.y, grid, visited_grid, plant, first_it);
                }
            }
        }
    }
    plant.fences * plant.count
}

fn is_corner(x: i32, y: i32, grid: &Grid, plant: &mut Plant) -> bool {
    let mut is_corner = false;
    if grid._left(x, y) == plant.fence_type && 
        grid._right(x,y) != plant.fence_type &&
        grid._up(x, y) != plant.fence_type &&
        grid._down(x, y) == plant.fence_type {
            is_corner = true;
        }
    
    if grid._left(x, y) == plant.fence_type && 
        grid._right(x,y) != plant.fence_type &&
        grid._up(x, y) == plant.fence_type &&
        grid._down(x, y) != plant.fence_type {
            is_corner = true;
        }

    if grid._left(x, y) != plant.fence_type && 
        grid._right(x,y) == plant.fence_type &&
        grid._up(x, y) == plant.fence_type &&
        grid._down(x, y) != plant.fence_type {
            is_corner = true;
        }

    if grid._left(x, y) != plant.fence_type && 
        grid._right(x,y) == plant.fence_type &&
        grid._up(x, y) != plant.fence_type &&
        grid._down(x, y) == plant.fence_type {
            is_corner = true;
        
        }
    
    is_corner

}

fn is_straight(x: i32, y: i32, grid: &Grid, plant: &mut Plant) -> bool {
    let mut is_straight = false;
    if grid._left(x, y) == plant.fence_type && 
        grid._right(x,y) == plant.fence_type &&
        grid._up(x, y) != plant.fence_type &&
        grid._down(x, y) != plant.fence_type {
            is_straight = true;
        }

    if grid._left(x, y) != plant.fence_type && 
        grid._right(x,y) != plant.fence_type &&
        grid._up(x, y) == plant.fence_type &&
        grid._down(x, y) == plant.fence_type {
            is_straight = true;
        }

    if grid._left(x, y) == plant.fence_type && 
        grid._right(x,y) != plant.fence_type &&
        grid._up(x, y) != plant.fence_type &&
        grid._down(x, y) != plant.fence_type {
            if grid._left_left(x,y) == plant.fence_type {
                is_straight = true;
            }
        }

    if grid._left(x, y) != plant.fence_type && 
        grid._right(x,y) == plant.fence_type &&
        grid._up(x, y) != plant.fence_type &&
        grid._down(x, y) != plant.fence_type {
            if grid._right_right(x,y) == plant.fence_type {
                is_straight = true;
            }
        }

    if grid._left(x, y) != plant.fence_type && 
        grid._right(x,y) != plant.fence_type &&
        grid._up(x, y) == plant.fence_type &&
        grid._down(x, y) != plant.fence_type {
            if grid._up_up(x,y) == plant.fence_type {
                is_straight = true;
            }
    }
    
    if grid._left(x, y) != plant.fence_type && 
        grid._right(x,y) != plant.fence_type &&
        grid._up(x, y) != plant.fence_type &&
        grid._down(x, y) == plant.fence_type {
            if grid._down_down(x,y) == plant.fence_type {
                is_straight = true;
            }
        } 

    is_straight
}

#[cfg(test)]
mod tests {
    static TESTINPUT: &str = "test1";
    use super::*;

    #[test]
    fn test1() {
        let part1 = part2(TESTINPUT);
        assert_eq!(part1, 80);
    }

    #[test]
    fn test3() {
        let part1 = part2("test3");
        assert_eq!(part1, 16);
    }

    #[test]
    fn test4() {
        let part1 = part2("test4");
        assert_eq!(part1, 16);
    }
    
    #[test]
    fn test5() {
        let part1 = part2("test5");
        assert_eq!(part1, 64);
    }

}
