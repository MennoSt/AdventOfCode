use std::fs;
use std::env;
use chrono::{NaiveDateTime, Duration};
use regex::Regex;
use std::collections::HashSet;
use chrono::Timelike;
use std::collections::HashMap;

#[derive(Clone)]
struct TimeStampData {
    time_stamp: NaiveDateTime,
    guard_action: String
}

struct Guard {
    number:u32,
    minutes_sleep_time:i64,
    sleepiest_minute:u32,
    minute_hashmap:HashMap<u32, u32>
}

fn parse_data(contents: &str) -> Vec<TimeStampData> 
{
    let format = "%Y-%m-%d %H:%M";
    let mut timestamp_data: Vec<TimeStampData> = Vec::new();
    for line in contents.lines() {
        let datetime_str = &line[1..17];
        let guard_str = &line[18..];
        let naive_datetime: NaiveDateTime  = NaiveDateTime::parse_from_str(datetime_str, format).unwrap();
        let timestamp = TimeStampData{time_stamp:naive_datetime, guard_action:guard_str.to_string()};
        timestamp_data.push(timestamp);
    }
    
    timestamp_data.sort_by_key(|key| key.time_stamp);
    
    return timestamp_data;
}

fn initiate_guard_vector(timestamp_data: &mut Vec<TimeStampData>) -> Vec<Guard> 
{
    let mut guard_vector: Vec<Guard> = Vec::new();
    let mut numbers: Vec<u32>= Vec::new();
    for guard_data in timestamp_data {
        let re = Regex::new(r"#(\d+)").unwrap();
        if let Some(caps) = re.captures(&guard_data.guard_action) {
            if let Some(guard_number) = caps.get(1) {
                let number = guard_number.as_str().parse().unwrap();
                numbers.push(number);
            }
        }
    }
    let hash_set:Vec<u32> = numbers.into_iter().collect::<HashSet<_>>().into_iter().collect();

    for number in hash_set
    {
        guard_vector.push(Guard{number,minutes_sleep_time:0,sleepiest_minute:0,minute_hashmap:HashMap::new()});
    }

    guard_vector
}

fn fill_guard_vector(mut timestamp_data: Vec<TimeStampData>) -> Vec<Guard> {
    let mut guard_vector = initiate_guard_vector(&mut timestamp_data);
    let mut last_number = 0;
    let mut falls_asleep = NaiveDateTime::from_timestamp(0, 0);
    let mut wakes_up = NaiveDateTime::from_timestamp(0, 0);

    for guard_data in timestamp_data {
        if guard_data.guard_action.contains("falls asleep")
        {
            falls_asleep = guard_data.time_stamp;
        } 
        else if guard_data.guard_action.contains("wakes up")
        {
            wakes_up = guard_data.time_stamp;
            let duration: Duration = wakes_up.signed_duration_since(falls_asleep);
            for guard in &mut guard_vector{
                if guard.number == last_number {
                    guard.minutes_sleep_time+=duration.num_minutes();
                    let fall_asleep_minute = falls_asleep.minute();
                    let wake_up_minute = wakes_up.minute();
                    
                    if fall_asleep_minute < wake_up_minute{
                        for number in fall_asleep_minute..wake_up_minute {
                            *guard.minute_hashmap.entry(number).or_insert(0) += 1;
                        }
                    } else {
                        for number in wake_up_minute..fall_asleep_minute {
                            *guard.minute_hashmap.entry(number).or_insert(0) += 1;
                        }
                    }
                }
            }
        } 
        else 
        {
            let re = Regex::new(r"#(\d+)").unwrap();
            if let Some(caps) = re.captures(&guard_data.guard_action) {
                if let Some(guard_number) = caps.get(1) {
                    last_number = guard_number.as_str().parse().unwrap();
                }
            }
        }        
    }

    for guard in &mut guard_vector{
            let max_key = guard.minute_hashmap
            .iter()
            .max_by_key(|&(_, value)| value) // Find the pair with the highest value
            .map(|(&key, _)| key); // Extract the key
        guard.sleepiest_minute=max_key.unwrap_or(0);
    }

    guard_vector
}


fn solution(input: &str) -> (u32, u32) 
{
    let timestamp_data = parse_data(&input);
    let guard_vector = fill_guard_vector(timestamp_data);
    
    let mut answer_part_1 = 0;
    let mut answer_part_2 =0;
    let mut largest_sleep = 0;
    let mut largest_count:u32 = 0;
    
    for guard in &guard_vector {
        if guard.minutes_sleep_time > largest_sleep
        {
            largest_sleep = guard.minutes_sleep_time;
            answer_part_1 = guard.number * guard.sleepiest_minute;
        }
    }

    for guard in &guard_vector {
        for value in &guard.minute_hashmap 
        {
            if value.1 > &largest_count {
                largest_count = *value.1;
                answer_part_2 = guard.number * guard.sleepiest_minute;
            }
        }
    }

    (answer_part_1, answer_part_2)
}



fn main() 
{
    let current_dir = env::current_dir().unwrap();
    let file_path = current_dir.join("input");
    let filepath_example = current_dir.join("exampleinput2018day4");
    let contents = fs::read_to_string(file_path).unwrap();
    let contents_example = fs::read_to_string(filepath_example).unwrap();

    let answer_example = solution(&contents_example);
    let answer = solution(&contents);

    assert_eq!(answer_example.0, 240);
    assert_eq!(answer.0,142515);
    println!("answer part1 {}", answer.0);

    assert_eq!(answer_example.1,4455);
    assert_eq!(answer.1, 5370);
    println!("answer part2 {}", answer.1);
}




    