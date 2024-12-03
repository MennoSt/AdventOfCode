use lib::filereader;

fn main()
{
    let content = filereader::_input("../input/day3");
    println!("{}",content);
    
    // let part1 = total_distance(&int_vectors);
    // let part2 = total_similarity_score(&int_vectors);
    
    // assert_eq!(part1, 1666427);
    // assert_eq!(part2, 24316233);
    // println!("{} {}", part1, part2);
}

fn parse_data(input: &str) {
    let contents = filereader::_input(input);
}

fn caculate_multiplication_sum(contents: &str) -> i32 {
    let mut first_digit = String::from("");
    let mut second_digit = String::from("");
    let mut start_second_digit = false;
    let mut multiplication_sum = 0;

    for i in 0..contents.len() {
        if starts_with_mulc(contents, i)
            {
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
                    } else {
                        break;
                    }
                    j+=1;
                }
                
                multiplication_sum += first_digit.parse().unwrap_or(0) * 
                    second_digit.parse().unwrap_or(0);
                println!("{}{}",first_digit, second_digit);
                first_digit.clear();
                second_digit.clear();
            }
    }
    return multiplication_sum;
}

fn starts_with_mulc(contents: &str, i: usize) -> bool {
    contents.chars().nth(i) == Some('m') &&
        contents.chars().nth(i+1) == Some('u') && 
        contents.chars().nth(i+2) == Some('l') &&
        contents.chars().nth(i+3) == Some('(')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let string = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let sum = caculate_multiplication_sum(string);
        assert_eq!(sum, 161);
    }
}