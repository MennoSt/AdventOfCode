use std::alloc::GlobalAlloc;
use std::arch::global_asm;
use std::fs;
use std::env;

#[derive(PartialEq)]
#[derive(Clone)]
struct Coord {
    x_coord:i128,
    y_coord:i128,
    id:i128,
}

fn main() 
{
    let current_dir = env::current_dir().unwrap();
    let file_path = current_dir.join("exampleinput");
    let contents_ex = fs::read_to_string(file_path).unwrap();
    let file_path = current_dir.join("input");
    let contents = fs::read_to_string(file_path).unwrap();
    
    assert_eq!(sum_answer(&contents_ex,1),374);
    assert_eq!(sum_answer(&contents,1),10228230);

    assert_eq!(sum_answer(&contents_ex,9),1030);
    assert_eq!(sum_answer(&contents_ex,99),8410);
    let sum = sum_answer(&contents,999999);
}

fn sum_answer(contents: &str, grow_factor:i128) -> i128
{
    let mut galaxies = determine_galaxies(&contents);
    let (xmax, ymax) = get_max_values(&galaxies);
    let(emtpy_columns,empty_rows) = get_empty_galaxy_lines(xmax, ymax, &galaxies);
    extend_galaxy(&mut galaxies, emtpy_columns, empty_rows,grow_factor);
    let answer = calculate_sum_shortestpaths(galaxies);
    answer
}

fn extend_galaxy(galaxies: &mut Vec<Coord>, emtpy_columns: Vec<i128>, empty_rows: Vec<i128>, factor:i128) 
{
    let mut increasex = 0;
    let mut increasey = 0;
    for galaxy in galaxies 
    {
        for column in &emtpy_columns 
        {
            if galaxy.x_coord > *column 
            {
                increasex += factor;
            }
        }

        for row in &empty_rows 
        {
            if galaxy.y_coord > *row 
            {
                increasey += factor;
            }
        }
        galaxy.x_coord += increasex;
        galaxy.y_coord += increasey;
        increasex = 0;
        increasey = 0;
    }
}

fn calculate_sum_shortestpaths(galaxies: Vec<Coord>) ->i128 
{
    let mut sum_shortest_paths = 0;
    for galaxy1 in &galaxies 
    {
        for galaxy2 in &galaxies 
        {
            if galaxy2.id > galaxy1.id
            {
                let shortest_path = (galaxy1.x_coord -galaxy2.x_coord).abs() +(galaxy1.y_coord -galaxy2.y_coord).abs();
                sum_shortest_paths += shortest_path;
            }
        }
    }
    println!("{}", sum_shortest_paths);
    sum_shortest_paths
}

fn get_empty_galaxy_lines(xmax: i128,ymax:i128, galaxies: &Vec<Coord>) -> (Vec<i128>,Vec<i128>) {
    let mut x_col_empty:Vec<i128> = Vec::new();
    let mut y_col_empty:Vec<i128> = Vec::new();

    for i in 0..xmax 
    {
        if is_column_empty(&galaxies, i) 
        {
            x_col_empty.push(i);
        }
    }

    for j in 0..ymax 
    {
        if is_row_empty(&galaxies, j) 
        {
            y_col_empty.push(j);
        }
    }
    println!("empty columns:");
    println!("{:?}", x_col_empty);
    println!("{:?}", y_col_empty);

    (x_col_empty,y_col_empty)

}

fn is_row_empty(galaxies: &Vec<Coord>, i: i128) -> bool 
{
    for galaxy in galaxies
    {
        if galaxy.y_coord == i
        {
            return false;
        }
    }
    return true;
}

fn is_column_empty(galaxies: &Vec<Coord>, i: i128) -> bool 
{
    for galaxy in galaxies
    {
        if galaxy.x_coord == i
        {
            return false;
        }
    }
    return true;
}

fn get_max_values(galaxies: &Vec<Coord>) -> (i128, i128) 
{
    let mut xmax = 0;
    let mut ymax = 0;
    for galaxy in galaxies
    {
        if galaxy.x_coord > xmax 
        {
            xmax = galaxy.x_coord;
        }
        if galaxy.y_coord > ymax 
        {
            ymax = galaxy.y_coord;
        }
    }
    (xmax, ymax)
}
    
fn determine_galaxies(contents: &str) ->Vec<Coord>
{
    let mut x = 0;
    let mut y = 0;
    let mut id = 1;
    let mut galaxies:Vec<Coord> = Vec::new();
    println!("initial coordinates:");
    for line in contents.lines() 
    {
        for c in line.chars() 
        {
            if c=='#'
            {
                println!("[{},{}]",x,y);
                galaxies.push(Coord { x_coord: x, y_coord: y ,id:id});
                id += 1;
            }
            x+=1;
        }
        x = 0;
        y += 1;
    }
    galaxies
}
