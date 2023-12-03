use std::fs;
use regex::Regex;
use std::collections::HashMap;

pub fn gear_ratios() {
    println!("Started");
    let contents = fs::read_to_string("./src/solutions/day3/input.txt")
        .expect("Should have been able to read the file");
//     let contents =
// "467..114..
// ...*......
// ..35..633.
// ......#...
// 617*......
// .....+.58.
// ..592.....
// ......755.
// ...$.*....
// .664.598..";

    let mut input = Vec::new();
    let mut part2: HashMap<(usize, usize), (i32, bool)> = HashMap::new();
    let _ = contents.lines().for_each(|line| { input.push(line) });

    let mut sum: i32 = 0;
    for i in 0..input.len() {
        // println!("Line: {}", i + 1);
        let chars: Vec<_> = input[i].chars().collect();
        let chars_len = chars.len();
        let mut leftover = 0;
        for mut j in 0..chars_len {

            if leftover > 0 {
                leftover -= 1;
                continue;
            }

            if chars[j].is_numeric() {
                while j + leftover < chars_len && chars[j + leftover].is_numeric() {
                    // println!("{}", chars[j]);
                    leftover += 1;
                }
                println!("----\n\n");

                sum += check_neighbors(&input, i, j, leftover, chars_len, &mut part2);
            }

        }

    }
    let res = part2.values().filter(|(value, condition)| *condition).fold(0, |acc, &(value, _)| acc + value);
    println!("sum: {}", sum);
    println!("res: {}", res);
}

fn check_neighbors(input: &Vec<&str>, i: usize, j: usize, leftover: usize, length: usize, part2: &mut HashMap<(usize, usize), (i32, bool)>) -> i32 {
    let number = &input[i][j..j+leftover];
    println!("number: {}", number);
    
    let start_x = if i == 0 { 0 } else { i - 1};
    let start_y = if j == 0 { 0 } else { j - 1};
    let end_x = if i == input.len() - 1 { input.len()} else { i + 2};
    let end_y = if j + leftover > length - 1 { length } else { j + leftover + 1 };

    // println!("{} {} {} {}", start_x, end_x, start_y, end_y);
    let lines = &input[start_x..end_x];
    for (index_x, line) in lines.iter().enumerate() {
        let (is_neighbor, is_astrix, index_y) = is_neighbor_with_symbol(&line[start_y..end_y]);
        if is_neighbor {
            let number = number.parse::<i32>().unwrap();

            if is_astrix {
                let x = if i != 0 { index_x } else { index_x + 1 };
                let y = if j != 0 { index_y } else { index_y + 1 };
                let key = (x + i, y + j);
                println!("index_x: {:?} i: {:?} index_y:{:?} j: {:?} {:?}", x, i, y, j, key);
                let contains = part2.contains_key(&key);
                if contains {
                    let (prev, _) = part2.get(&key).unwrap();
                    part2.insert(key, (prev * number, true));
                } else {
                    part2.insert(key, (number, false));
                }
            }

            return number;
        }
    }
    return 0;
}

fn is_neighbor_with_symbol(line: &str) -> (bool, bool, usize){
    println!("{}", line);
    let re = Regex::new(r#"[!@#$%^&*()\-=_/?<>\[\]\\+{}]"#).expect("Invalid regex pattern");
    for (index, ch) in line.chars().enumerate() {
        if is_symbol(&ch) { return (true, ch == '*', index)};
    }
    return (false, false, 0);
}

fn is_symbol(char: &char) -> bool {
    !char.is_numeric() && char != &'.'
}