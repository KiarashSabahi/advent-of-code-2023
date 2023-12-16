use std::fs;
use crate::solutions::day16::Direction::*;

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(PartialEq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

pub fn the_floor_will_be_lava() {
    let contents = fs::read_to_string("./src/solutions/day16/input.txt")
        .expect("Should have been able to read the file");

//     let contents = String::from("\
//     .|...\\....
// |.-.\\.....
// .....|-...
// ........|.
// ..........
// .........\\
// ..../.\\\\..
// .-.-/..|..
// .|....-|.\\
// ..//.|....
// ");

//     let contents = String::from("\\........-.........\\................................|...
// ......-/.............|-.../.....|...........././..\\.....
// -.........................|.....\\...................|.\\.
// .......-........../.......\\.........|..../........-.-|..");

    println!("part1: {:?}", part1(&contents));
}

fn part1(input: &String) -> usize {
    println!("{}", input);

    let mut energized: Vec<(usize, usize, Direction)> = Vec::new();
    let mut map: Vec<Vec<char>> = Vec::new();
    input.lines().for_each(|l| map.push(l.chars().collect()));

    // println!("{:?}", map);
    println!("---");
    match map[0][0] {
    '.' => {process_beam(0, 0, RIGHT, &map, &mut energized);},
    '|' => {process_beam(0, 0, DOWN, &map, &mut energized);},
    '-' => {process_beam(0, 0, RIGHT, &map, &mut energized);},
    '/' => {process_beam(0, 0, UP, &map, &mut energized);},
    '\\' => {process_beam(0, 0, DOWN, &map, &mut energized);},
        _ => {}
    }
    // process_beam(0, 0, RIGHT, &map, &mut energized);
    println!("---");




    // println!("{:?}", energized);

    for (i, j, d) in energized {
        // if map[i][j] == '.' { map[i][j] = '#' }
        // if map[i][j] == '\\' { map[i][j] = ')' }
        // if map[i][j] == '/' { map[i][j] = '(' }
        // if map[i][j] == '-' { map[i][j] = '_' }
        // if map[i][j] == '|' { map[i][j] = 'I' }
        map[i][j] = '#'
    }
    let mut sum = 0;
    println!("\nnew map\n");
    for row in map {
        for char in row {
            if char == '#' { sum += 1; }
            print!("{}", char)
        }
        println!();
    }
    println!();
    sum
}

fn process_beam(i: usize, j: usize, direction: Direction, map: &Vec<Vec<char>>, energized: &mut Vec<(usize, usize, Direction)>) {
    let height = map.len();
    let width = map[0].len();

    match energized.iter().find(|(v_i, v_j, v_d) | v_i == &i && v_j == &j) {
        Some((x, y, dir)) => {
            // println!("x: {x}, y: {y}");
            if direction == *dir { return; }
        },
        None => {
            energized.push((i, j, direction));
        }
    }
    let (next_i, next_j) = get_next_pos(i, j, direction, map);
    // println!("{} {}", next_i, next_j);

    if next_i == i && next_j == j {return;}
    let next_tile = map[next_i][next_j];
    // println!("next: {next_tile}");

    match next_tile {
        '.' => {
            process_beam(next_i, next_j, direction, map, energized);
        },
        '|' => {
            if direction == RIGHT || direction == LEFT {
                process_beam(next_i, next_j, UP, map, energized);
                process_beam(next_i, next_j, DOWN, map, energized);
            } else {
                process_beam(next_i, next_j, direction, map, energized);
            }
        },
        '-' => {
            if direction == UP || direction == DOWN {
                process_beam(next_i, next_j, LEFT, map, energized);
                process_beam(next_i, next_j, RIGHT, map, energized);
            } else {
                process_beam(next_i, next_j, direction, map, energized);
            }
        },
        '/' => {
            match direction {
                UP => {
                    process_beam(next_i, next_j, RIGHT, map, energized);
                },
                DOWN => {
                    process_beam(next_i, next_j, LEFT, map, energized);
                },
                LEFT => {
                    process_beam(next_i, next_j, DOWN, map, energized);
                },
                RIGHT => {
                    process_beam(next_i, next_j, UP, map, energized);
                }
            }
        },
        '\\' => {
            match direction {
                UP => {
                    process_beam(next_i, next_j, LEFT, map, energized);
                },
                DOWN => {
                    process_beam(next_i, next_j, RIGHT, map, energized);
                },
                LEFT => {
                    process_beam(next_i, next_j, UP, map, energized);
                },
                RIGHT => {
                    process_beam(next_i, next_j, DOWN, map, energized);
                }
            }
        },
        _ => {},
    }




}

fn get_next_pos(i: usize, j: usize, direction: Direction, map: &Vec<Vec<char>>) -> (usize, usize) {
    match direction {
        UP => { 
            if i != 0 {
                return (i - 1, j);
            }
        },
        DOWN => {
            if i != map.len() - 1 {
                return (i + 1, j);
            }
        },
        LEFT => {
            if j != 0 {
                return (i, j - 1);
            }
        },
        RIGHT => {
            if j != map[0].len() - 1{
                return (i, j + 1);
            }
        }
    }

    (i, j)
}