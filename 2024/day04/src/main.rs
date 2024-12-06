use lib::filereader;
use lib::grid::Grid;

fn main()
{
    let (part1,part2) = solve("../input/day04");
    println!("{}", part1);
    println!("{}", part2);

    assert_eq!(part1, 2500);
    assert_eq!(part2, 1933);
}

fn solve(input:&str) ->(i32,i32)
{
    let grid = filereader::_input_into_grid(input);
    let mut xmas_count = 0;
    let mut mas_count = 0;

    for i in 0..grid._width() as i32 {
        for j in 0..grid._height() as i32 {
            has_mas(&grid, i, j, &mut mas_count);
            has_xmas(&grid, i, j, &mut xmas_count);
        }
    }
    
    (xmas_count, mas_count)
}

fn has_mas(grid: &Grid, i: i32, j: i32, count: &mut i32) 
{
    let mas_strings = vec!["MSMS", "SMMS", "SMSM", "MSSM"];
    if grid._elem(i, j) == "A"
    {
        for mas in mas_strings {
           if grid._elem(i+1, j+1) == mas.chars().nth(0).unwrap().to_string() &&
            grid._elem(i-1, j-1) == mas.chars().nth(1).unwrap().to_string() &&
            grid._elem(i+1, j-1) == mas.chars().nth(2).unwrap().to_string() &&
            grid._elem(i-1, j+1) == mas.chars().nth(3).unwrap().to_string() {
                *count+=1;
            }
        }
    }
}

fn has_xmas(grid: &Grid, i: i32, j: i32, count: &mut i32) 
{
   let directions :Vec<Vec<(i32,i32)>> = vec![
        vec![(i+1,j),(i+2,j),(i+3,j)],
        vec![(i-1,j),(i-2,j),(i-3,j)],
        vec![(i,j+1),(i,j+2),(i,j+3)],
        vec![(i,j-1),(i,j-2),(i,j-3)],
        vec![(i+1,j+1),(i+2,j+2),(i+3,j+3)],
        vec![(i-1,j-1),(i-2,j-2),(i-3,j-3)],
        vec![(i-1,j+1),(i-2,j+2),(i-3,j+3)],
        vec![(i+1,j-1),(i+2,j-2),(i+3,j-3)]];
    
    if grid._elem(i,j) =="X" {
        for dir in directions {
            if grid._elem(dir.get(0).unwrap().0, dir.get(0).unwrap().1,) == "M" &&
                grid._elem(dir.get(1).unwrap().0, dir.get(1).unwrap().1,) == "A" &&
                grid._elem(dir.get(2).unwrap().0, dir.get(2).unwrap().1,) == "S" {
                *count+=1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let sum = solve("test1");
        assert_eq!(sum.0, 18);
    }

    #[test]
    fn test2() {
        let sum = solve("test2");
        assert_eq!(sum.1, 9);
    }

    #[test]
    fn test3() {
        let sum = solve("test3");
        assert_eq!(sum.1, 1);
    }

    #[test]
    fn test4() {
        let sum = solve("test4");
        assert_eq!(sum.1, 0);
    }
}