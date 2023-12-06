use std::fs;

pub fn wait_for_it() {
    let contents = fs::read_to_string("./src/solutions/day6/input.txt")
        .expect("Should have been able to read the file");

    println!("{contents}");

}