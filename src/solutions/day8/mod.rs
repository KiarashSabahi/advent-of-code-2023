use std::collections::HashMap;
use std::fs;
use std::hash::Hash;

pub fn haunted_wasteland() {
    let contents = fs::read_to_string("./src/solutions/day8/input.txt")
        .expect("Should have been able to read the file");

    println!("{}", part1(&contents));
    println!("{}", part2(&contents));
}

fn part1(input: &String) -> u32 {
    let mut lines = input.lines();
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    let steps = lines.next().unwrap().chars().collect::<Vec<char>>();
    lines.next();
    for line in lines {
        map.insert(&line[0..3], (&line[7..10], &line[12..15]));
    }
    traverse_1(&map, &steps, "AAA")
}

fn part2(input: &String) -> u32 {
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    let steps = parse_map(&input, &mut map);
    let mut counts  = Vec::new();
    for key in map.keys() {
        if key.ends_with('A') {
            counts.push(traverse_2(&map, &steps, key));
        }
    }

    lcm(counts)
}

fn parse_map<'a>(input: &'a String, map: &mut HashMap<&'a str, (&'a str, &'a str)>) -> (Vec<char>) {
    let mut lines = input.lines();
    let steps = lines.next().unwrap().chars().collect::<Vec<char>>();
    lines.next();
    for line in lines {
        map.insert(&line[0..3], (&line[7..10], &line[12..15]));
    }

    return steps;
}


fn traverse_1(map: &HashMap<&str, (&str, &str)>, steps: &Vec<char>, start: &str) -> u32 {
    let mut current = start;
    let mut counter: u32 = 0;
    loop {
        if current == String::from("ZZZ") {
            return counter;
        }
        for step in steps {
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

fn traverse_2(map: &HashMap<&str, (&str, &str)>, steps: &Vec<char>, start: &str) -> u32 {
    let mut current = start;
    let mut counter: u32 = 0;
    loop {
        if current.ends_with("Z") {
            return counter;
        }
        for step in steps {
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

pub fn lcm(nums: Vec<u32>) -> u32 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(Vec::from(&nums[1..]));
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: u32, b: u32) -> u32 {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
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
        assert_eq!(part1(&input), 6 );
    }

    #[test]
    fn test2() {

    let input = String::from("LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)");
        assert_eq!(part2(&input), 6 );
    }
}