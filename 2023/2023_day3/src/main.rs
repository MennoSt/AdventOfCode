mod filereader;

#[derive(PartialEq)]
#[derive(Clone)]
struct Coord {
    x_coord:i32,
    y_coord:i32,
}
#[derive(Clone)]
struct Gears {
    number:String,
    coordinates: Vec<Coord>,
}

struct Grid {
    grid_vec: Vec<String>,
}

impl Grid {
    fn height(&self) -> usize {
        return self.grid_vec.len();
    }
    fn width(&self) -> usize {
        return self.grid_vec[0].len();
    }
    fn elem(&self, x: i32, y: i32) -> char {
        let point_char = ".".chars().nth(0).unwrap();

        if x < 0 || y < 0 {
            return point_char;
        }

        let x_usize: usize = x as usize;
        let y_usize: usize = y as usize;

        if x_usize > (self.width() -1) || y_usize > (self.height()-1) {
            return point_char;
        }

        return self.grid_vec[y_usize].chars().nth(x_usize).unwrap();
    }
}

fn main() {
    let contents_ex = filereader::input("exampleinput2023day3");
    // let sum = calculate_sum_numbers(contents_ex.clone());
    // assert_eq!(sum, 4361);

    // let contents = filereader::input("input2023day3");
    // let sum = calculate_sum_numbers(contents.clone());
    // println!("anwser {}",sum);
    // assert_eq!(sum, 546312);

    // let contents_ex = filereader::input("exampleinput2023day3_2");
    // let gear_ratio_sum = calculate_gear_ratio(contents_ex.clone());
    // println!("example {}",gear_ratio_sum);
    // assert_eq!(gear_ratio_sum, 0);

    let contents_ex = filereader::input("exampleinput2023day3_3");
    let gear_ratio_sum = calculate_gear_ratio(contents_ex.clone());
    println!("example {}",gear_ratio_sum);
    assert_eq!(gear_ratio_sum, 200);

    // let contents_ex = filereader::input("exampleinput2023day3_4");
    // let gear_ratio_sum = calculate_gear_ratio(contents_ex.clone());
    // println!("example {}",gear_ratio_sum);
    // assert_eq!(gear_ratio_sum, 200);

    let contents_ex = filereader::input("exampleinput2023day3");
    let gear_ratio_sum = calculate_gear_ratio(contents_ex.clone());
    println!("anwser {}",gear_ratio_sum);
    assert_eq!(gear_ratio_sum, 467835);

    let contents = filereader::input("input2023day3");
    let gear_ratio_sum = calculate_gear_ratio(contents.clone());
    println!("anwser {}",gear_ratio_sum);
    assert_eq!(gear_ratio_sum, 87449461);
    // tried 87449372
}

fn calculate_gear_ratio(contents: String) ->i32{
    let grid = read_into_grid(contents);
    let mut vect_numbers:Vec<String> = Vec::new();
    let mut vect_gear:Vec<Gears> = Vec::new();
    let mut vect_coord:Vec<Coord> = Vec::new();

    for y in 0..(grid.height()+1) as i32  {
        let mut num = "".to_string();
        let mut is_symbol_bool = false;
        for x in 0..(grid.width()+1) as i32 {
            let elem = grid.elem(x,y);
            if is_digit_0_to_9(elem) {
                num.push(elem);
                if !is_symbol_bool {
                    is_symbol_bool = is_symbol_around(&grid,x,y);
                }
                let coord_vec = symbol_vec_coordinates(&grid, x, y);
                for coord in coord_vec {
                    vect_coord.push(coord);
                }
            } else if is_symbol_bool && num != "" {
                vect_numbers.push(num.clone());
                vect_coord.dedup();
                let gear = Gears{number:num.clone(), coordinates:vect_coord.clone()};
                vect_gear.push(gear);
                vect_coord.clear();
                num = "".to_string();
                is_symbol_bool = false;
            } else {
                num = "".to_string();
                is_symbol_bool = false;
            }
            }
        }

    let mut sum = 0;
    
    for i in 0..vect_gear.len() {
        let mut count = 0;
        let mut gear_multiply = 0;
        for j in 0..vect_gear.len() {
            if i != j {
                if has_one_common_coordinate(vect_gear[i].coordinates.clone(), vect_gear[j].coordinates.clone())
                {
                    count += 1;
                    gear_multiply = vect_gear[i].number.parse::<i32>().unwrap() *
                    vect_gear[j].number.parse::<i32>().unwrap();
                    sum += gear_multiply;
                    println! ("{}",gear_multiply);
                    println! ("{}", count);
                }
            }
        }
        // if count == 1 {
            // sum += gear_multiply/2;
        // }
    }

    sum/2
}

fn has_one_common_coordinate(coord_one: Vec<Coord>, coord_two: Vec<Coord>) ->bool{
    let mut counter = 0;

    for i in 0..coord_one.len() {
        for j in 0..coord_two.len(){
            if coord_one[i] == coord_two[j] {
                counter +=1;
            }
        }
    }
    if counter == 1 {
        return true;
    } else if counter > 1 {
        return true;
    }
    else {
        return false;
    }
}

fn calculate_sum_numbers(contents: String) ->i32{
    let grid = read_into_grid(contents);

    let mut vect_numbers:Vec<String> = Vec::new();
    for y in 0..(grid.height()) as i32  {
        let mut num = "".to_string();
        let mut is_symbol_bool = false;
        for x in 0..(grid.width()+1) as i32 {
            let elem = grid.elem(x,y);
            if is_digit_0_to_9(elem) {
                num.push(elem);
                if !is_symbol_bool {
                    is_symbol_bool = is_symbol_around(&grid,x,y);
                }
            } else if is_symbol_bool && num != "" {
                vect_numbers.push(num.clone());
                num = "".to_string();
                is_symbol_bool = false;
            } else {
                num = "".to_string();
                is_symbol_bool = false;
            }
            }
        }

    let mut sum = 0;
    for vec in vect_numbers {
        sum += vec.parse::<i32>().unwrap();
    }
    sum
}

fn read_into_grid(contents: String) -> Grid {
    let mut contents_vector: Vec<String> = Vec::new();
    for line in contents.lines() {
        contents_vector.push(line.to_string());
    }
    let grid = Grid{grid_vec:contents_vector};
    grid
}


fn is_symbol_around (grid:&Grid, x:i32, y:i32) ->bool
{
    if is_symbol(grid.elem(x,y+1)) || is_symbol(grid.elem(x,y-1)) ||
       is_symbol(grid.elem(x+1,y)) || is_symbol(grid.elem(x-1,y)) || 
       is_symbol(grid.elem(x+1,y+1)) || is_symbol(grid.elem(x+1,y-1)) ||
       is_symbol(grid.elem(x-1,y+1)) || is_symbol(grid.elem(x-1,y-1)) 
    {
        return true;
    } else {
        return false;
    }
}

fn symbol_vec_coordinates(grid:&Grid, x:i32, y:i32) ->Vec<Coord>
{
    let mut vect_coord:Vec<Coord> = Vec::new();
    let vect_coord_it:Vec<Coord> = vec![
        Coord{x_coord:x+1,y_coord:y},
        Coord{x_coord:x+1,y_coord:y-1},
        Coord{x_coord:x+1,y_coord:y+1},
        Coord{x_coord:x,y_coord:y-1},
        Coord{x_coord:x,y_coord:y+1},
        Coord{x_coord:x-1,y_coord:y-1},
        Coord{x_coord:x-1,y_coord:y},
        Coord{x_coord:x-1,y_coord:y+1},
        ];

    for vec in vect_coord_it {
        if is_symbol(grid.elem(vec.x_coord,vec.y_coord)) {
            vect_coord.push(vec);
        }
    }

    return vect_coord;
}

fn is_digit_0_to_9(char: char) -> bool {
    char >= '0' && char <= '9'
}


fn is_symbol (c:char) ->bool
{
    let point_char = ".".chars().nth(0).unwrap();
    if c == point_char || is_digit_0_to_9(c) {
        return false;
    }
    return true;
}

