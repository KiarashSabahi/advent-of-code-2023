use std::fs;

pub fn point_of_incidence () {
    let contents = fs::read_to_string("./src/solutions/day13/input.txt")
        .expect("Should have been able to read the file");

    println!("{}", part1(&contents));
    println!("{}", part2(&contents));
}

fn part1(contents: &String) -> usize {
    solve(&contents, false)
}

fn part2(contents: &String) -> usize {
    solve(&contents, true)
}

fn solve(contents: &String, part2: bool) -> usize {
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
        sum += check_vertical(&map, part2).unwrap_or(0);
        sum += check_horizontal(&map, part2).unwrap_or(0) * 100;
    }
    sum
}

fn check_vertical(map: &Vec<Vec<char>>, part2: bool) -> Option<usize> {
    if map.is_empty() { return None }
    let height = map.len();
    let width = map[0].len();
    'a: for column in 1..width {
        let mut i = column - 1;
        let mut j = column;
        let mut smudge = 0;

        loop {
            for row in 0..height {
                if map[row][i] != map[row][j] {
                    if smudge == 0 {
                        smudge = 1;
                    } else {
                        continue 'a;
                    }
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
            if part2 {
                if smudge == 1 {
                    return Some(column)
                }
            } else {
                return Some(column);

            }
        }

    }
    None
}


fn check_horizontal(map: &Vec<Vec<char>>, part2: bool) -> Option<usize> {
    if map.is_empty() { return None }

    let height = map.len();
    let width = map[0].len();
    'a: for row in 1..height {
        let mut i = row - 1;
        let mut j = row;
        let mut smudge = 0;
        loop {
            for column in 0..width {
                if map[i][column] != map[j][column] {
                    if smudge == 0 {
                        smudge = 1;
                    } else {
                        continue 'a;
                    }
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
            if part2 {
                if smudge == 1 {
                    return Some(row)
                }
            } else {
                return Some(row);
            }
        }
    }
    None
}