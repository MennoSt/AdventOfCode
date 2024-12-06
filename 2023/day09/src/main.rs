use lib::filereader;

fn main()
{
    let answer = part1("exampleinput2023day9");
    assert_eq!(answer, 114);

    let answer = part1("../input/day09");
    assert_eq!(answer, 1782868781);
    
    let answer = part2("exampleinput2023day9");
    assert_eq!(answer, 2);
    
    let answer = part2("../input/day09");
    assert_eq!(answer, 1057);
}

fn vec_to_int(input_vec: Vec<&str>) -> Vec<i32> {
    let mut result_vec = Vec::new();

    for word in input_vec {
        if let Ok(num) = word.parse::<i32>() {
            result_vec.push(num);
        } else {
            result_vec.push(0);
        }
    }
    result_vec
}

fn is_all_zeros(vec: &Vec<i32>) -> bool {
    for num in vec {
        if *num != 0 {
            return false;
        }
    }
    true
}
fn part1(contents: &str) -> i32{
    let dataset = parse_data(contents);
    let mut count = 0;

    for data in dataset {
        let start_vector = &data;
        let mut vector_surroundings:Vec<Vec<i32>> = Vec::new();
        vector_surroundings.push(start_vector.clone());
        fill_surroundings(&mut vector_surroundings, &start_vector);

        for vec in vector_surroundings {
            let last = vec.last().unwrap();
            count += last;
        }
    }
    count
}

fn part2(contents: &str) -> i32{
    let dataset = parse_data(&contents);

    let mut count = 0;
    for data in dataset {
        let mut tmp_count = 0;
        let mut vector_surroundings:Vec<Vec<i32>> = Vec::new();
        let start_vector = &data;
        vector_surroundings.push(start_vector.clone());
        fill_surroundings(&mut vector_surroundings, start_vector);

        vector_surroundings.reverse();
        for vec in vector_surroundings {
            let first = vec.first().unwrap();
            tmp_count = first-tmp_count;
        }
        count += tmp_count;
    }
    count
}

fn fill_surroundings(vector_surroundings: &mut Vec<Vec<i32>>, current_vector: &Vec<i32>)
{
    let mut new_vector:Vec<i32> = Vec::new();

    for i in 0..current_vector.len()-1 {
        let diff = current_vector[i+1]-current_vector[i];
        new_vector.push(diff);
    }

    vector_surroundings.push(new_vector.clone());

    if !is_all_zeros(&new_vector)
    {
        fill_surroundings(vector_surroundings, &new_vector)
    }
}

fn parse_data(contents: &str) -> Vec<Vec<i32>>
{
    let contents = filereader::_input(&contents);
    let mut int_vector:Vec<Vec<i32>> = Vec::new();

    for content in contents.lines() {
        let test:Vec<&str> = content.split_whitespace().collect();
        int_vector.push(vec_to_int(test));
    }

    int_vector
}
