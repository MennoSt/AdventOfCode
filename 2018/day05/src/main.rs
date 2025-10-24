use std::sync::mpsc::Receiver;

use lib::filereader;

fn main()
{
    let contents = filereader::_input("../input/day05");
    let answer = reduce_polymer(contents);
    println!("{}",answer);
    assert_eq!(answer, 11264);
    // filereader::_print_content(contents.as_str()); 

}

fn reduce_polymer(polymer: String) -> usize{

    let mut index = 0;
    let mut reduced_polymer = polymer.to_string().clone();
    let mut length= reduced_polymer.len() -1;

    while index < length {
        
       let x = reduced_polymer.chars().nth(index+1).unwrap(); 
       let y = reduced_polymer.chars().nth(index).unwrap();

       let same = x.to_ascii_lowercase() == y.to_ascii_lowercase();
       let upper_and_lowercase =  x.is_lowercase() != y.is_lowercase(); 
       if same && upper_and_lowercase
       {
            reduced_polymer.remove(index+1);
            reduced_polymer.remove(index);
            if index >= 1 
            {
                index -= 1;
            }
            else {
                index= 0;
            }

            length = reduced_polymer.len() -1;
       } else {
           index+=1
       }
    }

    reduced_polymer.len()
}

#[cfg(test)]
mod tests {
    use crate::reduce_polymer;

    #[test]
    fn test1() {
        let test_input = "dabAcCaCBAcCcaDA";
        let polymer = reduce_polymer(test_input.to_string());
        assert_eq!(polymer, 10);
    }

    #[test]
    fn test2() {
        // let part2 = part2(TESTINPUT, 7);
        // assert_eq!(part2, "6,1");
    }  
}