use std::fs;
use std::ops::Add;
use crate::solutions::day10::Position::{Bottom, Left, Right, Up};

#[derive(PartialEq, Debug)]
enum Position {
    Left,
    Right,
    Up,
    Bottom,
    None
}

pub fn pipe_maze() {
    let contents = fs::read_to_string("./src/solutions/day10/input.txt")
        .expect("Should have been able to read the file");
    let contents = String::from(
        "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L");
    println!("{}", part2(contents));
}

fn part1(input: String) -> usize{

    let mut page: Vec<Vec<char>> = Vec::new();
    let lines = input.lines();
    let mut start_cords = (0, 0);
    for (i, line) in lines.enumerate() {
        let mut row: Vec<char> = Vec::new();
        for (j, char) in line.chars().enumerate() {
            // println!("{} {} {}", char, i, j);
            row.push(char);
            if char == 'S' { start_cords = (i, j) }
        }
        page.push(row);
    }
    println!("page: \n-----\n{}\n-----\n", input);
    println!("start: {:?}", start_cords);
    search(&page, start_cords).len().div_ceil(2)
}

fn part2(input: String) -> usize{

    let mut page: Vec<Vec<char>> = Vec::new();
    let lines = input.lines();
    let mut start_cords = (0, 0);
    for (i, line) in lines.enumerate() {
        let mut row: Vec<char> = Vec::new();
        for (j, char) in line.chars().enumerate() {
            row.push(char);
            if char == 'S' { start_cords = (i, j) }
        }
        page.push(row);
    }
    println!("page: \n-----\n{}\n-----\n", input);
    let path = search(&page, start_cords);
    let page_width = page[1].len();
    let page_height = page.len();

    let mut new_string = String::new();
    
    for i in 0..page_height {
        for j in 0..page_width {
            if path.contains(&(i, j)) {
                new_string.push(page[i][j]);
            } else {
                new_string.push('.');
            }
        }
        new_string += "\n"
    }

    println!("new maze: \n{}", new_string);


    0
}

fn search(page: &Vec<Vec<char>>, start_cord: (usize, usize)) -> Vec<(usize, usize)>{
    //go up
    if start_cord.0 != 0  && ['F', '7', '|'].contains(&page[start_cord.0 - 1][start_cord.1]){
        let path = iterate(page, start_cord, Up);
        if path.len() != 0 {
            return path;
        }
    }


    //go right
    if start_cord.1 != page[1].len() - 1 && ['-', '7', 'J'].contains(&page[start_cord.0][start_cord.1 + 1]) {
        let path = iterate(page, start_cord, Right);
        if path.len() != 0 {
            return path;
        }
    }

    //go bot
    if start_cord.0 != page.len() - 1 && ['J', 'L', '|'].contains(&page[start_cord.0 + 1][start_cord.1]) {
        let path = iterate(page, start_cord, Bottom);
        if path.len() != 0 {
            return path;
        }
}

    //go left
    if start_cord.1 != 0 && ['F', 'L', '-'].contains(&page[start_cord.0][start_cord.1 - 1]) {
        let path = iterate(page, start_cord, Left);
        if path.len() != 0 {
            return path;
        }
    }

    Vec::new()
}

fn iterate(page: &Vec<Vec<char>>, start_cord: (usize, usize), next_pos: Position) -> Vec<(usize, usize)>{
    let mut prev_cord = start_cord;
    let mut current_cord = get_cord(next_pos, prev_cord, page);
    let mut len: usize = 0;
    let mut path: Vec<(usize, usize)> = Vec::new();
    loop {
        path.push(prev_cord);
        let next_char = page[current_cord.0][current_cord.1];
        if next_char == '.' {
            break;
        }
        if next_char == 'S' {
            return path;
        }
        len += 1;
        let next = get_next_pos(next_char, prev_cord.0, prev_cord.1, current_cord.0, current_cord.1);
        if next == current_cord {
            break;
        }
        prev_cord = current_cord;
        current_cord = (next.0, next.1);
    }
    Vec::new()
}

fn get_cord(pos: Position, cord: (usize, usize), page: &Vec<Vec<char>>) -> (usize, usize) {
    let width =  page[0].len();
    let height =  page.len();

    match pos {
        Left => {
            if cord.1 != 0 {
                return (cord.0, cord.1 - 1)
            }
        }
        Right => {
            if cord.1 != width - 1 {
                return (cord.0, cord.1 + 1)
            }
        }
        Up => {
            if cord.0 != 0 {
                return (cord.0 - 1, cord.1)
            }
        }
        Bottom => {
            if cord.0 != height - 1 {
                return (cord.0 + 1, cord.1)
            }
        }
        Position::None => {return cord}
    }
    cord
}

fn cord_to_pos(prev_i: usize, prev_j: usize, current_i:usize, current_j:usize) -> Position {
    if prev_i == current_i && prev_j + 1 == current_j {
        return Right
    } else if prev_i == current_i && prev_j - 1 == current_j {
        return Left
    } else if prev_i + 1 == current_i && prev_j == current_j {
        return Bottom
    } else if prev_i - 1 == current_i && prev_j == current_j {
        return Up

    }
    Position::None
}

fn get_next_pos(ch: char, prev_i: usize, prev_j: usize, current_i:usize, current_j:usize) -> (usize, usize){
    let next_pos = cord_to_pos(prev_i, prev_j, current_i, current_j);
    match ch {
        'F' => {
            if next_pos == Left {
                return (current_i + 1, current_j)
            } else if next_pos == Up {
                return (current_i, current_j + 1)
            }
        }
        '7' => {
            if next_pos == Right {
                return (current_i + 1, current_j)
            } else if next_pos == Up && current_j != 0 {
                return (current_i, current_j - 1)
            }
        }
        '|' => {
            if next_pos == Up && current_i != 0 {
                return (current_i - 1, current_j)
            } else if next_pos == Bottom {
                return (current_i + 1, current_j)
            }
        }
        '-' => {
            if next_pos == Left && current_j != 0 {
                return (current_i , current_j - 1)
            } else if next_pos == Right {
                return (current_i, current_j + 1)
            }
        }
        'J' => {
            if next_pos == Bottom && current_j != 0 {
                return (current_i , current_j - 1)
            } else if next_pos == Right && current_i != 0 {
                return (current_i - 1, current_j)
            }
        }
        'L' => {
            if next_pos == Bottom {
                return (current_i , current_j + 1)
            } else if next_pos == Left && current_i != 0 {
                return (current_i - 1, current_j)
            }
        }
        _ => {return (0, 0)}
    }
    (0, 0)
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let input = String::from(
"7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ");
        assert_eq!(part1(input), 8 );
    }

    #[test]
    fn test2() {
        let input = String::from(
"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........");
        assert_eq!(part2(input), 4 );
    }

}