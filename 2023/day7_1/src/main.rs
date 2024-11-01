mod filereader;
use std::{collections::HashMap, vec};

#[derive(PartialEq)]
#[derive(Debug)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}
struct Hand {
    cards:String,
    cards_score:i128,
    bid:i32,
    rank:i32,
    hand_type:HandType
}

fn count(str: &str, c:char) -> usize {
    let mut count = 0;
    for char in str.chars() {
        if char == c {
            count += 1;
        }
    }

    count
}

fn remove_char(mut vec_chars: Vec<char>, t: char) -> Vec<char>{
    
    while vec_chars.contains(&t) {
        vec_chars.remove(vec_chars.iter().position(|c| *c == t).unwrap());
    }
    return vec_chars
}

fn get_index(vec_chars: Vec<char>, char:char) -> usize 
{
    let x: usize = 14-vec_chars.iter().position(|&c| c == char).unwrap();
    x
}

fn main() {
    // let contents_ex = filereader::_input("exampleinput2023day7");
    // let sum = calculate_sum(contents_ex);
    // assert_eq!(sum, 6440);
    
    let contents = filereader::_input("input2023day7");
    let sum2 = calculate_sum(contents);
    println!("answer {}", sum2);
}

fn calculate_sum(contents: String) -> i32 {
    let mut vec_hand = fill_handtypes(contents);
    fill_card_scores(&mut vec_hand);

    let mut rank = 1;
    let hand_types = [HandType::HighCard, HandType::OnePair, HandType::TwoPair, HandType::ThreeOfAKind,
        HandType::FullHouse, HandType::FourOfAKind, HandType::FiveOfAKind];
    
    for hand_type in hand_types {
        for hand in vec_hand.iter_mut() {
            if hand.hand_type == hand_type {
                hand.rank =rank;
                rank+=1;
            }
        }
    }

    let mut sum = 0;
    for hand in vec_hand {
        if hand.hand_type == HandType::FullHouse{
            println!("card: {},score: {}, rank {}",hand.cards,hand.cards_score, hand.rank);
        }
        sum += hand.bid * hand.rank;
    }
    sum
}

fn fill_card_scores(vec_hand: &mut Vec<Hand>) {
    let vec_chars: Vec<char> = vec!['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3','2'];
    for hand in vec_hand.iter_mut() {
        let str = &hand.cards;
        for i in 0..5 {
            for char in vec_chars.clone(){
                if str.chars().nth(i).unwrap() == char {
                    let index = get_index(vec_chars.clone(),char) as i128;
                    hand.cards_score += index*30_i128.pow(5-i as u32);
                }
            }
        }
    }
    vec_hand.sort_by_key(|f|f.cards_score);
}

fn fill_handtypes(contents: String) -> Vec<Hand>{
    let vec_chars: Vec<char> = vec!['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3','2'];
    let mut vec_hand = parse_data(contents);
    for char in vec_chars.clone() {
        for hand in vec_hand.iter_mut() {
            if count(&hand.cards,char) == 5 {
                hand.hand_type = HandType::FiveOfAKind;
            }
            else if count(&hand.cards,char) == 4 {
                hand.hand_type = HandType::FourOfAKind;
            }
            else if count(&hand.cards,char) == 3 {
                hand.hand_type = HandType::ThreeOfAKind;
                let vec_chars_mut = remove_char(vec_chars.clone(), char);
                for vec in vec_chars_mut {
                    if count(&hand.cards,vec) == 2 {
                        hand.hand_type = HandType::FullHouse;
                                            }
                }
            }
            else if count(&hand.cards,char) == 2 && hand.hand_type!=HandType::FullHouse&& hand.hand_type!=HandType::ThreeOfAKind{
                hand.hand_type = HandType::OnePair;
                let vec_chars_mut = remove_char(vec_chars.clone(), char);
                for vec in vec_chars_mut {
                    if count(&hand.cards,vec) == 2 {
                        hand.hand_type = HandType::TwoPair;
                    }
                }
            }
    }
    }
    vec_hand
}


fn split_string_by_whitespace(text: &str) -> Vec<String> {
    let mut words = vec![];
    for word in text.split_whitespace() {
        words.push(word.to_string());
    }
    words
}

fn parse_data(contents: String) -> Vec<Hand> {
    let mut vec_hands: Vec<Hand> = Vec::new();

    for content in contents.lines() {
        let vect= split_string_by_whitespace(content);
        let hand = Hand { cards: vect[0].clone(),
            cards_score:0,
            bid:vect[1].parse::<i32>().unwrap(),
            rank:0,
            hand_type:HandType::HighCard};

        vec_hands.push(hand);
    }
    vec_hands
}