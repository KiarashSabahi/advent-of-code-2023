use std::fs;

pub fn parabolic_reflector_dish() {
    let contents = fs::read_to_string("./src/solutions/day14/input.txt")
        .expect("Should have been able to read the file");

//     let contents = String::from("O....#....
// O.OO#....#
// .....##...
// OO.#O....O
// .O.....O#.
// O.#..O.#.#
// ..O..#O..O
// .......O..
// #....###..
// #OO..#....");

    println!("\npart1: {:?}", part2(&contents));
}

fn part1(contents: &String) -> usize {
    println!("{}\n", contents);

    let mut dish:Vec<Vec<char>> = Vec::new();
    for line in contents.lines() {
        if !line.is_empty() {
            dish.push(line.chars().collect::<Vec<char>>())
        }
    }

    let mut sum = 0;

    for i in 0..dish.len() {
        for j in 0..dish[i].len() {
            if dish[i][j] != 'O' { continue; }
            tilt_north(i, j, &mut dish);
        }
    }


    for i in 0..dish.len() {
        for j in 0..dish[i].len() {
            if dish[i][j] == 'O' {
                sum += dish.len() - i;
            }
        }
    }
    sum
}

fn part2(contents: &String) -> usize {
    println!("{}\n", contents);

    let mut dish:Vec<Vec<char>> = Vec::new();
    for line in contents.lines() {
        if !line.is_empty() {
            dish.push(line.chars().collect::<Vec<char>>())
        }
    }

    let mut sum = 0;

    for i in 0..1000 {
        println!("{}", 1000 - i);
        rotate(&mut dish);
    }
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
    // println!("\nnorth");
    for i in 0..dish.len() {
        for j in 0..dish[i].len() {
            if dish[i][j] != 'O' { continue; }
            tilt_north(i, j, dish);
        }
    }


    // for i in 0..dish.len() {
    //     for j in 0..dish[i].len() {
    //         print!("{}", dish[i][j]);
    //         if dish[i][j] == 'O' {
    //             // sum += dish.len() - i;
    //         }
    //     }
    //     println!();
    // }

    // println!("\nwest");
    for i in 0..dish.len() {
        for j in 0..dish[i].len() {
            if dish[i][j] != 'O' { continue; }
            tilt_west(i, j, dish);
        }
    }

    //
    // for i in 0..dish.len() {
    //     for j in 0..dish[i].len() {
    //         print!("{}", dish[i][j]);
    //         if dish[i][j] == 'O' {
    //             // sum += dish.len() - i;
    //         }
    //     }
    //     println!();
    // }


    // println!("\nsouth");
    for i in 0..dish.len() {
        for j in 0..dish[height-i].len() {
            if dish[height-i][j] != 'O' { continue; }
            tilt_south(height - i, j, dish);
        }
    }

    //
    // for i in 0..dish.len() {
    //     for j in 0..dish[i].len() {
    //         print!("{}", dish[i][j]);
    //         if dish[i][j] == 'O' {
    //             // sum += dish.len() - i;
    //         }
    //     }
    //     println!();
    // }

    // println!("\neast");
    for i in 0..dish.len() {
        for j in 0..dish[i].len() {
            if dish[i][width - j] != 'O' { continue; }
            tilt_east(i, width - j, dish);
        }
    }

    //
    // for i in 0..dish.len() {
    //     for j in 0..dish[i].len() {
    //         print!("{}", dish[i][j]);
    //         if dish[i][j] == 'O' {
    //             // sum += dish.len() - i;
    //         }
    //     }
    //     println!();
    // }
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