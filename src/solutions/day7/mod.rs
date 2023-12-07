use std::fs;

pub fn day7() {
    let contents = fs::read_to_string("./src/solutions/day7/input.txt")
        .expect("Should have been able to read the file");


    camel_cards(&contents);
}

fn camel_cards(input: &String) -> u32{
    println!("{}", input);
    0
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