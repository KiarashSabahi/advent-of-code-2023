use std::fs;

pub fn point_of_incidence () {
    let contents = fs::read_to_string("./src/solutions/day13/input.txt")
        .expect("Should have been able to read the file");

//     let contents = String::from(
// "#.##..##.
// ..#.##.#.
// ##......#
// ##......#
// ..#.##.#.
// ..##..##.
// #.#.##.#.
//
// #...##..#
// #....#..#
// ..##..###
// #####.##.
// #####.##.
// ..##..###
// #....#..#");

    println!("{}", part1(&contents));
}

fn part1(contents: &String) -> usize {
    println!("{}", contents);
    let mut maps: Vec<Vec<Vec<char>>> = Vec::new();
    let mut map: Vec<Vec<char>> = Vec::new();
    
    for line in contents.lines() {
        if line.is_empty() {
            maps.push(map);
            map = Vec::new();
        } else {
            map.push(line.chars().collect::<Vec<char>>())
        }
    }
    maps.push(map);
    let mut sum = 0;
    for map in maps {
        // println!("{:?}", map);
        sum += check_vertical(&map).unwrap_or(0);
        sum += check_horizontal(&map).unwrap_or(0) * 100;
        println!("a  {:?}", check_vertical(&map).unwrap_or(0));
        println!("a  {:?}", check_horizontal(&map).unwrap_or(0));
    }

    sum
}

fn check_vertical(map: &Vec<Vec<char>>) -> Option<usize> {
    if map.is_empty() { return None }
    let height = map.len();
    let width = map[0].len();
    'a: for column in 1..width {
        let mut i = column - 1;
        let mut j = column;
        loop {
            for row in 0..height {
                // println!("{:?} and {:?}", map[row][i], map[row][j]);
                if map[row][i] != map[row][j] {
                    continue 'a;
                }
            }
            if i > 0 && j < width - 1 {
                i -= 1;
                j += 1;
            } else {
                break;
            }
        }
        if j == width - 1 || i == 0 {
            // for row in 0..height {
            //     println!("{:?}", map[row][i]);
            // }
            // println!("");
            // for row in 0..height {
            //     println!("{:?}", map[row][j]);
            // }
            return Some(column);
        }

    }
    None
}


fn check_horizontal(map: &Vec<Vec<char>>) -> Option<usize> {
    if map.is_empty() { return None }

    let height = map.len();
    let width = map[0].len();
    'a: for row in 1..height {
        // println!("check: {}", row);
        let mut i = row - 1;
        let mut j = row;
        loop {
            for column in 0..width {
                // println!("{:?} and {:?}", map[i][column], map[j][column]);
                if map[i][column] != map[j][column] {
                    continue 'a;
                }
            }
            if i > 0 && j < height - 1 {
                i -= 1;
                j += 1;
            } else {
                break;
            }
        }
        if j == height - 1 || i == 0 {
            // for column in 0..width {
            //     println!("{:?}", map[row][i]);
            // }
            // println!("");
            // for column in 0..width {
            //     println!("{:?}", map[row][j]);
            // }
            return Some(row);
        }

    }
    None
}