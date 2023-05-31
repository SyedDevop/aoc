use std::{dbg, print};

pub fn process_part1(input: &str) -> String {
    let matrix: Vec<Vec<u32>> = input
        .lines()
        .map(|i| i.chars().map(|a| a.to_digit(10).unwrap()).collect())
        .collect();
    let (y, x) = (matrix.len(), matrix[0].len());
    for ya in 1..y {
        for xa in 1..x {}
    }
    "".to_string()
}

pub fn process_part2(input: &str) -> String {
    let _matrix = input;
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "30373
25512
65332
33549
35390";
    #[test]
    fn part1_works() {
        assert_eq!(process_part1(INPUT), "21");
    }

    #[test]
    #[ignore = "Not Implemented"]
    fn part2_works() {
        assert_eq!(process_part2(INPUT), "24933642");
    }
}
