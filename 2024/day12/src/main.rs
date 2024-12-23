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

fn part1(file:&str) -> i32
{
    let grid = filereader::_input_into_grid(file);
    let mut visited_grid = grid._create_visiter_grid();
    let mut total_price=0;
    for x in 0..grid._width() as i32 {
        for y in 0..grid._height() as i32 {
            let fence_type = grid._elem(x,y);
            let mut plant = Plant{fence_type:fence_type, count:0,fences:0};
            if visited_grid._elem(x,y) != 1 {
                total_price += calculate_fences_p1(x, y, &grid, &mut visited_grid, &mut plant)
            }
        }
    }
    total_price
}

fn calculate_fences_p1(x:i32, y:i32, grid:&Grid, visited_grid: &mut Gridi32, plant:&mut Plant) -> i32
{
    visited_grid._set(x, y,1);
    let directions = vec![Coordinate{x:x-1,y:y}, 
                                            Coordinate{x:x+1,y:y}, 
                                            Coordinate{x:x,y:y-1}, 
                                            Coordinate{x:x,y:y+1}];

    plant.fences += 4;
    plant.count += 1;
    for dir in directions {
        if grid._is_within_grid(dir.x,dir.y) {
            if plant.fence_type ==  grid._elem(dir.x, dir.y) {
                plant.fences -= 1;
                if visited_grid._elem(dir.x, dir.y) != 1 {
                    calculate_fences_p1(dir.x, dir.y, grid, visited_grid, plant);
                }
            }
        }
    }
    plant.fences * plant.count
}

fn part2(file:&str) -> i32
{
    let grid = filereader::_input_into_grid(file);
    let mut visited_grid = grid._create_visiter_grid();
    let mut total_price=0;
    
    for x in 0..grid._width() as i32 {
        for y in 0..grid._height() as i32 {
            let fence_type = grid._elem(x,y);
            let mut plant = Plant{fence_type:fence_type, count:0, fences:0};
            if visited_grid._elem(x,y) != 1 {
                total_price += calculate_fences_p2(x, y, &grid, &mut visited_grid, &mut plant);
            }
        }
    }
    total_price
}

fn calculate_fences_p2(x:i32, y:i32, grid:&Grid, visited_grid: &mut Gridi32, plant:&mut Plant) -> i32
{
    let directions = vec![Coordinate{x:x-1,y:y}, 
    Coordinate{x:x,y:y-1}, 
    Coordinate{x:x+1,y:y}, 
    Coordinate{x:x,y:y+1}];
    
    visited_grid._set(x, y,1);
    plant.count += 1;

    update_not_surrounded(x, y, grid, plant);
    update_corner(x, y, grid, plant);
    update_single_end(x, y, grid,plant);
    update_t_shape(x, y, grid, plant);
    update_cross(x, y, grid, plant);
    
    for dir in directions {
        if grid._is_within_grid(dir.x,dir.y) {
            if plant.fence_type ==  grid._elem(dir.x, dir.y) {
                if visited_grid._elem(dir.x, dir.y) != 1 {
                    calculate_fences_p2(dir.x, dir.y, grid, visited_grid, plant);
                }
            }
        }
    }
    plant.fences * plant.count
}

fn update_cross (x: i32, y: i32, grid: &Grid, plant: &mut Plant) {
    if grid._left(x, y) == plant.fence_type && 
        grid._right(x,y) == plant.fence_type &&
        grid._up(x, y) == plant.fence_type &&
        grid._down(x, y) == plant.fence_type {
            if grid._right(x, y-1) != plant.fence_type {
                plant.fences += 1;
            } 
            if grid._right(x, y+1) != plant.fence_type {
                plant.fences += 1;
            } 
            if grid._left(x, y-1) != plant.fence_type {
                plant.fences += 1;
            } 
            if grid._left(x, y+1) != plant.fence_type {
                plant.fences += 1;
            }
        }
}

fn update_t_shape(x: i32, y: i32, grid: &Grid, plant: &mut Plant) 
{
    if grid._left(x, y) != plant.fence_type && 
        grid._right(x,y) == plant.fence_type &&
        grid._up(x, y) == plant.fence_type &&
        grid._down(x, y) == plant.fence_type {
            if grid._right(x, y-1) != plant.fence_type && grid._right(x, y+1) != plant.fence_type {
                plant.fences += 2;
            } else if grid._right(x, y-1) != plant.fence_type {
                plant.fences += 1;
            } else if grid._right(x, y+1) != plant.fence_type {
                plant.fences += 1;
            }
        }
    
    if grid._left(x, y) == plant.fence_type && 
        grid._right(x,y) != plant.fence_type &&
        grid._up(x, y) == plant.fence_type &&
        grid._down(x, y) == plant.fence_type {
            if grid._left(x, y-1) != plant.fence_type && grid._left(x, y+1) != plant.fence_type {
                plant.fences += 2;
            } else if grid._left(x, y-1) != plant.fence_type {
                plant.fences += 1;
            } else if grid._left(x, y+1) != plant.fence_type {
                plant.fences += 1;
            }
        }
    
    if grid._left(x, y) == plant.fence_type && 
        grid._right(x,y) == plant.fence_type &&
        grid._up(x, y) != plant.fence_type &&
        grid._down(x, y) == plant.fence_type {
            if grid._right(x, y+1) != plant.fence_type && grid._left(x, y+1) != plant.fence_type {
                plant.fences += 2;
            } else if grid._right(x, y+1) != plant.fence_type {
                plant.fences += 1;
            } else if grid._left(x, y+1) != plant.fence_type {
                plant.fences += 1;
            }
        }
    
    if grid._left(x, y) == plant.fence_type && 
        grid._right(x,y) == plant.fence_type &&
        grid._up(x, y) == plant.fence_type &&
        grid._down(x, y) != plant.fence_type {
            if grid._right(x, y-1) != plant.fence_type && grid._left(x, y-1) != plant.fence_type {
                plant.fences += 2;
            } else if grid._right(x, y-1) != plant.fence_type {
                plant.fences += 1;
            } else if grid._left(x, y-1) != plant.fence_type {
                plant.fences += 1;
            }
        }
}

fn update_not_surrounded(x: i32, y: i32, grid: &Grid, plant: &mut Plant) {
    if grid._left(x, y) != plant.fence_type && 
    grid._right(x,y) != plant.fence_type &&
    grid._up(x, y) != plant.fence_type &&
    grid._down(x,y) != plant.fence_type {
        plant.fences+=4;
    }
}

fn update_single_end(x: i32, y: i32, grid: &Grid, plant: &mut Plant) {
    if is_single_end(x, y, grid, plant) {
        plant.fences += 2;
    }
}

fn is_single_end(x: i32, y: i32, grid: &Grid, plant: &mut Plant ) -> bool {
    let ftype = &plant.fence_type;
    let mut count = 0;

    if grid._left(x,y) == *ftype {
        count +=1;
    }
    if grid._right(x,y) == *ftype {
        count +=1;
    }
    if grid._up(x,y) == *ftype {
        count +=1;
    }
    if grid._down(x,y) == *ftype {
        count +=1;
    }
    count == 1
}

fn update_corner(x: i32, y: i32, grid: &Grid, plant: &mut Plant) {
    let mut is_corner = false;
    let mut is_free_corner = false;
    
    if grid._left(x, y) == plant.fence_type && 
        grid._right(x,y) != plant.fence_type &&
        grid._up(x, y) == plant.fence_type &&
        grid._down(x, y) != plant.fence_type {
            is_corner = true;
            if grid._left(x, y-1) != plant.fence_type {
                is_free_corner = true;
            }
        }

    if grid._left(x, y) == plant.fence_type && 
        grid._right(x,y) != plant.fence_type &&
        grid._up(x, y) != plant.fence_type &&
        grid._down(x, y) == plant.fence_type {
            is_corner = true;
            if grid._left(x, y+1) != plant.fence_type {
                is_free_corner = true;
            }
        }

    if grid._left(x, y) != plant.fence_type && 
        grid._right(x,y) == plant.fence_type &&
        grid._up(x, y) == plant.fence_type &&
        grid._down(x, y) != plant.fence_type {
            is_corner = true;
            if grid._right(x, y-1) != plant.fence_type {
                is_free_corner = true;
            }
        }
    
    if grid._left(x, y) != plant.fence_type && 
        grid._right(x,y) == plant.fence_type &&
        grid._up(x, y) != plant.fence_type &&
        grid._down(x, y) == plant.fence_type {
            is_corner = true;
            if grid._right(x, y+1) != plant.fence_type {
                is_free_corner = true;
            }
        }

        if is_corner && is_free_corner {
            plant.fences += 2;
        } else if is_corner {
            plant.fences +=1;
        }
}

fn main() {
    let start = Instant::now();
    
    utils::answer((part1(INPUT), 1370258),(part2(INPUT), 805814));
    
    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
}

#[cfg(test)]
mod tests {
    static TESTINPUT: &str = "test1";
    use super::*;

    #[test]
    fn test1_p1() {
        let part1 = part1(TESTINPUT);
        assert_eq!(part1, 140);
    }
    
    #[test]
    fn test2_p1() {
        let part1 = part1("test8");
        assert_eq!(part1, 1930);
    }

    #[test]
    fn test3_p1() {
        let part1 = part1("test3");
        assert_eq!(part1, 40);
    }

    #[test]
    fn test1() {
        let part1 = part2(TESTINPUT);
        assert_eq!(part1, 80);
    }

    #[test]
    fn test2() {
        let part1 = part2("test2");
        assert_eq!(part1, 40);
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
        assert_eq!(part1, 236);
    }

    #[test]
    fn test6() {
        let part1 = part2("test6");
        assert_eq!(part1, 368);
    }

    #[test]
    fn test7() {
        let part1 = part2("test7");
        assert_eq!(part1, 76);
    }

    #[test]
    fn test8() {
        let part1 = part2("test8");
        assert_eq!(part1, 1206);
    }
}
