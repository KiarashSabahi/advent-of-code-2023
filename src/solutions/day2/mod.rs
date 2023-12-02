use std::fs;

#[derive(Debug)]
struct Game {
    id: i32,
    blue: i32,
    red: i32,
    green: i32,
}

pub fn cube_conundrum() {
    let mut games: Vec<Game> = Vec::new();
    parser(&mut games);

    let mut sum = 0;
    for game in games {
        println!("{:?}", game);
        if game.blue <= 14 && game.green <= 13 && game.red <= 12 {
            sum += game.id;
        }
    }
    println!("{}", sum);
}

fn parser(games: &mut Vec<Game>) {
    let contents = fs::read_to_string("./src/solutions/day2/input.txt")
        .expect("Should have been able to read the file");
    // let contents = "Game 100: 2 green, 1 blue; 9 red, 8 green, 1 blue; 4 red, 10 green, 1 blue; 17 green, 8 red; 5 green, 1 blue, 7 red; 14 red, 12 green";
//     let contents = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    for (index, line) in contents.lines().enumerate() {
        let mut index_pointer = 0;
        index_pointer = line.find(':').unwrap();

        let id = &line[5..index_pointer];
        index_pointer +=2;
        games.push(process_cubes(&line[index_pointer..], index as i32));
    }
}

fn process_cubes(line: &str, index: i32) -> Game {


    // println!("{line}");
    let mut index_pointer = 0;
    let mut length_pointer = 1;

    let mut green_count= 0;
    let mut red_count= 0;
    let mut blue_count= 0;


    loop {
        let new = &line[index_pointer..];
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

        // println!("green: {green_count}");
        // println!("blue: {blue_count}");
        // println!("red: {red_count}");

        if index_pointer >= line.len() {
            return Game {
                id: index + 1,
                blue: blue_count,
                red: red_count,
                green: green_count,
            }
        }

        index_pointer += 2;
    }
}