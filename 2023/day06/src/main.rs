use lib::filereader;

fn main() {
    let contents = filereader::_input("../input/day06");
    let input_data = parse_data(contents);

    let mut multiply = 1;
    for input in input_data{
        let mut count = 0;
        let time:i32 = input[0];
        for sec_button in 0..time {
            let distance = (time - sec_button) * sec_button;
            if distance> input[1] {
                count += 1;
            }
        }
        multiply*=count;
    }

    println!("multiply day one: {}",multiply);
    let mut multiply2:i128 = 1;
    let mut count = 0;
    let time:i128 = 42686985;
    let distance1:i128 = 284100511221341;
    for sec_button in 0..time {
        let distance = (time - sec_button) * sec_button;
        if distance > distance1 {
            count += 1;
        }
    }
    multiply2*=count;
    println!("multiply day two: {}",multiply2);

}

fn parse_data(contents: String) -> Vec<Vec<i32>> {
    let mut vec_hashmap:Vec<Vec<i32>> = vec![];
    let mut vect_int:Vec<Vec<i32>> = Vec::new();

    for content in contents.lines() {
        let content = content.replace("Time:", "");
        let content = content.replace("Distance:", "");
        let integers:Vec<i32> = content.split_whitespace().map(|number| number.parse::<i32>().unwrap()).collect();
        vect_int.push(integers);
    }

    for i in 0..vect_int[0].len(){
        let first = vect_int[0][i];
        let second = vect_int[1][i];
        vec_hashmap.push(vec![first,second]);
    }
    vec_hashmap
}
