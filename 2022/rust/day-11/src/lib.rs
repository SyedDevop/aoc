use std::{collections::VecDeque, dbg, todo};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, multispace1},
    multi::separated_list1,
    sequence::{delimited, preceded},
    *,
};

#[derive(Debug)]
enum Value {
    Old,
    Num(u64),
}
#[derive(Debug)]
enum Operation {
    Mul((Value, Value)),
    Add((Value, Value)),
}

#[derive(Debug)]
struct Test {
    divisible: u64,
    true_recp: u64,
    false_recp: u64,
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u64>,
    opration: Operation,
    test: Test,
    inspect_count: u64,
}

impl Monkey {
    fn inspect(&mut self, is_releafe: bool, magic_num: u64) -> u64 {
        self.inspect_count += 1;
        let old_item = self.items.pop_front().unwrap();
        let mut new_item: u64 = match &self.opration {
            Operation::Mul(val) => match val {
                (Value::Old, Value::Old) => old_item * old_item,
                (Value::Old, Value::Num(num)) => old_item * num,
                _ => panic!("No operation  to perform"),
            },
            Operation::Add(val) => match val {
                (Value::Old, Value::Num(num)) => old_item + num,
                _ => panic!("No operation  to perform"),
            },
        };
        new_item %= magic_num;
        if is_releafe {
            new_item /= 3;
        };
        new_item
    }
    fn test(&self, num: u64) -> usize {
        if num % self.test.divisible == 0 {
            self.test.true_recp as usize
        } else {
            self.test.false_recp as usize
        }
    }
}

fn test(input: &str) -> IResult<&str, Test> {
    let (input, divisible) = preceded(tag("Test: divisible by "), complete::u64)(input)?;
    let (input, _) = multispace1(input)?;
    let (input, true_recp) = preceded(tag("If true: throw to monkey "), complete::u64)(input)?;
    let (input, _) = multispace1(input)?;
    let (input, false_recp) = preceded(tag("If false: throw to monkey "), complete::u64)(input)?;
    Ok((
        input,
        Test {
            divisible,
            true_recp,
            false_recp,
        },
    ))
}

fn value(input: &str) -> IResult<&str, Value> {
    alt((
        tag("old").map(|_| Value::Old),
        complete::u64.map(Value::Num),
    ))(input)
}
fn opration(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("Operation: new = ")(input)?;
    let (input, value_1) = value(input)?;
    let (input, oprater) = delimited(multispace1, alt((tag("*"), tag("+"))), multispace1)(input)?;
    let (input, value_2) = value(input)?;
    let oper = match oprater {
        "*" => Operation::Mul((value_1, value_2)),
        "+" => Operation::Add((value_1, value_2)),
        _ => panic!("unknown Optration"),
    };
    Ok((input, oper))
}

fn parse_monkeys(input: &str) -> IResult<&str, Monkey> {
    let (input, _id) = delimited(tag("Monkey "), complete::u64, tag(":"))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, items) = preceded(
        tag("Starting items: "),
        separated_list1(tag(", "), complete::u64),
    )(input)?;
    let (input, _) = multispace1(input)?;
    let (input, opration) = opration(input)?;
    let (input, _) = multispace1(input)?;
    let (input, test) = test(input)?;
    Ok((
        input,
        Monkey {
            items: VecDeque::from(items),
            opration,
            test,
            inspect_count: 0,
        },
    ))
}

pub fn process_part1(input: &str) -> String {
    let (_, mut monkeys) = separated_list1(tag("\n\n"), parse_monkeys)(input).unwrap();

    let magic_trick = monkeys
        .iter()
        .map(|monkey| monkey.test.divisible)
        .product::<u64>();
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            for _ in 0..monkeys[i].items.len() {
                let new_item = monkeys[i].inspect(true, magic_trick);
                let pass_index = monkeys[i].test(new_item);
                monkeys[pass_index].items.push_back(new_item);
            }
        }
    }

    monkeys.sort_by_key(|mon| mon.inspect_count);
    monkeys
        .iter()
        .rev()
        .take(2)
        .map(|i| i.inspect_count)
        .product::<u64>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, mut monkeys) = separated_list1(tag("\n\n"), parse_monkeys)(input).unwrap();

    let magic_trick = monkeys
        .iter()
        .map(|monkey| monkey.test.divisible)
        .product::<u64>();
    for _ in 0..10_000 {
        for i in 0..monkeys.len() {
            for _ in 0..monkeys[i].items.len() {
                let new_item = monkeys[i].inspect(false, magic_trick);
                let pass_index = monkeys[i].test(new_item);
                monkeys[pass_index].items.push_back(new_item);
            }
        }
    }

    monkeys.sort_by_key(|mon| mon.inspect_count);
    dbg!(&monkeys.iter().map(|i| i.inspect_count).collect::<Vec<_>>());
    monkeys
        .iter()
        .rev()
        .take(2)
        .map(|i| i.inspect_count)
        .product::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn part1_works() {
        assert_eq!(process_part1(INPUT), "10605");
    }

    #[test]
    fn part2_works() {
        assert_eq!(process_part2(INPUT), "2713310158");
    }
}
