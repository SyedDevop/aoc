use std::{dbg, todo};

use nom::character::complete::{alpha1, newline};
use nom::multi::separated_list1;
use nom::*;

fn parse_grid(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    separated_list1(
        newline,
        alpha1.map(|letters: &str| letters.chars().collect()),
    )(input)
}

pub fn process_part1(input: &str) -> String {
    let (_, grid) = parse_grid(input).unwrap();
    dbg!(grid);
    "".to_string()
}

pub fn process_part2(input: &str) -> String {
    input.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn part1_works() {
        assert_eq!(process_part1(INPUT), "31");
    }

    #[test]
    #[ignore = "reason"]
    fn part2_works() {
        assert_eq!(process_part2(INPUT), "");
    }
}
