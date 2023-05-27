use std::collections::HashMap;
use std::println;

pub fn process_part1(input: &str) -> String {
    let resualt = input.lines().collect::<Vec<_>>();
    let mut creat_list: HashMap<u16, Vec<char>> = HashMap::new();

    println!("{:#?}", &resualt[0..8]);
    for (i, line) in resualt[0..8].iter().enumerate() {
        let mut a: Vec<char> = vec![];
        for x in (1..line.len()).step_by(4) {
            let v: char = line.chars().collect::<Vec<char>>()[x];
            a.push(v);
        }
        creat_list.insert(i as u16 + 1, a);
    }
    println!("{:?}", creat_list);
    "".to_string()
}

pub fn process_part2(input: &str) -> String {
    let resualt = input;

    resualt.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

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
