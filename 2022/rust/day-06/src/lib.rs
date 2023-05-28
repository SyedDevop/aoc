pub fn process_part1(input: &str) -> String {
    let codes = input.chars();

    let mut unique = String::new();
    for (i, ch) in codes.enumerate() {
        if !unique.contains(ch) {
            unique.push(ch);
        } else {
            let dup_index = unique.find(ch).unwrap(); // existing char in unique.
            unique.drain(0..=dup_index);
            unique.push(ch);
        }
        if unique.len() == 4 {
            dbg!(&unique);
            unique.clear();
            let c = (i as u32 + 1).to_string();
            unique += &c;
            break;
        }
    }
    unique
}

pub fn process_part2(input: &str) -> String {
    let codes = input.chars();

    let mut unique = String::new();
    for (i, ch) in codes.enumerate() {
        if !unique.contains(ch) {
            unique.push(ch);
        } else {
            let dup_index = unique.find(ch).unwrap(); // existing char in unique.
            unique.drain(0..=dup_index);
            unique.push(ch);
        }
        if unique.len() == 14 {
            dbg!(&unique);
            unique.clear();
            let c = (i as u32 + 1).to_string();
            unique += &c;
            break;
        }
    }
    unique
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        assert_eq!(process_part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), "7");
        assert_eq!(process_part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), "5");
        assert_eq!(process_part1("nppdvjthqldpwncqszvftbrmjlhg"), "6");
        assert_eq!(process_part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), "10");
        assert_eq!(process_part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), "11");
    }

    #[test]
    fn part2_works() {
        assert_eq!(process_part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), "19");
        assert_eq!(process_part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), "23");
        assert_eq!(process_part2("nppdvjthqldpwncqszvftbrmjlhg"), "23");
        assert_eq!(process_part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), "29");
        assert_eq!(process_part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), "26");
    }
}
