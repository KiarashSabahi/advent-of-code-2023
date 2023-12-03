use std::fs;
use regex::Regex;

pub fn gear_ratios() {
    println!("Started");
    let contents = fs::read_to_string("./src/solutions/day3/input.txt")
        .expect("Should have been able to read the file");
//     let contents = "467..114..
// ...*......
// ..35..633.
// ......#...
// 617*......
// .....+..58
// ..592.....
// ......755.
// ...$.*....
// .664.598..";

    let mut input = Vec::new();

    let re = Regex::new(r#"[!@#$%^&*()\-=_/?<>\[\]\\+{}]"#).expect("Invalid regex pattern");


    let _ = contents.lines().for_each(|line| { input.push(line) });

    let mut sum = 0;
    for i in 0..input.len() {
        let chars: Vec<_> = input[i].chars().collect();
        let mut left_over = 0;
        let chars_len = chars.len();
        for mut j in 0..chars_len {
            if chars[j].is_numeric() {
                println!("{:?} {:?}", chars[j], left_over);
                if left_over > 0 {
                    left_over -= 1;
                    continue;
                }
                let mut start = j;
                while j + 1 < chars_len && chars[j].is_numeric() {
                    j += 1;
                }
                let len = j - start;
                if len == 0 {
                    continue;
                }
                left_over += len - 1;


                let x_start = if start != 0 { start - 1 } else {0};
                let x_end = if j != chars.len() {j + 1 } else {j};
                let y_start= if i != 0 { i - 1 } else {0};
                let y_end = if i != input.len() - 1 {i + 2 } else {input.len() - 1};

                // println!("{} {} {} {} {}",i , x_start, x_end, y_start, y_end);

                let t = &input[y_start..y_end];
                for line in t {
                    if re.is_match(&line[x_start..x_end]){
                        let m = &input[i][start..start + len];
                        println!("sssssssssssss{}", m);
                        sum += m.parse::<i32>().unwrap();
                        break;
                    }
                }
                println!("{:?}", t);
            }

        }
        println!("next line")
    }
    println!("{}", sum)
}

fn is_symbol(char: &str) -> bool {
    !char.chars().next().unwrap().is_numeric() && char != "."
}