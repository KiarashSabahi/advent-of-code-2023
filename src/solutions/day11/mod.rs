use std::{fs, vec};

pub fn cosmic_expansion() {
    let contents = fs::read_to_string("./src/solutions/day9/input.txt")
        .expect("Should have been able to read the file");

    let contents = String::from("...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....");

    println!("{}", part1(&contents));
}

fn part1(input: &String) -> usize{


    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    for (i, line) in input.lines().enumerate() {
        for (j, char) in line.chars().enumerate() {
            if char == '#' {
                galaxies.push((i, j));
            }
        }
    }
    println!("{:?}", galaxies);

    for start in &galaxies {
       for end in &galaxies {
            if end == start {
                continue;
            }
            println!("{:?}{:?}{:?}", start, end, start.0.abs_diff(end.0) + start.1.abs_diff(end.1));
       } 
    }


    0
}

fn is_row_empty(galaxies: &Vec<(usize, usize)>) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let input = String::from(
            "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....");
        assert_eq!(part1(&input), 374);
    }


}