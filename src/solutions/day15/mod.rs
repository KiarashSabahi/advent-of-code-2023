use std::fs;

pub fn lens_library() {
    let contents = fs::read_to_string("./src/solutions/day15/input.txt")
        .expect("Should have been able to read the file");

    // let contents = String::from("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");

    println!("part1: {:?}", part1(&contents));
}

fn part1(input: &String) -> usize{
    println!("{}", input);

    let mut steps: Vec<&str> = input.split(",").collect();

    let mut total_sum = 0;

    for step in steps {
        let mut sum = 0;
        for char in step.chars() {
            // println!("{} {}", char, char as usize);
            sum += char as usize;
            sum = sum * 17;
            sum = sum%256;
        }

        total_sum += sum;
    }


    total_sum
}