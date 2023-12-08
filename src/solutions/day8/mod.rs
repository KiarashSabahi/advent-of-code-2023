use std::collections::HashMap;
use std::fs;
use std::hash::Hash;

pub fn haunted_wasteland() {
    let contents = fs::read_to_string("./src/solutions/day8/input.txt")
        .expect("Should have been able to read the file");

    part1(contents);
}

fn part1(input: String) -> u32 {
    let mut lines = input.lines();
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    let steps = lines.next().unwrap().chars().collect::<Vec<char>>();
    lines.next();
    for line in lines {
        map.insert(&line[0..3], (&line[7..10], &line[12..15]));
    }
    traverse(&map, steps)
}

fn traverse(map: &HashMap<&str, (&str, &str)>, steps: Vec<char>) -> u32 {
    let mut current = "AAA";
    let mut counter: u32 = 0;
    loop {
        if current == String::from("ZZZ") {
            println!("{}", counter);
            return counter;
        }
        for step in &steps {
            let (left, right) = map.get(&current).unwrap();
            if step == &'R' {
                current = right;
            } else {
                current = left;
            }
            counter += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let input = String::from(
            "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)");
        assert_eq!(part1(input), 6 );
    }
}