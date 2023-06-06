use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::preceded,
    *,
};

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i64),
}
struct Process {
    x: i64,
    clock: i64,
}

impl Process {
    pub fn mult(&self) -> i64 {
        dbg!(&self.x, &self.clock);
        self.x * self.clock
    }
}

use Instruction::*;

fn parser_execution(input: &str) -> IResult<&str, Vec<Instruction>> {
    let (input, ins) = separated_list1(
        newline,
        alt((
            tag("noop").map(|_| Noop),
            preceded(tag("addx "), complete::i64).map(Addx),
        )),
    )(input)?;
    Ok((input, ins))
}

pub fn process_part1(input: &str) -> String {
    let steps = [20, 60, 100, 140, 180, 220];
    let mut pros = Process { x: 1, clock: 0 };
    let (_, excu) = parser_execution(input).unwrap();

    let mut signal: Vec<i64> = vec![];
    for (_, ins) in excu.iter().enumerate() {
        match ins {
            Noop => {
                pros.clock += 1;
                if steps.contains(&pros.clock) {
                    signal.push(pros.mult())
                }
            }
            Addx(value) => {
                for _ in 0..=1 {
                    pros.clock += 1;
                    if steps.contains(&pros.clock) {
                        signal.push(pros.mult())
                    }
                }
                pros.x += value;
            }
        }
    }
    signal.iter().sum::<i64>().to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut pros = Process { x: 1, clock: 0 };
    let (_, excu) = parser_execution(input).unwrap();
    let mut img: String = "".to_string();

    for ins in excu.iter() {
        let sprite = (pros.x)..(pros.x + 3);
        match ins {
            Noop => {
                pros.clock += 1;
                if sprite.contains(&(pros.clock % 40)) {
                    img.push('#')
                } else {
                    img.push('.')
                }
            }
            Addx(value) => {
                for _ in 0..=1 {
                    pros.clock += 1;
                    if sprite.contains(&(pros.clock % 40)) {
                        img.push('#')
                    } else {
                        img.push('.')
                    };
                }
                pros.x += value;
            }
        }
    }

    img.chars()
        .collect::<Vec<_>>()
        .chunks(40)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn part1_works() {
        assert_eq!(process_part1(INPUT), "13140");
    }

    #[test]
    fn part2_works() {
        assert_eq!(
            process_part2(INPUT),
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
        );
    }
}
