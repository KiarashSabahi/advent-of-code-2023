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

    println!("\npart1: {:?}", part1(&contents));
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
            tilt_up(i, j, &mut dish);
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

fn tilt_up(i: usize, j: usize, dish: &mut Vec<Vec<char>>){
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
