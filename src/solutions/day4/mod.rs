use std::fs;

pub fn scratchcards() {
    println!("started");

    let contents = fs::read_to_string("./src/solutions/day4/input.txt")
        .expect("Should have been able to read the file");

//
//     let contents = "\
// Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
// Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
// Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
// Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
// Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
// Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    // println!("{}", contents);
    let mut sum = 0;

    for line in contents.lines() {
        let mut matches = 0;
        // let mut iter = line.chars();
        // while let Some(ch) = iter.next() {
        //     if ch == ':' {
        //         iter.next();
        //         break
        //     }
        //
        // }
        let i = line.find(":").unwrap();
        let j = line.find("|").unwrap();
        let target = &line[i + 2..j - 1];
        let current = &line[j + 2..];
        let target_nums = extract_numbers(target);
        let current_nums = extract_numbers(current);
        for num in &current_nums {
            if target_nums.contains(&num) {
                // println!("{}", num);
                matches += 1;
            }
        }
        println!("{:?}", current_nums);
        println!("{:?}", target_nums);
        println!("{:?}", matches);

        if matches != 0 {         sum += (2_i32.pow(matches - 1)); }
        println!("{}", sum);

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