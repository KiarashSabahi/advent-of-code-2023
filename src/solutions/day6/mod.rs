use std::fs;

pub fn wait_for_it() {
    let contents = fs::read_to_string("./src/solutions/day6/input.txt")
        .expect("Should have been able to read the file");

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
            if distance > distances[index] {
                count += 1;
            }
        }
        println!("{count}");
        result *= count;
    }

    println!("part 1: {result}");

    
    

    let mut lines = contents.lines();

    let time = lines.next().unwrap()[5..].replace(" ", "").parse::<u64>().unwrap();

    let distance = lines.next().unwrap()[9..].replace(" ", "").parse::<u64>().unwrap();

    let mut count = 0;
    for i in 0..time {
        let lap = i * (time - i); 
        if lap > distance {
            count += 1;
        }
    }
    println!("{count}");


}