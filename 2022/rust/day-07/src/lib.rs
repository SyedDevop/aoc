pub fn process_part1(_input: &str) -> String {
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
    fn part1_5_works() {
        assert_eq!(process_part1(INPUT), "95437");
    }

    #[test]
    fn part2_works() {
        assert_eq!(process_part2(INPUT), "19");
    }
}
