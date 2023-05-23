use std::str::FromStr;

// use std::println;
//
enum Move {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}
impl FromStr for Move {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissor),
            _ => Err("Not a know move".to_string()),
        }
    }
}

//
// fn get_move_score(letter: &char) -> u8 {
//     if 'A' == *letter || 'X' == *letter {
//         1
//     } else if 'B' == *letter || 'Y' == *letter {
//         2
//     } else {
//         3
//     }
// }

pub fn process_part1(input: &str) -> String {
    let resualt = input.lines().map(|l| {
        let moves: Vec<Move> = l
            .split(' ')
            .map(|s| s.parse::<Move>().unwrap())
            .collect::<Vec<_>>();
        moves
    });
    println!("{:#?}", resualt);
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
