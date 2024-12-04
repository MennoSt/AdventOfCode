use lib::filereader;

fn main()
{
    let content = filereader::_input("../input/day03");
    let part1 = part1(&content);
    let part2 = part2(&content);
    
    assert_eq!(part1, 164730528);
    assert_eq!(part2, 70478672);
    println!("{} {}", part1, part2);
}

fn part1(contents: &str) -> i32 {
    let mut multiplication_sum = 0;
    for i in 0..contents.len() {
        if starts_with_mulc(contents, i)
            {
                update_sum(contents, i, &mut multiplication_sum);
            }
    }
    return multiplication_sum;
}

fn part2(contents: &str) -> i32 {
    let mut enabled = true;
    let mut multiplication_sum = 0;
    
    for i in 0..contents.len() {
        toggle_enabled(contents, i, &mut enabled);
        if starts_with_mulc(contents, i) && enabled
        {
            update_sum(contents, i, &mut multiplication_sum);
        }
    }
    return multiplication_sum;
}

fn update_sum(contents: &str, i: usize, multiplication_sum: &mut i32) {
    let mut first_digit = String::from("");
    let mut second_digit = String::from("");
    let mut start_second_digit = false;
    if let Some(ch) = contents.chars().nth(i+4){
        if ch.is_ascii_digit() {
            first_digit.push(ch);
        }
    }
                
    let mut j = 5;
    let mut ch = contents.chars().nth(i+j).unwrap();
    while ch != ')' {
        ch = contents.chars().nth(i+j).unwrap();
        if ch.is_ascii_digit() && !start_second_digit{
            first_digit.push(ch);
        } else if ch == ',' {
            start_second_digit=true;
        } else if ch.is_ascii_digit() && start_second_digit {
            second_digit.push(ch);
        } else if ch != ')' && start_second_digit {
            second_digit.clear();
            first_digit.clear();
            break;
        } else {
            break;
        }
        j+=1;
    }

    *multiplication_sum += first_digit.parse().unwrap_or(0) * 
        second_digit.parse().unwrap_or(0);
    first_digit.clear();
    second_digit.clear();
}

fn starts_with_mulc(contents: &str, i: usize) -> bool {
    contents.chars().nth(i) == Some('m') &&
        contents.chars().nth(i+1) == Some('u') && 
        contents.chars().nth(i+2) == Some('l') &&
        contents.chars().nth(i+3) == Some('(')
}

fn toggle_enabled(contents: &str, i: usize, enabled: &mut bool) {
    if *enabled {
        if contents.chars().nth(i) == Some('d') &&
            contents.chars().nth(i+1) == Some('o') && 
            contents.chars().nth(i+2) == Some('n') &&
            contents.chars().nth(i+3) == Some('\'') &&
            contents.chars().nth(i+4) == Some('t') &&
            contents.chars().nth(i+5) == Some('(') &&
            contents.chars().nth(i+6) == Some(')') 
            {
                *enabled = false;
            }
    } else {
        if contents.chars().nth(i) == Some('d') &&
        contents.chars().nth(i+1) == Some('o') &&
        contents.chars().nth(i+2) == Some('(') &&
        contents.chars().nth(i+3) == Some(')') 
        {
            *enabled = true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let string = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let sum = part1(string);
        assert_eq!(sum, 161);
    }

    #[test]
    fn test2() {
        let string = "+mul(32,64]then(mul(11,8)mul(8,5))";
        let sum = part1(string);
        assert_eq!(sum, 128);
    }

    #[test]
    fn test3() {
        let string = "+mul(32,)64]then(mul(11,8)mul(8,5))";
        let sum = part1(string);
        assert_eq!(sum, 128);
    }

    #[test]
    fn test4() {
        let string = "xmul(2,4)";
        let sum = part2(string);
        assert_eq!(sum, 8);
    }
    #[test]
    fn test5() {
        let string = "don't()xmul(2,4)";
        let sum = part2(string);
        assert_eq!(sum, 0);
    }

    #[test]
    fn test6() {
        let string = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let sum = part2(string);
        assert_eq!(sum, 48);
    }
}