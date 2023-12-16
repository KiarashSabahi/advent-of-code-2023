use std::collections::HashMap;
use std::fs;

pub fn parabolic_reflector_dish() {
    let contents = fs::read_to_string("./src/solutions/day14/input.txt")
        .expect("Should have been able to read the file");

    println!("part1: {:?}", part1(&contents));
    println!("part2: {:?}", part2(&contents));
}

fn part1(contents: &String) -> usize {
    let mut dish:Vec<Vec<char>> = parse(contents);
    for i in 0..dish.len() {
        for j in 0..dish[i].len() {
            if dish[i][j] != 'O' { continue; }
            tilt_north(i, j, &mut dish);
        }
    }
    calculate_weight(dish)
}

fn part2(contents: &String) -> usize {
    let mut dish:Vec<Vec<char>> = parse(contents);
    for _ in 0..1000 {
        rotate(&mut dish);
    }
    calculate_weight(dish)
}

fn parse(input: &String) -> Vec<Vec<char>>{
    let mut dish:Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        if !line.is_empty() {
            dish.push(line.chars().collect::<Vec<char>>())
        }
    }
    dish
}

fn calculate_weight(dish: Vec<Vec<char>>) -> usize {
    let mut sum = 0;

    for i in 0..dish.len() {
        for j in 0..dish[i].len() {
            if dish[i][j] == 'O' {
                sum += dish.len() - i;
            }
        }
    }

    sum
}

fn rotate(dish: &mut Vec<Vec<char>>) {

    let height = dish.len() - 1;
    let width = dish[0].len() - 1;

    for i in 0..height + 1 {
        for j in 0..width + 1 {
            if dish[i][j] != 'O' { continue; }
            tilt_north(i, j, dish);
        }
    }

    for i in 0..height + 1 {
        for j in 0..width + 1 {
            if dish[i][j] != 'O' { continue; }
            tilt_west(i, j, dish);
        }
    }

    for i in 0..height + 1 {
        for j in 0..width + 1 {
            if dish[height-i][j] != 'O' { continue; }
            tilt_south(height - i, j, dish);
        }
    }

    for i in 0..height + 1 {
        for j in 0..width + 1 {
            if dish[i][width - j] != 'O' { continue; }
            tilt_east(i, width - j, dish);
        }
    }
}

fn tilt_north(i: usize, j: usize, dish: &mut Vec<Vec<char>>){
    let mut target = i;
    while target != 0{
        if dish[target - 1][j] != '.'{
            break;
        } else {
            target -= 1;
        }
    }

    if target != i {
        dish[i][j] = '.';
        dish[target][j] = 'O';
    }
}

fn tilt_west(i: usize, j: usize, dish: &mut Vec<Vec<char>>){
    let mut target = j;
    while target != 0{
        if dish[i][target - 1] != '.'{
            break;
        } else {
            target -= 1;
        }
    }

    if target != j {
        dish[i][j] = '.';
        dish[i][target] = 'O';
    }
}

fn tilt_south(i: usize, j: usize, dish: &mut Vec<Vec<char>>){
    let mut target = i;
    while target != dish.len() - 1{
        if dish[target + 1][j] != '.'{
            break;
        } else {
            target += 1;
        }
    }

    if target != i {
        dish[i][j] = '.';
        dish[target][j] = 'O';
    }
}

fn tilt_east(i: usize, j: usize, dish: &mut Vec<Vec<char>>){
    let mut target = j;
    while target != dish[0].len() - 1{
        if dish[i][target + 1] != '.'{
            break;
        } else {
            target += 1;
        }
    }

    if target != j {
        dish[i][j] = '.';
        dish[i][target] = 'O';
    }
}