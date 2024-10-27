use std::alloc::GlobalAlloc;
use std::arch::global_asm;
use std::fs;
use std::env;

#[derive(PartialEq)]
#[derive(Clone)]
struct Coord {
    x_coord:i32,
    y_coord:i32,
}

fn main() 
{
    let current_dir = env::current_dir().unwrap();
    let file_path = current_dir.join("exampleinput");
    let contents = fs::read_to_string(file_path).unwrap();
    
    let mut galaxies = determine_galaxies(&contents);
    let (xmax, ymax) = get_max_values(&galaxies);

    let(emtpy_columns,empty_rows) = get_empty_galaxy_lines(xmax, ymax, &galaxies);
    extend_galaxy(&mut galaxies, emtpy_columns, empty_rows);
    
    println!("extended galaxy:");
    for galaxy in galaxies {
        println!("[{},{}]",galaxy.x_coord,galaxy.y_coord);
    }
}

fn extend_galaxy(galaxies: &mut Vec<Coord>, emtpy_columns: Vec<i32>, empty_rows: Vec<i32>) {
    for galaxy in galaxies 
    {
        let mut increasex =0;
        let mut increasey = 0;
        for column in &emtpy_columns 
        {
            if galaxy.x_coord > *column {
                increasex+=1;
            }
        }

        for row in &empty_rows 
        {
            if galaxy.y_coord > *row {
                increasey+=1;
            }
        }
        galaxy.x_coord += increasex;
        galaxy.y_coord += increasey;
        increasex = 0;
        increasey = 0;
    }
}

fn get_empty_galaxy_lines(xmax: i32,ymax:i32, galaxies: &Vec<Coord>) -> (Vec<i32>,Vec<i32>) {
    let mut x__col_empty:Vec<i32> = Vec::new();
    let mut y__col_empty:Vec<i32> = Vec::new();

    for i in 0..xmax 
    {
        if is_column_empty(&galaxies, i) 
        {
            x__col_empty.push(i);
        }
    }

    for j in 0..ymax 
    {
        if is_row_empty(&galaxies, j) 
        {
            y__col_empty.push(j);
        }
    }
    println!("empty columns:");
    println!("{:?}", x__col_empty);
    println!("{:?}", y__col_empty);

    (x__col_empty,y__col_empty)

}

fn is_row_empty(galaxies: &Vec<Coord>, i: i32) -> bool {
    for galaxy in galaxies
    {
        if galaxy.y_coord == i
        {
            return false;
        }
    }
    return true;
}

fn is_column_empty(galaxies: &Vec<Coord>, i: i32) -> bool {
    for galaxy in galaxies
    {
        if galaxy.x_coord == i
        {
            return false;
        }
    }
    return true;
}

fn get_max_values(galaxies: &Vec<Coord>) -> (i32, i32) {
    let mut xmax = 0;
    let mut ymax = 0;
    for galaxy in galaxies
    {
        if galaxy.x_coord > xmax {
            xmax = galaxy.x_coord;
        }
        if galaxy.y_coord > ymax {
            ymax = galaxy.y_coord;
        }
    }
    (xmax, ymax)
    }
    
fn determine_galaxies(contents: &str) ->Vec<Coord>
{
    let mut x = 0;
    let mut y = 0;
    let mut galaxies:Vec<Coord> = Vec::new();
    println!("initial coordinates:");
    for line in contents.lines() 
    {
        for c in line.chars() 
        {
            if c=='#'
            {
                println!("[{},{}]",x,y);
                galaxies.push(Coord { x_coord: x, y_coord: y })
            }
            x+=1;
        }
        x = 0;
        y += 1;
    }
    galaxies
}
