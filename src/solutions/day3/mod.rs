use std::fs;
use std::collections::HashMap;

pub fn gear_ratios() {
    let contents = fs::read_to_string("./src/solutions/day3/input.txt")
        .expect("Should have been able to read the file");

    let mut input = Vec::new();
    let mut ratios: HashMap<(usize, usize), (i32, bool)> = HashMap::new();
    contents.lines().for_each(|line| { input.push(line) });

    let mut sum: i32 = 0;
    for i in 0..input.len() {
        let chars: Vec<_> = input[i].chars().collect();
        let chars_len = chars.len();
        let mut leftover = 0;
        for j in 0..chars_len {

            if leftover > 0 {
                leftover -= 1;
                continue;
            }

            if chars[j].is_numeric() {
                while j + leftover < chars_len && chars[j + leftover].is_numeric() {
                    leftover += 1;
                }
                sum += check_neighbors(&input, i, j, leftover, chars_len, &mut ratios);
            }
        }
    }
    let res = ratios.values().filter(|(_, condition)| *condition).fold(0, |acc, &(value, _)| acc + value);
    println!("sum: {}", sum);
    println!("res: {}", res);
}

fn check_neighbors(input: &Vec<&str>, i: usize, j: usize, leftover: usize, length: usize, ratios: &mut HashMap<(usize, usize), (i32, bool)>) -> i32 {
    let number = &input[i][j..j+leftover];

    let start_x = if i == 0 { 0 } else { i - 1};
    let start_y = if j == 0 { 0 } else { j - 1};
    let end_x = if i == input.len() - 1 { input.len()} else { i + 2};
    let end_y = if j + leftover > length - 1 { length } else { j + leftover + 1 };

    let lines = &input[start_x..end_x];
    for (index_x, line) in lines.iter().enumerate() {
        let (is_neighbor, is_astrix, index_y) = is_neighbor_with_symbol(&line[start_y..end_y]);
        if is_neighbor {
            let number = number.parse::<i32>().unwrap();

            if is_astrix {
                let x = if i != 0 { index_x } else { index_x + 1 };
                let y = if j != 0 { index_y } else { index_y + 1 };
                let key = (x + i, y + j);
                calculate_ratio(ratios, key, number);
            }

            return number;
        }
    }
    0
}

fn is_neighbor_with_symbol(line: &str) -> (bool, bool, usize){
    for (index, ch) in line.chars().enumerate() {
        if is_symbol(&ch) { return (true, ch == '*', index)};
    }
    (false, false, 0)
}

fn calculate_ratio(ratios: &mut HashMap<(usize, usize), (i32, bool)>, key: (usize, usize), number: i32) {
    let contains = ratios.contains_key(&key);
    if contains {
        let (prev, _) = ratios.get(&key).unwrap();
        ratios.insert(key, (prev * number, true));
    } else {
        ratios.insert(key, (number, false));
    }
}

fn is_symbol(char: &char) -> bool {
    !char.is_numeric() && char != &'.'
}