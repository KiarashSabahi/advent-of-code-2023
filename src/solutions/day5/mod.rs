use std::fs;
use std::num;

pub fn seed_fertilizer() {

    let contents = fs::read_to_string("./src/solutions/day5/input.txt")
        .expect("Should have been able to read the file");
//
//     let contents ="\
// seeds: 79 14 55 13
//
// seed-to-soil map:
// 50 98 2
// 52 50 48
//
// soil-to-fertilizer map:
// 0 15 37
// 37 52 2
// 39 0 15
//
// fertilizer-to-water map:
// 49 53 8
// 0 11 42
// 42 0 7
// 57 7 4
//
// water-to-light map:
// 88 18 7
// 18 25 70
//
// light-to-temperature map:
// 45 77 23
// 81 45 19
// 68 64 13
//
// temperature-to-humidity map:
// 0 69 1
// 1 0 69
//
// humidity-to-location map:
// 60 56 37
// 56 93 4";

    // println!("{}", contents);

    let mut content_lines = contents.lines().filter(|l| !l.is_empty()).into_iter();
    let seeds_line
        = &content_lines.next().unwrap();
    let seeds: Vec<i64> = seeds_line[7..].split_whitespace().map(|c| c.parse::<i64>().unwrap()).collect();
    println!("{:?}", seeds);

    let mut maps:Vec<Vec<Vec<i64>>> = Vec::new();

    let mut area: Vec<Vec<i64>> = Vec::new();
    for line in content_lines {
        if line.contains(":") {
            if !area.is_empty() {
                maps.push(area);
                area = Vec::new();
            }
            // println!("{}", line);
            continue; }
        // println!("{}", line);
        let map = line.split_whitespace().map(|c| c.parse::<i64>().unwrap()).collect();
        area.push(map);
    }
    maps.push(area);
    println!("{:?}", maps);
    let mut lowest = i64::MAX;

    let test: Vec<_> = seeds[0..1].iter().cloned().collect();
    for seed in seeds {
        println!("\nseed: {}", seed);
        let mut temp = seed;
        'map: for map in &maps {
            println!("map: {:?}", map);
            for area in map {
                if temp >= area[1] && temp < area[1] + area[2] {
                    println!("area: {:?} seed: {}", area, temp);
                    temp = temp + area[0] - area[1];
                    println!("temp: {}", temp);
                    continue 'map;
                }
            }
        }
        if temp < lowest { lowest = temp; }
        println!("result: {}", lowest);
    }







}