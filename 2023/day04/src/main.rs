mod filereader;

struct Card {
    _number:i32,
    copies:i32,
    winning_numbers: Vec<i32>,
    my_numbers: Vec<i32>,
}

fn main() {
    let contents = filereader::input("exampleinput2023day4");
    let points = calculate_points(contents);
    assert_eq!(points,13);

    let contents = filereader::input("../input/day04");
    let points = calculate_points(contents);
    assert_eq!(points,23678);

    let contents = filereader::input("exampleinput2023day4");
    let scratchcards = calculate_total_scratccards(contents);
    assert_eq!(scratchcards,30);

    let contents = filereader::input("../input/day04");
    let scratchcards = calculate_total_scratccards(contents);
    assert_eq!(scratchcards,15455663);
}

fn calculate_total_scratccards(contents: String) ->i32 {
    let mut vect_cards: Vec<Card> = parse_input_data(contents);
    let mut total_scratchcards = 0;

    for i in 0..vect_cards.len() {
        let mut win_numbers: usize = 0;
            for number in vect_cards[i].my_numbers.clone() {
                if vect_cards[i].winning_numbers.contains(&number)
                {
                    win_numbers += 1;
                }
            }
            for j in 1..(win_numbers+1) {
                if i+j <=vect_cards.len(){
                    vect_cards[i+j].copies += vect_cards[i].copies;
                }
            }
            total_scratchcards += vect_cards[i].copies;
    }
    total_scratchcards
}

fn calculate_points(contents: String) ->i32 {
    let vect_cards: Vec<Card> = parse_input_data(contents);
    let mut points = 0;
    for card in vect_cards {
        let mut win_numbers:u32 = 0;
        for number in card.my_numbers {
            if card.winning_numbers.contains(&number)
            {
                win_numbers += 1;
            }
        }
        points += 2_i32.pow(win_numbers)/2;
    }
    points
}

fn parse_input_data(contents: String) -> Vec<Card> {

    let mut vect_cards: Vec<Card>  = Vec::new();
    for content in contents.lines() {
        let example_str = content.replace(":", "");
        let example_str = example_str.replace("Card ", "");
        let mut vect_win_numbers_for_struct: Vec<i32> = Vec::new();
        let mut vec_my_numbers_for_struct:  Vec<i32> = Vec::new();

        let split_strings: Vec<&str> = example_str.split(" | ").collect();
        let winning_numbers_str: Vec<&str> = split_strings[0].split_whitespace().collect();
        let my_numbers_str: Vec<&str> = split_strings[1].split_whitespace().collect();

        for i in 1..winning_numbers_str.len() {
            vect_win_numbers_for_struct.push(winning_numbers_str[i].parse::<i32>().unwrap());
        }
        for num in my_numbers_str {
            vec_my_numbers_for_struct.push(num.parse::<i32>().unwrap())
        }

        let card =  Card {_number:winning_numbers_str[0].parse::<i32>().unwrap(),
                                copies:1,
                                winning_numbers:vect_win_numbers_for_struct,
                                my_numbers:vec_my_numbers_for_struct,};
        
        vect_cards.push(card);
    }
    return vect_cards;
}

