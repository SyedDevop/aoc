fn code_for_char(cr: char) -> u32 {
    let base = if cr.is_lowercase() {
        'a' as u32 - 1
    } else {
        'A' as u32 - 27
    };
    (cr as u32) - base
}

pub fn process_part1(input: &str) -> String {
    let resualt = input
        .lines()
        .filter_map(|line| line.parse::<String>().ok())
        .fold(0, |acc, line| {
            let srt_len = line.len();
            let (part_one, part_two) = line.split_at(srt_len / 2);
            let a = part_one.chars().find(|c| part_two.contains(*c)).unwrap();
            acc + code_for_char(a)
        });

    resualt.to_string()
}

pub fn process_part2(input: &str) -> String {
    let resualt: u32 = input
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|para| {
            para[0]
                .chars()
                .find(|b| para[1].contains(*b) && para[2].contains(*b))
                .unwrap()
        })
        .map(code_for_char)
        .sum();
    resualt.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "157");
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "12");
    }
}
