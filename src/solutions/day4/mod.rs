use std::fs;

pub fn scratchcards() {
    println!("started");

    let contents = fs::read_to_string("./src/solutions/day4/input.txt")
        .expect("Should have been able to read the file");

    let mut sum = 0;

    for line in contents.lines() {
        let mut matches = 0;

        let i = line.find(":").unwrap();
        let j = line.find("|").unwrap();
        let target = &line[i + 2..j - 1];
        let current = &line[j + 2..];
        let target_nums = extract_numbers(target);
        let current_nums = extract_numbers(current);
        for num in &current_nums {
            if target_nums.contains(&num) {
                matches += 1;
            }
        }

        if matches != 0 {
            sum += 2_i32.pow(matches - 1);
        }

    }

    println!("{}", sum);
}

fn extract_numbers(target: &str) -> Vec<i32>{
    let mut numbers = Vec::new();
    let mut parts = target.split_whitespace().map(|s| s.parse::<i32>());

    loop {
        match parts.next() {
            Some(Ok(a)) => {
                numbers.push(a);
            }
            _ => {break;}
        }
    }

    numbers
}