use std::dbg;

use nom::branch::alt;
use nom::bytes::complete::{is_a, tag};
use nom::character::complete::{alpha1, newline};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::*;

#[derive(Debug)]
enum Cmds<'a> {
    Cd(Cd<'a>),
    Ls(Vec<Files<'a>>),
}

#[derive(Debug)]
enum Cd<'a> {
    Root,
    Up,
    Down(&'a str),
}

#[derive(Debug)]
enum Files<'a> {
    File { size: u32, name: &'a str },
    Dir(&'a str),
}

fn file(input: &str) -> IResult<&str, Files> {
    let (input, (size, name)) = separated_pair(
        nom::character::complete::u32,
        tag(" "),
        is_a("qwertyuiopasdfghjklzxcvbnm."),
    )(input)?;
    Ok((input, Files::File { size, name }))
}

fn directory(input: &str) -> IResult<&str, Files> {
    let (input, _) = tag("dir ")(input)?;
    let (input, name) = alpha1(input)?;
    Ok((input, Files::Dir(name)))
}

fn ls(input: &str) -> IResult<&str, Cmds> {
    let (input, _) = tag("$ ls")(input)?;
    let (input, _) = newline(input)?;
    let (input, files) = separated_list1(newline, alt((file, directory)))(input)?;
    Ok((input, Cmds::Ls(files)))
}
fn cd(input: &str) -> IResult<&str, Cmds> {
    let (input, _) = tag("$ cd ")(input)?;
    let (input, dir) = alt((tag(".."), alpha1, tag("/")))(input)?;
    let op = match dir {
        "/" => Cmds::Cd(Cd::Root),
        ".." => Cmds::Cd(Cd::Up),
        name => Cmds::Cd(Cd::Down(name)),
    };
    Ok((input, op))
}

fn cmd_parser(input: &str) -> IResult<&str, Vec<Cmds>> {
    let (input, cmd) = separated_list1(newline, alt((ls, cd)))(input)?;

    Ok((input, cmd))
}

pub fn process_part1(input: &str) -> String {
    let cmds = cmd_parser(input).unwrap();
    dbg!(cmds);
    "".to_string()
}

pub fn process_part2(_input: &str) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
    #[test]
    fn part1_works() {
        assert_eq!(process_part1(INPUT), "94853");
    }
    #[test]
    #[ignore = "not impleted"]
    fn part1_5_works() {
        assert_eq!(process_part1(INPUT), "95437");
    }

    #[test]
    #[ignore = "not impleted"]
    fn part2_works() {
        assert_eq!(process_part2(INPUT), "19");
    }
}
