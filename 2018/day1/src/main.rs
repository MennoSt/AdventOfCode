
use std::fs;
use std::env;
use std::collections::HashSet;

fn main() {
    let current_dir = env::current_dir().unwrap();
    let file_path = current_dir.join("input");
    let contents = fs::read_to_string(file_path).unwrap();

    let sum = calculate_sum(contents.lines());
    print!("Answer part 1 is {}\n", sum);
    assert_eq!(sum, 430);

    //Examples part 2:
    assert_eq!(calculate_frequency(str.lines()), 10);
    let str = "-6\n+3\n+8\n+5\n-6\n";
    assert_eq!(calculate_frequency(str.lines()), 5);
    let str = "+7\n+7\n-2\n-7\n-4\n";
    assert_eq!(calculate_frequency(str.lines()), 14);

    let frequency = calculate_frequency(contents.lines());
    print!("Answer part 2 is {}\n", frequency);
    assert_eq!(frequency, 462);
}

fn calculate_sum(lines: std::str::Lines<'_>) -> i32 {
    let mut sum = 0;
    for line in lines 
    {
        let sign = &line[0..1];
        let number = &line[1..].parse::<i32>().unwrap();
        if sign == "+" {
            sum += number;
        } else {
            sum -= number;
        }
    }
    sum
}

fn calculate_frequency(lines: std::str::Lines<'_>) -> i32 {
    let mut sum =0;
    let mut seen = HashSet::new();

    loop {
        for line in lines.clone()
        {
            let sign = &line[0..1];
            let number = &line[1..].parse::<i32>().unwrap();

            if sign == "+" {
                sum += number;
            } else {
                sum -= number;
            }
            
            if seen.contains(&sum) {
                return sum;
            } else {
                seen.insert(sum);
            }
        }
    }
}
