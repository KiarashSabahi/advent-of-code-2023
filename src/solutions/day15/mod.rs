use std::collections::HashMap;
use std::fs;
use std::hash::Hash;

pub fn lens_library() {
    let contents = fs::read_to_string("./src/solutions/day15/input.txt")
        .expect("Should have been able to read the file");

    println!("part1: {:?}", part1(&contents));
    println!("part2: {:?}", part2(&contents));
}

fn part1(input: &String) -> usize{

    let mut steps: Vec<&str> = input.split(",").collect();

    let mut total_sum = 0;

    for step in steps {
        total_sum += calculate_ascii(step);
    }


    total_sum
}

fn part2(input: &String) -> usize {

    let mut steps: Vec<&str> = input.split(",").collect();
    let mut boxes: HashMap<usize, Vec<(&str, usize)>> = HashMap::new();

    'a: for step in steps {
        if step.contains("-") {
            let original_word = step.split("-").collect::<Vec<&str>>()[0];
            let hash = calculate_ascii(original_word);
            boxes.entry(hash).and_modify(|vect| remove_entry(vect, original_word));
        } else if step.contains("=") {
            let original_word = step.split("=").collect::<Vec<&str>>()[0];
            let lens_power: usize = step.split("=").collect::<Vec<&str>>()[1].parse().unwrap();
            let hash = calculate_ascii(original_word);
            let lens_vector = boxes.entry(hash).or_insert(Vec::new());
            for lens in &mut *lens_vector {
                if lens.0 == original_word {
                    lens.1 = lens_power;
                    continue 'a;
                }
            }

            lens_vector.push((original_word, lens_power));
        }
    }

    let mut sum = 0;
    for (i, lens_vector) in &boxes {
        if lens_vector.len() == 0 {
            continue;
        }
        for (j, lenz) in lens_vector.iter().enumerate() {
            sum += (i + 1) * (j + 1) * lenz.1;
        }
    }
    sum
}


fn remove_entry(lens_vector: &mut Vec<(&str, usize)>, original_word: &str) {
    for index in 0..lens_vector.len() {
        if lens_vector[index].0 == original_word {
            lens_vector.remove(index);
            return;
        }
    }
}

fn calculate_ascii(chars: &str) -> usize {
    let mut sum = 0;
    for char in chars.chars() {
        sum += char as usize;
        sum = sum * 17;
        sum = sum%256;
    }
    sum
}