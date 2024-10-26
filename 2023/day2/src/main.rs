mod filereader;

fn main() {
    let contents = filereader::input("input2023day2");

    let answer_part1 = calculate_sum_part1(contents.clone());
    assert_eq!(answer_part1, 2156);
    println!("Answer part one {}", answer_part1);

    let mut sum = 0;
    for line in contents.lines() {
        sum += multiply_max_cubes(line);
    }
    assert_eq!(sum, 66909);
    println!("Answer part two {}", sum);

}

fn multiply_max_cubes(example_str: &str) -> u32 {
    let example_str = example_str.replace(",", "");
    let example_str = example_str.replace(";", "");
    let example_str = example_str.replace(":", "");
    let example_str = example_str.replace("Game ", "");

    let split_text: Vec<&str> = example_str.split(" ").collect();

    let mut red_max = 0;
    let mut blue_max = 0;
    let mut green_max = 0;
    for i in 1..split_text.len()
    {
        if split_text[i] == "red" {
            let red_value = split_text[i-1].parse::<u32>().unwrap();
            if red_value > red_max {
                red_max = red_value;
            }
        }
        if split_text[i] == "blue" {
            let blue_value = split_text[i-1].parse::<u32>().unwrap();
            if blue_value > blue_max {
                blue_max = blue_value;
            }
        }
        if split_text[i] == "green" {
            let green_value = split_text[i-1].parse::<u32>().unwrap();
            if green_value > green_max {
                green_max = green_value;
            }
        }
    }
    let multiplication = red_max*blue_max*green_max;
    return multiplication;
}

fn calculate_sum_part1(contents: String) -> u32 {
    let mut sum = 0;
    for line in contents.lines() {
        sum += fill_cube_data(line);
    }
    return sum;
}

fn fill_cube_data(example_str: &str) -> u32 {
    let example_str = example_str.replace(",", "");
    let example_str = example_str.replace(";", "");
    let example_str = example_str.replace(":", "");
    let example_str = example_str.replace("Game ", "");

    let split_text: Vec<&str> = example_str.split(" ").collect();

    for i in 1..split_text.len()
    {
        if split_text[i] == "red" {
            if split_text[i-1].parse::<u32>().unwrap() > 12 {
                return 0;
            }
        }
        if split_text[i] == "blue" {
            if split_text[i-1].parse::<u32>().unwrap() > 14 {
                return 0;
            }
        }
        if split_text[i] == "green" {
            if split_text[i-1].parse::<u32>().unwrap() > 13 {
                return 0;
            }
        }
    }
    let sum = split_text[0].parse::<u32>().unwrap();
    return sum;
}