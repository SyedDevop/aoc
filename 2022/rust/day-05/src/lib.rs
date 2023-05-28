use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
struct MoveCreate {
    count: usize,
    from: usize,
    to: usize,
}

impl FromStr for MoveCreate {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut proceeder_list = s.split(' ');
        Ok(Self {
            count: proceeder_list.nth(1).unwrap().parse::<usize>().unwrap(),
            from: proceeder_list.nth(1).unwrap().parse::<usize>().unwrap(),
            to: proceeder_list.nth(1).unwrap().parse::<usize>().unwrap(),
        })
    }
}

pub fn process_part1(input: &str) -> String {
    let lines = input.lines().collect::<Vec<_>>();
    let mut creat_list: HashMap<usize, Vec<char>> = HashMap::new();

    for (i, x) in (1..lines[0].len()).step_by(4).enumerate() {
        for line in lines.iter().take(8) {
            if let Some(ch) = line.chars().nth(x) {
                if ch != ' ' {
                    creat_list.entry(i + 1).or_insert_with(Vec::new).push(ch);
                }
            }
        }
        creat_list.get_mut(&(i + 1)).unwrap().reverse();
    }

    let move_create = lines[10..]
        .iter()
        .filter_map(|a| a.parse::<MoveCreate>().ok());

    for m in move_create {
        (0..m.count).for_each(|_| {
            let lat = creat_list.get_mut(&m.from).unwrap().pop().unwrap();
            creat_list.get_mut(&m.to).unwrap().push(lat)
        })
    }
    (1..=9)
        .map(|c| creat_list.get(&c).unwrap().last().unwrap())
        .collect::<String>()
}

pub fn process_part2(input: &str) -> String {
    let lines = input.lines().collect::<Vec<_>>();
    let mut creat_list: HashMap<usize, Vec<char>> = HashMap::new();

    for (i, x) in (1..lines[0].len()).step_by(4).enumerate() {
        for line in lines.iter().take(8) {
            if let Some(ch) = line.chars().nth(x) {
                if ch != ' ' {
                    creat_list.entry(i + 1).or_insert_with(Vec::new).push(ch);
                }
            }
        }
        creat_list.get_mut(&(i + 1)).unwrap().reverse();
    }

    let move_create = lines[10..]
        .iter()
        .filter_map(|a| a.parse::<MoveCreate>().ok());

    for m in move_create {
        let from_list = creat_list.get_mut(&m.from).unwrap();

        let mut lat = from_list
            .drain((from_list.len() - m.count)..)
            .collect::<Vec<_>>();
        creat_list.get_mut(&m.to).unwrap().append(&mut lat)
    }
    (1..=9)
        .map(|c| creat_list.get(&c).unwrap().last().unwrap())
        .collect::<String>()
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
