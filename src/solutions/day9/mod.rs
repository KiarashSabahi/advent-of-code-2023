use std::fs;

pub fn mirage_maintenance() {
    let contents = fs::read_to_string("./src/solutions/day9/input.txt")
        .expect("Should have been able to read the file");

    println!("{}", part1(&contents));
    println!("{}", part2(&contents));
}

fn part1(input: &str) -> i32 {
    let mut sum = 0;
    for line in input.lines() {
        let nums:Vec<i32> = line.split_whitespace().map(|n| n.parse::<i32>().unwrap()).collect();
        sum += recursive_iteration_part1(nums);
    }

    sum
}

fn part2(input: &str) -> i32 {
    let mut sum = 0;
    for line in input.lines() {
        let nums:Vec<i32> = line.split_whitespace().map(|n| n.parse::<i32>().unwrap()).collect();
        sum += recursive_iteration_part2(nums);
    }

    sum
}

fn recursive_iteration_part1(line: Vec<i32>) -> i32 {
    let length = line.len();
    if line[length - 1] == 0 && line[0] == 0 {
        0
    } else {
        let mut next_line: Vec<i32> = Vec::new();
        for i in 0..length-1 {
            next_line.push(line[i + 1] - line[i])
        }
        recursive_iteration_part1(next_line) + line[length - 1]
    }
}

fn recursive_iteration_part2(line: Vec<i32>) -> i32 {
    let length = line.len();
    if line[length - 1] == 0 && line[0] == 0 {
        0
    } else {
        let mut next_line: Vec<i32> = Vec::new();
        for i in 0..length-1 {
            next_line.push(line[i + 1] - line[i])
        }
        line[0] - recursive_iteration_part2(next_line)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let input = String::from(
            "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45");
        assert_eq!(part1(&input), 114 );
    }

    #[test]
    fn test2() {
        let input = String::from(
            "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45");
        assert_eq!(part2(&input), 2 );
    }
}
