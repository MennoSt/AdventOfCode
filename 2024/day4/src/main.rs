use lib::filereader;
use lib::grid;
use lib::grid::Grid;

fn main()
{
    let grid = parse_data("../input/day4");
    let part1 = part1(&grid);
    println!("{}",part1);
    let part2 = part2(&grid);
    println!("{}",part2);
    
    assert_eq!(part1,2500);
    assert_eq!(part2,1933);
}

fn part1(grid:&Grid) ->i32
{
    let mut count = 0;
    for i in 0..grid._width() as i32 {
        for j in 0..grid._height() as i32 {
            has_xmas(&grid, i, j, &mut count);
        }
    }

    return count;
}

fn part2(grid:&Grid) ->i32
{
    let mut count = 0;
    for i in 0..grid._width() as i32 {
        for j in 0..grid._height() as i32 {
            has_mas(&grid, i, j, &mut count);
        }
    }

    return count;
}


fn has_mas(grid: &Grid, i: i32, j: i32, count: &mut i32) {
    let borrowed_strings = vec!["MSMS", "SMMS", "SMSM", "MSSM"];
    if grid._elem(i, j) == "A"
    {
        for elem in borrowed_strings {
           if grid._elem(i+1, j+1) == elem.chars().nth(0).unwrap().to_string() &&
            grid._elem(i-1, j-1) == elem.chars().nth(1).unwrap().to_string() &&
            grid._elem(i+1, j-1) == elem.chars().nth(2).unwrap().to_string() &&
            grid._elem(i-1, j+1) == elem.chars().nth(3).unwrap().to_string() {
                *count+=1;
            }
        }
    }
}

fn has_xmas(grid: &Grid, i: i32, j: i32, count: &mut i32) {
    
    if grid._elem(i, j) == "X" &&
        grid._elem(i+1, j) == "M" &&
        grid._elem(i+2, j) == "A" &&
        grid._elem(i+3, j) == "S" {
            println!("{}{}{}",i,j,"xmas");
            *count+=1;
        }

    if grid._elem(i, j) == "X" &&
        grid._elem(i-1, j) == "M" &&
        grid._elem(i-2, j) == "A" &&
        grid._elem(i-3, j) == "S" {
            println!("{}{}{}",i,j,"xmas");
            *count+=1;
    }

    if grid._elem(i, j) == "X" &&
    grid._elem(i, j+1) == "M" &&
    grid._elem(i, j+2) == "A" &&
    grid._elem(i, j+3) == "S" {
        println!("{}{}{}",i,j,"xmas");
        *count+=1;
    }

    if grid._elem(i, j) == "X" &&
    grid._elem(i, j-1) == "M" &&
    grid._elem(i, j-2) == "A" &&
    grid._elem(i, j-3) == "S" {
        println!("{}{}{}",i,j,"xmas");
        *count+=1;
    }

    if grid._elem(i, j) == "X" &&
    grid._elem(i-1, j-1) == "M" &&
    grid._elem(i-2, j-2) == "A" &&
    grid._elem(i-3, j-3) == "S" {
        println!("{}{}{}",i,j,"xmas");
        *count+=1;
    }

    if grid._elem(i, j) == "X" &&
    grid._elem(i+1, j+1) == "M" &&
    grid._elem(i+2, j+2) == "A" &&
    grid._elem(i+3, j+3) == "S" {
        println!("{}{}{}",i,j,"xmas");
        *count+=1;
    }

    if grid._elem(i, j) == "X" &&
    grid._elem(i-1, j+1) == "M" &&
    grid._elem(i-2, j+2) == "A" &&
    grid._elem(i-3, j+3) == "S" {
        println!("{}{}{}",i,j,"xmas");
        *count+=1;
    }

    if grid._elem(i, j) == "X" &&
    grid._elem(i+1, j-1) == "M" &&
    grid._elem(i+2, j-2) == "A" &&
    grid._elem(i+3, j-3) == "S" {
        println!("{}{}{}",i,j,"xmas");
        *count+=1;
    }

}

fn parse_data(contents: &str)-> Grid
{
    let contents = filereader::_input(&contents);
    let grid = read_into_grid(&contents);
    grid
}

fn read_into_grid(contents: &str) -> Grid {
    let mut contents_vector: Vec<Vec<String>> = Vec::new();
    for line in contents.lines() {
        let test:Vec<char> = line.chars().collect();
        let strings = test
        .iter()
        .map(|c| String::from(c.to_string()))
        .collect::<Vec<String>>();
        contents_vector.push(strings);
    }
    let grid = Grid{grid_vec:contents_vector};
    grid
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let grid = parse_data("test1");
        let sum = part1(&grid);
        assert_eq!(sum, 18);
    }

    #[test]
    fn test2() {
        let grid = parse_data("test2");
        let sum = part2(&grid);
        assert_eq!(sum, 9);
    }

    #[test]
    fn test3() {
        let grid = parse_data("test3");
        let sum = part2(&grid);
        assert_eq!(sum, 1);
    }

    #[test]
    fn test4() {
        let grid = parse_data("test4");
        let sum = part2(&grid);
        assert_eq!(sum, 0);
    }
}