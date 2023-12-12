use std::fs;


pub fn seed_fertilizer() {

    let contents = fs::read_to_string("./src/solutions/day5/input.txt")
        .expect("Should have been able to read the file");

    let mut content_lines = contents.lines().filter(|l| !l.is_empty()).into_iter();
    let seeds_line
        = &content_lines.next().unwrap();
    let seeds: Vec<u64> = seeds_line[7..].split_whitespace().map(|c| c.parse::<u64>().unwrap()).collect();

    let mut maps:Vec<Vec<Vec<u64>>> = Vec::new();

    let mut area: Vec<Vec<u64>> = Vec::new();
    for line in content_lines {
        if line.contains(":") {
            if !area.is_empty() {
                maps.push(area);
                area = Vec::new();
            }
            continue;
        }
        let map = line.split_whitespace().map(|c| c.parse::<u64>().unwrap()).collect();
        area.push(map);
    }
    maps.push(area);


    let mut lowest = u64::MAX;
    for i in (0..seeds.len()).step_by(2) {
        for j in seeds[i]..seeds[i]+seeds[i+1] {
            let mut temp = j;
            for map in &maps {
                for area in map {
                    if temp >= area[1] && temp < area[1] + area[2] {
                        temp = temp + area[0] - area[1];
                        break;
                    }
                }
            }
            if temp < lowest {
                lowest = temp;
            }
        }
    }
    println!("result for part 2: {}", lowest);

    lowest = u64::MAX;

    for seed in seeds {
        let mut temp = seed;
        for map in &maps {
            for area in map {
                if temp >= area[1] && temp < area[1] + area[2] {
                    temp = temp + area[0] - area[1];
                    break;
                }
            }
        }
        if temp < lowest { lowest = temp; }
    }

    println!("result for part 1: {}", lowest);
}