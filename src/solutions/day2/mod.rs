use std::fs;

pub fn cube_conundrum() {
    parser();
}

fn parser() {
    let mut sum_part1 = 0;
    let mut sum_part2 = 0;

    let contents = fs::read_to_string("./src/solutions/day2/input.txt")
        .expect("Should have been able to read the file");

    for (index, line) in contents.lines().enumerate() {
        let index_pointer = line.find(':').unwrap() + 2;
        let (bool, sum) = process_cubes(&line[index_pointer..]);
        if bool {
            sum_part1 += index + 1;
        }

        sum_part2 += sum;

    }
    println!("{}", sum_part1);
    println!("{}", sum_part2);
}

fn process_cubes(line: &str) -> (bool, i32) {


    let mut index_pointer = 0;
    let mut length_pointer;

    let mut green_count= 0;
    let mut red_count= 0;
    let mut blue_count= 0;


    loop {
        length_pointer = line[index_pointer..].find(' ').unwrap();
        let count = line[index_pointer..index_pointer + length_pointer].parse::<i32>().unwrap();
        index_pointer += length_pointer;
        let color_letter = &line[index_pointer + 1..2 + index_pointer];

        if color_letter == "g" {
            index_pointer += 6;
            if count > green_count {
                green_count = count;
            }
        } else if color_letter == "r" {
            index_pointer += 4;
            if count > red_count {
                red_count = count;
            }
        } else if color_letter == "b" {
            index_pointer += 5;
            if count > blue_count {
                blue_count = count;
            }
        } else {
            panic!("error parsing")
        }

        if index_pointer >= line.len() {
            return if blue_count <= 14 && green_count <= 13 && red_count <= 12 {
                (true, blue_count * red_count * green_count)
            } else {
                (false, blue_count * red_count * green_count)
            }
        }

        index_pointer += 2;
    }
}