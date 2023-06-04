// use std::{dbg, print};

fn is_visible_in_row(idx: usize, matrix: &Vec<u32>) -> (bool, (u32, u32)) {
    let tree = matrix[idx];
    let mut result: Vec<bool> = vec![];
    let mut result_left: Vec<bool> = vec![];
    let mut right_num = 0;

    let mut left_num = 0;
    for right in (idx + 1)..matrix.len() {
        right_num += 1;
        if tree > matrix[right] {
            result.push(true);
        } else {
            result.push(false);
            break;
        }
    }
    for left in (0..idx).rev() {
        left_num += 1;
        if tree > matrix[left] {
            result_left.push(true);
        } else {
            result_left.push(false);
            break;
        }
    }
    let is_visible = result.iter().all(|&x| x) || result_left.iter().all(|&x| x);
    (is_visible, (left_num, right_num))
}

fn is_visible_in_column(idx: usize, idy: usize, matrix: &Vec<Vec<u32>>) -> (bool, (u32, u32)) {
    let mut new_row: Vec<u32> = vec![];
    for row in matrix {
        new_row.push(row[idx]);
    }

    is_visible_in_row(idy, &new_row)
}

pub fn process_part1(input: &str) -> String {
    let matrix: Vec<Vec<u32>> = input
        .lines()
        .map(|i| i.chars().map(|a| a.to_digit(10).unwrap()).collect())
        .collect();
    let (y, x) = (matrix.len() - 1, matrix[0].len() - 1);
    let mut a = 0;
    for ya in 1..y {
        for xa in 1..x {
            if is_visible_in_row(xa, &matrix[ya]).0 || is_visible_in_column(xa, ya, &matrix).0 {
                a += 1;
            }
        }
    }
    let total = (x * 2 + 2) + (y * 2 - 2) + a;
    total.to_string()
}

pub fn process_part2(input: &str) -> String {
    let matrix: Vec<Vec<u32>> = input
        .lines()
        .map(|i| i.chars().map(|a| a.to_digit(10).unwrap()).collect())
        .collect();
    let (y, x) = (matrix.len() - 1, matrix[0].len());

    let mut scenic_scores = vec![];
    for ya in 1..y {
        for xa in 1..x {
            let row = is_visible_in_row(xa, &matrix[ya]).1;
            let col = is_visible_in_column(xa, ya, &matrix).1;
            scenic_scores.push((row.0 * row.1) * (col.0 * col.1));
        }
    }
    scenic_scores.iter().max().unwrap().to_string()
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
    fn part2_works() {
        assert_eq!(process_part2(INPUT), "8");
    }
}
