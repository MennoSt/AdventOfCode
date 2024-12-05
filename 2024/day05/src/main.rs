use lib::filereader;

fn main()
{
    let (ordering_vec,pages_vec) = parse_data("../input/day05");

    let part1 = part1(&ordering_vec, &pages_vec);
    println!("{}", part1);
    // println!("{}", part2);

    // assert_eq!(part1, 2500);
    // assert_eq!(part2, 1933);
}

fn part1(ordering_vec: &Vec<Vec<i32>>, pages_vec: &Vec<Vec<i32>>) ->i32 {
    let mut sum = 0;
    for page in pages_vec {
        if has_correct_order(&ordering_vec, &page) {
            sum += get_middle_value(&page);
        }
    }
    sum
}

fn get_middle_value (page:&Vec<i32>) -> i32{
    return page[page.len()/2];
}

fn has_correct_order (ordering_vec:&Vec<Vec<i32>>, page:&Vec<i32>) -> bool {
    for i in 0..(page.len()-1) {
        let vec = vec![page[i],page[i+1]];
        if !ordering_vec.contains(&vec) {
            // println!("{:?}",vec);
            return false;
        }
    }
    return true;
}

fn parse_data(contents: &str) -> (Vec<Vec<i32>>, Vec<Vec<i32>>)
{
    let contents = filereader::_input(&contents);
    let mut vec1:Vec<Vec<i32>> = Vec::new();
    let mut vec2:Vec<Vec<i32>> = Vec::new();

    for content in contents.lines() {
        if content.contains('|') {
            let numbers: Vec<i32> = content
            .split('|')    
            .map(|s| s.parse::<i32>().unwrap()) 
            .collect();
            vec1.push(numbers);
        } else if content.contains(',') {
            let numbers: Vec<i32> = content
            .split(',')
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
            vec2.push(numbers);
        }
    }
    (vec1,vec2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = parse_data("test1");
        let page = vec![75,47,61,53,29];
        let valid_order = has_correct_order(&data.0, &page);
        assert!(valid_order);
    }

    #[test]
    fn test2() {
        let data = parse_data("test1");
        let page = vec![97,13,75,29,47];
        let valid_order = has_correct_order(&data.0, &page);
        assert!(!valid_order);
    }

    #[test]
    fn test3() {
        let data = parse_data("test1");
        let part1 = part1(&data.0, &data.1);
        assert_eq!(part1,143);
    }


}