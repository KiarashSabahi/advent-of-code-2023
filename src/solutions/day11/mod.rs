use std::fs;

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



    0
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