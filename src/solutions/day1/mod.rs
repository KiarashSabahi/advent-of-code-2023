use std::fs;

const NUMBERS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

pub fn trebuchet() {
    let contents = fs::read_to_string("./src/solutions/day1/input.txt")
        .expect("Should have been able to read the file");
    let mut sum = 0;
    // let contents = "eight5fourtwotwo";

    for line in contents.lines() {
        let first: i32 ;
        let second: i32;

        let mut iterable1 = line.chars().enumerate();
        loop {
            let (index, ch) = iterable1.next().unwrap();
            if ch.is_numeric() {
                let value = get_value(&line[0..index], false);
                match value {
                    Some(number) => first = number,
                    None => first = ch.to_digit(10).unwrap() as i32
                }
                break;
            }
        }

        let mut iterable2 = line.chars().rev().enumerate();
        loop {
            let (index, ch) = iterable2.next().unwrap();
            if ch.is_numeric() {
                let value = get_value(&line[line.len() - index..], true);
                match value {
                    Some(number) => second = number,
                    None => second = ch.to_digit(10).unwrap() as i32
                }
                break;
            }
        }

        let number = first * 10 + second;
        sum = sum + number;
    }
    println!("{sum}");
}

fn get_value(x: &str, reverse: bool) -> Option<i32>{
    for index in 0..x.len() + 1 {
        for i in 0..9 {
            if reverse {
                if x[x.len() - index..].contains(NUMBERS[i]) {
                    return Some((i + 1) as i32);
                }
            } else {
                if x[..index].contains(NUMBERS[i]) {
                    return Some((i + 1) as i32);
                }
            }
        }
    }

    None
}