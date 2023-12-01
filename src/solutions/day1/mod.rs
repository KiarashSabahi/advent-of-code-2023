use std::fs;

pub fn trebuchet() {
    let contents = fs::read_to_string("./src/solutions/day1/input.txt")
        .expect("Should have been able to read the file");
    let mut sum = 0;

    for content in contents.lines() {
        let first: u32;
        let second: u32;
        let line = parse_line(content);

        let mut iterable1 = line.chars();
        loop {
            let ch = iterable1.next().unwrap();
            if ch.is_numeric() {
                first = ch.to_digit(10).unwrap();
                break;
            }
        }

        let mut iterable2 = line.chars().rev();
        loop {
            let ch = iterable2.next().unwrap();
            if ch.is_numeric() {
                second = ch.to_digit(10).unwrap();
                break;
            }
        }

        let number = first * 10 + second;
        sum += number;
    }
    println!("{sum}");
}

fn parse_line(line: &str) -> String {
    let mut parsed;
    parsed = line.replace("one", "o1e");
    parsed = parsed.replace("two", "t2o");
    parsed = parsed.replace("three", "t3e");
    parsed = parsed.replace("four", "f4r");
    parsed = parsed.replace("five", "f5e");
    parsed = parsed.replace("six", "s6x");
    parsed = parsed.replace("seven", "s7n");
    parsed = parsed.replace("eight", "e8t");
    parsed = parsed.replace("nine", "n9e");
    parsed
}
