mod filereader;
use std::time::{SystemTime, Duration};
use std::collections::HashMap;

#[derive(Clone)]
struct Map {
    source: i128,
    destination: i128,
    range:i128,
}

fn main() {
    let start = SystemTime::now();
    let contents_ex = filereader::_input("exampleinput2023day5");
    let contents = filereader::_input("input2023day5");

    // let answer = calculate_smallest_location_part1(contents_ex.clone());
    // assert_eq!(answer, 35);

    // let answer = calculate_smallest_location_part1(contents.clone());
    // assert_eq!(answer, 825516882);
    // println!("answer part one {}", answer);

    // let smallest_location = calculate_smallest_location_part2(contents_ex.clone());
    // println! ("answer is: {}", smallest_location);
    // assert_eq!(smallest_location,46);

    // let smallest_location = calculate_smallest_location_part2(contents.clone());
    // println! ("answer is: {}", smallest_location);

    // let location = convert_location_to_seed(map_vectors.1.clone() ,35);
    // println!("location answer {}", location);
    
    
    
    // assert_eq!(convert_location_to_seed(map_vectors.1.clone(),82),79);
    // assert_eq!(convert_location_to_seed(map_vectors.1.clone(),43),14);
    // assert_eq!(convert_location_to_seed(map_vectors.1.clone(),86),55);
    // assert_eq!(convert_location_to_seed(map_vectors.1.clone(),35),13);
    
    
    assert_eq!(calculate_lowest_location(contents_ex), 46);

    let answer_part_two = calculate_lowest_location(contents);
    println!("answer: {}", answer_part_two);



    
    let duration = start.elapsed();
    println!("Execution time: {:?} milliseconds", duration);

}

fn calculate_lowest_location(contents: String) ->i128{
    let map_vectors:(Vec<i128>,Vec<Vec<Map>>) = parse_input_data(contents);

    let mut seedmap: HashMap<i128, i128> = HashMap::new();
    for i in 0..map_vectors.0.len() {
        if i % 2 ==0 {
            seedmap.insert(map_vectors.0[i],map_vectors.0[i+1]);
        }
    }

    println!("{:?}", seedmap);

    let mut location:i128 = 0;

    let max_possible = ::std::i128::MAX;

    // 5844012 110899473
    for i in 0..max_possible
    {
        let seed = convert_location_to_seed(map_vectors.1.clone(), i);
        for map in &seedmap {
            if seed >= *map.0 && seed < map.0 + map.1 {
                println!("output seed {}",seed);
                println!("location input {}", i);
                location = i;
                return location;
            }
        }
    }
    return location;
}


fn convert_location_to_seed(mut map_vectors:Vec<Vec<Map>>, location:i128) -> i128 {

    let mut location:i128 = location;
    let mut seed = location.clone();
    map_vectors.reverse();

    // println!("{}",location);
    for map_vector in map_vectors {
        for map in map_vector {
            if location >= map.destination && location < (map.destination + map.range) {
                seed = map.source + location - map.destination;
                location = seed.clone();
                break;
            }
        }
        // println!("{}",location);
    }
    seed
}

// if seed < map.source + map.range && seed >= map.source {
//     location = map.destination + seed - map.source;
//     seed = location.clone();

fn calculate_smallest_location_part2(contents: String) -> i128 {
    let map_vectors:(Vec<i128>,Vec<Vec<Map>>) = parse_input_data(contents);
    let mut smallest_location = ::std::i128::MAX;

    let mut seedmap: HashMap<i128, i128> = HashMap::new();
    for i in 0..map_vectors.0.len() {
        if i % 2 ==0 {
            seedmap.insert(map_vectors.0[i],map_vectors.0[i+1]);
        }
    }

    println!("{:?}", seedmap);

    for (key, value) in seedmap {
        println!("{}", "new key pair");
        for i in 0..value {
            let location = convert_seed_to_location(map_vectors.1.clone(), key+i);
            if location < smallest_location {
                smallest_location = location;
            }
        }
    }
    smallest_location
}

fn calculate_smallest_location_part1(contents: String) -> i128 {
    let map_vectors:(Vec<i128>,Vec<Vec<Map>>) = parse_input_data(contents);
    let mut smallest_location = ::std::i128::MAX;

    for seed in map_vectors.0 {
        let location = convert_seed_to_location(map_vectors.1.clone(), seed);
        if location < smallest_location {
            smallest_location = location;
        }
    }
    smallest_location
}

fn convert_seed_to_location(map_vectors:Vec<Vec<Map>>, seed:i128) -> i128 {

    let mut seed:i128 = seed;
    let mut location = seed.clone();
    for map_vector in map_vectors {
        for map in map_vector {
            if seed < map.source + map.range && seed >= map.source {
                location = map.destination + seed - map.source;
                seed = location.clone();
                break;
            }
        }
    }
    location
}

fn parse_input_data(contents: String) -> (Vec<i128>,Vec<Vec<Map>>) {
    let inputs: Vec<String> = contents.split("\n\n").map(String::from).collect();
    let seed_vec: Vec<i128> = inputs[0].split_whitespace()
    .filter_map(|s| s.chars().filter(|c| c.is_numeric()).collect::<String>().parse().ok())
    .collect();

    let mut conversion_maps: Vec<Vec<Map>> = Vec::new();
    for i in 1..inputs.len() {
        let map: Vec<Map> = pars_into_map(inputs[i].to_string());
        conversion_maps.push(map);
    }
    (seed_vec, conversion_maps)
}

fn pars_into_map (input: String)->Vec<Map> {
    let mut seed_to_soil_map: Vec<Map> = Vec::new();
    let seed_to_soil_vec: Vec<&str> = input.split('\n').collect();
    for i in 1..seed_to_soil_vec.len() {
        let tmp_vec: Vec<i128> = seed_to_soil_vec[i].split_whitespace().map(|s| s.parse().unwrap()).collect();
        let map = Map {source:tmp_vec[1], destination:tmp_vec[0], range:tmp_vec[2]};
        seed_to_soil_map.push(map);
    }
    seed_to_soil_map
    }
