use lib::filereader;

fn main()
{
    let contents = filereader::_input("../puzzle_input/day1");
    let (vec1, vec2) = parse_data(contents);
    let part1 = part1(vec1.clone(), vec2.clone());
    assert_eq!(part1, 1666427);
    
    let contents = filereader::_input("../puzzle_input/day1");
    let (vec1, vec2) = parse_data(contents);
    let part2 = part2(vec1, vec2);
    println!("{} {}", part1, part2);
    assert_eq!(part2, 24316233)
}

fn part2(vec1: Vec<i32>, vec2: Vec<i32>) -> i32 {
    let mut count_vector: Vec<i32> = Vec::new();
    for value in vec1.clone() {
        let count = vec2.iter().filter(|&x| *x == value as i32).count();
        count_vector.push(count as i32);
    }
    
    let result: Vec<i32> = vec1.iter()
    .zip(count_vector.iter()) // Pair elements from both vectors
    .map(|(a, b)| a * b) // Subtract each pair
    .collect();
    let part2:i32 = result.iter().sum();
    part2
}

fn part1(vec1: Vec<i32>, vec2: Vec<i32>) -> i32 {
    let result: Vec<i32> = vec1.iter()
    .zip(vec2.iter()) // Pair elements from both vectors
    .map(|(a, b)| (a - b).abs()) // Subtract each pair
    .collect();

    let part1:i32 = result.iter().sum();
    part1
}

fn parse_data(contents: String) -> (Vec<i32>, Vec<i32>) {
    let mut vec1: Vec<i32> = Vec::new();
    let mut vec2: Vec<i32> = Vec::new();
    for content in contents.lines() {
        let numbers: Vec<i32> = content
        .split_whitespace() // Split by whitespace
        .filter_map(|s| s.parse::<i32>().ok()) // Parse each piece to an integer
        .collect();
        vec1.push(numbers[0]);
        vec2.push(numbers[1]);
    }
    vec1.sort();
    vec2.sort();
    (vec1, vec2)
}
