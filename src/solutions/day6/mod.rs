use core::time;
use std::fs;

pub fn wait_for_it() {
    let contents = fs::read_to_string("./src/solutions/day6/input.txt")
        .expect("Should have been able to read the file");

//     let contents = "Time:      7  15   30
// Distance:  9  40  200";

    println!("{contents}");

    let mut lines = contents.lines();

    let times: Vec<_> = lines.next().unwrap()[5..].split_whitespace().filter(|c| !c.is_empty()).map(|c| c.parse::<i32>().unwrap()).collect();
    println!("{:?}", times);

    let distances: Vec<_> = lines.next().unwrap()[9..].split_whitespace().filter(|c| !c.is_empty()).map(|c| c.parse::<i32>().unwrap()).collect();
    println!("{:?}", distances);

    let mut result = 1;

    for (index, time) in times.iter().enumerate() {
        let mut count = 0;
        for i in 0..*time {
            let distance = i * (time - i); 
            println!("time: {} i: {} time - i: {}", time, i, time - i);
            if distance > distances[index] {
                count += 1;
            }
        }
        println!("{count}");
        result *= count;
    }

    println!("{result}");

    


}