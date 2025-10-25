use std::sync::mpsc::Receiver;

use lib::filereader;

fn main()
{
    let contents = filereader::_input("../input/day05");
    let answerp1 = reduce_polymer(contents.clone());
    println!("{}",answerp1);
    assert_eq!(answerp1, 11264);
    let answerp2 = reduce_polymer_with_exclusion(contents);
    println!("{}",answerp2);
    assert_eq!(answerp2, 4552);
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

fn reduce_polymer_with_exclusion(polymer: String) -> usize{

    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let mut vec:Vec<usize> = Vec::new();

    for ch in alphabet.chars()
    {
        let result = polymer.chars().filter(|&c| c != ch.to_ascii_uppercase() && c != ch)
            .collect();
    
        let length = reduce_polymer(result);
        vec.push(length);
    }
    
    let size = vec.iter().min().unwrap();
    *size
}

#[cfg(test)]
mod tests {
    use crate::{reduce_polymer, reduce_polymer_with_exclusion};

    #[test]
    fn test1() {
        let test_input = "dabAcCaCBAcCcaDA";
        let polymer = reduce_polymer(test_input.to_string());
        assert_eq!(polymer, 10);
    }

    #[test]
    fn test2() {
        let test_input = "dabAcCaCBAcCcaDA";
        let polymer = reduce_polymer_with_exclusion(test_input.to_string());
        assert_eq!(polymer, 4); 
    }  
}