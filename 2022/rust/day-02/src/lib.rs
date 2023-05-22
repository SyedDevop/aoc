pub fn process_part1(input: &str) -> String {
    let resualt = input
        .lines()
        .filter_map(|line| line.parse::<String>().ok())
        .collect::<Vec<_>>();

    "".to_string()
}

pub fn process_part2(input: &str) -> String {
    let resualt = input
        .lines()
        .filter_map(|line| line.parse::<String>().ok())
        .collect::<Vec<_>>();
    "".to_string()
}
//Note..
//me : X(roc), Y(paper), Z(scissor)
//        1        2         3
//other: A(roc), B(paper), C(scissor)
//sore : win-6 lost-0 draw-3

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "15");
    }

    // #[test]
    // fn part2_works() {
    //     let result = process_part2(INPUT);
    //     assert_eq!(result, "45000");
    // }
}
