use lib::filereader;

fn main()
{
    let int_vectors = parse_data("../input/day1");
    let part1 = total_distance(&int_vectors);
    let part2 = total_similarity_score(&int_vectors);
    
    assert_eq!(part1, 1666427);
    assert_eq!(part2, 24316233);
    println!("{} {}", part1, part2);
}

fn parse_data(input: &str) -> (Vec<i32>, Vec<i32>) {
    let contents = filereader::_input(input);
    let mut vec1: Vec<i32> = Vec::new();
    let mut vec2: Vec<i32> = Vec::new();
    for content in contents.lines() {
        let numbers: Vec<i32> = content
        .split_whitespace() 
        .filter_map(|s| s.parse::<i32>().ok()) 
        .collect();
        vec1.push(numbers[0]);
        vec2.push(numbers[1]);
    }
    vec1.sort();
    vec2.sort();
    (vec1, vec2)
}

fn total_distance(vectors: &(Vec<i32>, Vec<i32>)) -> i32 {
    let distance: Vec<i32> = vectors.0.iter()
    .zip(vectors.1.iter()) 
    .map(|(a, b)| (a - b).abs())
    .collect();

    return distance.iter().sum();
}

fn total_similarity_score(vectors: &(Vec<i32>, Vec<i32>) ) -> i32 {
    let mut count_vector: Vec<i32> = Vec::new();
    for value in vectors.0.clone() {
        let count = vectors.1.iter().filter(|&x| *x == value as i32).count();
        count_vector.push(count as i32);
    }
    
    let scores: Vec<i32> = vectors.0.iter()
    .zip(count_vector.iter()) 
    .map(|(a, b)| a * b)
    .collect();

    return scores.iter().sum();
}
