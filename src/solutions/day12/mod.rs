use std::collections::HashMap;
use std::fs;

pub fn hot_springs() {
    let contents = fs::read_to_string("./src/solutions/day12/input.txt")
        .expect("Should have been able to read the file");
    // let contents = String::from(
    //     ".??..??...?##. 1,1,3");
    println!("{}", part1(&contents));
    println!("{}", part2(&contents));

}

fn part1(contents: &String) -> usize {
    let mut sum = 0;
    for line in contents.lines() {
        let mut split = line.split_whitespace();
        let pattern = split.next().unwrap().as_bytes();
        let numbers = split.next().unwrap().split(',').map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        let mut cache = HashMap::new();
        let x = possible_ways(&mut cache, &pattern, &numbers, 0);
        sum += x;
    }
    sum
}

fn part2(contents: &String) -> usize {
    let mut sum = 0;
    for line in contents.lines() {
        let mut split = line.split_whitespace();
        let pattern = split.next().unwrap();
        let numbers = split.next().unwrap().split(',').map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        let mut new_pattern = String::from(pattern);
        for _ in 0..4 {
            new_pattern += "?";
            new_pattern += pattern;
        }
        let new_nums = (0..5).flat_map(|_| &numbers).copied().collect::<Vec<_>>();
        println!("{} {:?}", new_pattern, new_nums);
        let mut cache = HashMap::new();
        let x = possible_ways(&mut cache, &new_pattern.as_bytes(), &new_nums, 0);
        sum += x;
    }
    sum
}

fn possible_ways(cache: &mut HashMap<(usize, usize, usize), usize>,pattern: &[u8], numbers: &[usize], len: usize) -> usize {
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

    let key = (pattern.len(), len, numbers.len());
    if let Some(&x) = cache.get(&key) {
        return x;
    }

    let mut ways: usize = 0;
    match pattern[0] {
        b'.' => {
            if len == 0 {
                ways = possible_ways(cache, &pattern[1..], numbers, 0);
            } else if len != numbers[0] {
                ways = 0;
            } else {
                ways = possible_ways(cache, &pattern[1..], &numbers[1..], 0);
            }
        },
        b'#' => {
            if len == 0 {
                ways = possible_ways(cache, &pattern[1..], numbers, 1);
            } else {
                ways = possible_ways(cache, &pattern[1..], numbers, len + 1);
            }
        },
        b'?' => {
            if len == 0 {
                ways += possible_ways(cache, &pattern[1..], numbers, 0);
                ways += possible_ways(cache, &pattern[1..], numbers, 1);
            } else {
                let mut ans =  possible_ways(cache, &pattern[1..], numbers, len + 1);
                if len == numbers[0] {
                    ans += possible_ways(cache, &pattern[1..], &numbers[1..], 0);
                }
                ways += ans;
            }
        },
        _ => ways = 0
    };
    cache.insert(key, ways);
    ways
}
