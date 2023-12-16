mod solutions;

use std::time::{Duration, Instant};

fn main() {
    let mut time = Duration::new(0, 0);
    let count  = 1;
    for _ in 0..count {
        println!("Start");
        let now = Instant::now();
        solutions::day13::point_of_incidence();

        let elapsed = now.elapsed();
        time += elapsed;

    }

    println!("Elapsed: {:.2?}", time/count);

}
