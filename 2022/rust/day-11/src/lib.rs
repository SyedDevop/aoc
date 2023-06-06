use std::{
    collections::{BTreeMap, BTreeSet},
    dbg, todo,
};

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
    Num(u32),
}
#[derive(Debug)]
enum Operation {
    Mul((Value, Value)),
    Add((Value, Value)),
}

#[derive(Debug)]
struct Test {
    divisible: u32,
    true_recp: u32,
    false_recp: u32,
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u32>,
    opration: Operation,
    test: Test,
}
fn test(input: &str) -> IResult<&str, Test> {
    let (input, divisible) = preceded(tag("Test: divisible by "), complete::u32)(input)?;
    let (input, _) = multispace1(input)?;
    let (input, true_recp) = preceded(tag("If true: throw to monkey "), complete::u32)(input)?;
    let (input, _) = multispace1(input)?;
    let (input, false_recp) = preceded(tag("If false: throw to monkey "), complete::u32)(input)?;
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
        complete::u32.map(Value::Num),
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
    let (input, _id) = delimited(tag("Monkey "), complete::u32, tag(":"))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, items) = preceded(
        tag("Starting items: "),
        separated_list1(tag(", "), complete::u32),
    )(input)?;
    let (input, _) = multispace1(input)?;
    let (input, opration) = opration(input)?;
    let (input, _) = multispace1(input)?;
    let (input, test) = test(input)?;
    Ok((
        input,
        Monkey {
            items,
            opration,
            test,
        },
    ))
}

pub fn process_part1(input: &str) -> String {
    let (_, monkeys) = separated_list1(tag("\n\n"), parse_monkeys)(input).unwrap();
    let mon: BTreeSet<Vec<Monkey>> = BTreeSet::from(monkeys);
    let mon_map = BTreeMap::from(monkeys);
    dbg!(mon);
    dbg!(mon_map);
    "".to_string()
}

pub fn process_part2(input: &str) -> String {
    input.to_string()
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
    #[ignore = "reason"]
    fn part2_works() {
        assert_eq!(process_part2(INPUT), "");
    }
}
