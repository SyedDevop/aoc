use itertools::Itertools;
use std::collections::HashSet;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::separated_pair,
    *,
};

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

fn direction(input: &str) -> IResult<&str, Direction> {
    let (input, dir) = alt((
        complete::char('L').map(|_| Direction::Left),
        complete::char('R').map(|_| Direction::Right),
        complete::char('U').map(|_| Direction::Up),
        complete::char('D').map(|_| Direction::Down),
    ))(input)?;
    Ok((input, dir))
}

fn parse_moves(input: &str) -> IResult<&str, Vec<Direction>> {
    let (_, moves) =
        separated_list1(newline, separated_pair(direction, tag(" "), complete::u32))(input)?;
    let vecs: Vec<Direction> = moves
        .iter()
        .flat_map(|(dir, steps)| vec![*dir; *steps as usize])
        .collect();
    Ok(("", vecs))
}

pub fn process_part1(input: &str) -> String {
    let (_, moves) = parse_moves(input).unwrap();
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut tail_positions = HashSet::from([tail]);

    for head_move in moves.iter() {
        match head_move {
            Direction::Up => head.1 += 1,
            Direction::Down => head.1 -= 1,
            Direction::Right => head.0 += 1,
            Direction::Left => head.0 -= 1,
        }
        let x_range = (head.0 - 1)..=(head.0 + 1);
        let y_range = (head.1 - 1)..=(head.1 + 1);

        let is_tail_connected = x_range.cartesian_product(y_range).any(|s| s == tail);
        if !is_tail_connected {
            let mut new_tail = Clone::clone(&head);
            match head_move {
                Direction::Up => new_tail.1 -= 1,
                Direction::Down => new_tail.1 += 1,
                Direction::Right => new_tail.0 -= 1,
                Direction::Left => new_tail.0 += 1,
            }
            tail = new_tail;
            tail_positions.insert(new_tail);
        }
    }
    tail_positions.len().to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, moves) = parse_moves(input).unwrap();
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut tail_positions = HashSet::from([tail]);
    let mut tail_head: Vec<(i32, i32)> = vec![];
    for head_move in moves.iter() {
        match head_move {
            Direction::Up => head.1 += 1,
            Direction::Down => head.1 -= 1,
            Direction::Right => head.0 += 1,
            Direction::Left => head.0 -= 1,
        }

        if tail_head.is_empty() {
            let x_range = (head.0 - 1)..=(head.0 + 1);
            let y_range = (head.1 - 1)..=(head.1 + 1);

            let is_tail_connected = x_range.cartesian_product(y_range).any(|s| s == tail);
            if !is_tail_connected {
                let mut new_tail = Clone::clone(&head);
                match head_move {
                    Direction::Up => new_tail.1 -= 1,
                    Direction::Down => new_tail.1 += 1,
                    Direction::Right => new_tail.0 -= 1,
                    Direction::Left => new_tail.0 += 1,
                }

                tail_head.push(new_tail);
            }
        } else {
            for i in 0..tail_head.len() {
                if i == 0 {
                    let x_range = (head.0 - 1)..=(head.0 + 1);
                    let y_range = (head.1 - 1)..=(head.1 + 1);

                    let is_tail_connected = x_range
                        .cartesian_product(y_range)
                        .any(|s| s == tail_head[0]);
                    if !is_tail_connected {
                        let mut new_tail = Clone::clone(&tail_head[0]);
                        match head_move {
                            Direction::Up => new_tail.1 -= 1,
                            Direction::Down => new_tail.1 += 1,
                            Direction::Right => new_tail.0 -= 1,
                            Direction::Left => new_tail.0 += 1,
                        }

                        tail_head.push(new_tail);
                    }
                }

                let x_range = (tail_head[0].0 - 1)..=(head.0 + 1);
                let y_range = (head.1 - 1)..=(head.1 + 1);
            }
        }

        let x_range = (head.0 - 1)..=(head.0 + 1);
        let y_range = (head.1 - 1)..=(head.1 + 1);

        let is_tail_connected = x_range.cartesian_product(y_range).any(|s| s == tail);
        if !is_tail_connected {
            let mut new_tail = Clone::clone(&head);
            match head_move {
                Direction::Up => new_tail.1 -= 1,
                Direction::Down => new_tail.1 += 1,
                Direction::Right => new_tail.0 -= 1,
                Direction::Left => new_tail.0 += 1,
            }
            tail = new_tail;
            tail_positions.insert(new_tail);
        }
    }
    tail_positions.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
    #[test]
    fn part1_works() {
        assert_eq!(process_part1(INPUT), "13");
    }

    #[test]
    fn part2_works() {
        assert_eq!(process_part2(INPUT), "1");
        assert_eq!(
            process_part2(
                "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"
            ),
            "36"
        );
    }
}
