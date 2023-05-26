use std::ops::RangeInclusive;
use std::str::FromStr;

struct NumberRange {
    left: RangeInclusive<u16>,
    right: RangeInclusive<u16>,
}

impl FromStr for NumberRange {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left_pair, right_pair) = s
            .split_once(',')
            .ok_or_else(|| "Invalid input format".to_string())?;
        let (left_start, left_end) = left_pair.split_once('-').unwrap();
        let (right_start, right_end) = right_pair.split_once('-').unwrap();

        let left = left_start.parse::<u16>().unwrap()..=left_end.parse::<u16>().unwrap();
        let right = right_start.parse::<u16>().unwrap()..=right_end.parse::<u16>().unwrap();
        Ok(Self { left, right })
    }
}
pub fn process_part1(input: &str) -> String {
    let resualt = input
        .lines()
        .map(|line| line.parse::<NumberRange>().unwrap());
    let mut in_range = 0;
    for num in resualt {
        if num.right.contains(num.left.start()) && num.right.contains(num.left.end())
            || num.left.contains(num.right.start()) && num.left.contains(num.right.end())
        {
            in_range += 1
        }
    }
    in_range.to_string()
}

pub fn process_part2(input: &str) -> String {
    let resualt = input
        .lines()
        .map(|line| line.parse::<NumberRange>().unwrap())
        .filter(|num| {
            num.right.contains(num.left.start())
                || num.right.contains(num.left.end())
                || num.left.contains(num.right.start())
                || num.left.contains(num.right.end())
        })
        .count();

    resualt.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "2");
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "4");
    }
}
