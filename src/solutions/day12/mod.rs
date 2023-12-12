use std::fs;
use std::str::{Chars, from_utf8};

pub fn hot_springs() {
    let contents = fs::read_to_string("./src/solutions/day12/input.txt")
        .expect("Should have been able to read the file");
//     let contents = String::from(
//         "???.### 1,1,3
// .??..??...?##. 1,1,3
// ?#?#?#?#?#?#?#? 1,3,1,6
// ????.#...#... 4,1,1
// ????.######..#####. 1,6,5
// ?###???????? 3,2,1");
    println!("{}", part1(contents));

}

fn part1(contents: String) -> usize {
    let mut sum = 0;
    for line in contents.lines() {
        println!("{}", line);
        let mut split = line.split_whitespace();
        let pattern = split.next().unwrap().as_bytes();
        let numbers = split.next().unwrap().split(',').map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        let x = possible_ways(&pattern, &numbers, 0);
        println!("possible ways: {}", x);
        sum += x;
    }
    sum
}

fn possible_ways(pattern: &[u8], numbers: &[usize], len: usize) -> usize {
    // println!("{:?}", String::from_utf8(Vec::from(pattern)).unwrap().chars().collect::<Vec<_>>()[0]);
    // println!("{:?} {:?} {:?}", String::from_utf8(Vec::from(pattern)).unwrap(), numbers, len);

    if pattern.is_empty() {
        return if len == 0 && numbers.len() == 0 {
            1
        } else if numbers.len() == 1 && len == numbers[0] {
            1
        } else {
            0
        }
    }

    if numbers.is_empty() && len != 0 {
        return 0;
    }

    let mut ways: usize = 0;
    match pattern[0] {
        b'.' => {
            if len == 0 {
                ways = possible_ways(&pattern[1..], numbers, 0);
            } else if len != numbers[0] {
                ways = 0;
            } else {
                ways = possible_ways(&pattern[1..], &numbers[1..], 0);
            }
        },
        b'#' => {
            if len == 0 {
                ways = possible_ways(&pattern[1..], numbers, 1);
            } else {
                ways = possible_ways(&pattern[1..], numbers, len + 1);
            }
        },
        b'?' => {
            if len == 0 {
                ways += possible_ways(&pattern[1..], numbers, 0);
                ways += possible_ways(&pattern[1..], numbers, 1);
            } else {
                let mut ans =  possible_ways(&pattern[1..], numbers, len + 1);
                if len == numbers[0] {
                    ans += possible_ways(&pattern[1..], &numbers[1..], 0);
                }
                ways += ans;
                return ans;
            }
        },
        _ => ways = 0
    };

    ways
}
