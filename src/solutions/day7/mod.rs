use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Hand {
    cards: String,
    bid: u32,
    strength: u8
}

pub fn day7() {
    let contents = fs::read_to_string("./src/solutions/day7/input.txt")
        .expect("Should have been able to read the file");


    camel_cards(&contents);
}

fn camel_cards(input: &String) -> u32{

    let get_cards = |h: &str| -> Hand { Hand {
        cards: String::from(&h[..5]),
        bid: h[6..].parse::<u32>().unwrap(),
        strength: estimate_type(&h[..5]),
    } };

    // println!("{}", input);
    let hands_line = input.lines();
    let mut hands: Vec<Hand> = hands_line.map(get_cards).collect();
    hands.sort_by(|a, b| { if a.strength != b.strength { a.strength.cmp(&b.strength) } else {return compare_labels(a, b)}});
    println!("{:?}", hands);

    let mut sum = 0;
    for (index, hand) in hands.iter().enumerate() {
        sum += (index as u32+ 1) * hand.bid;
    }

    println!("{:?}", sum);
    0
}

fn estimate_type(h: &str) -> u8{
    let mut cards:HashMap<char, u8> = HashMap::new();
    for char in h.chars() {
        let prev = cards.get(&char).unwrap_or(&0);
        cards.insert(char, prev + 1);
    }
    let mut found_full_house = false;
    let mut found_pair = false;
    for value in cards.values() {
        if *value == 5 { return 6; }
        else if *value == 4 {return 5;}
        else if *value == 3 {found_full_house = true;}
        else if *value == 2 {
            if found_pair { return 2;
            } else {
                found_pair = true;
            }
        }
    }

    if found_pair {
        return if found_full_house { 4 } else { 1 }
    } else if found_full_house {
        return 3;
    }

    0
}

fn compare_labels(a: &Hand, b: &Hand) -> Ordering{
    for i in 0..5 {
        let a_char = a.cards.chars().nth(i).unwrap();
        let b_char = b.cards.chars().nth(i).unwrap();
        if get_card_value(a_char) > get_card_value(b_char) {
            return Ordering::Greater;
        } else if get_card_value(a_char) < get_card_value(b_char) {
            return Ordering::Less;
        }
    }
    return Ordering::Equal;
}

fn get_card_value(x: char) -> u32{
    return match x {
        'T' => {10},
        'J' => {11},
        'Q' => {12},
        'K' => {13},
        'A' => {14},
        _ => { x.to_digit(10).unwrap() }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let input = String::from(
        "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483");
        camel_cards(&input);
        assert_eq!(1, 2 -1 );
    }
}