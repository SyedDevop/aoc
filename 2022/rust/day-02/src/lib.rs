use std::println;

fn get_move_score(letter: &char) -> u128 {
    if 'A' == *letter || 'X' == *letter {
        1
    } else if 'B' == *letter || 'Y' == *letter {
        2
    } else {
        3
    }
}

pub fn process_part1(input: &str) -> String {
    let resualt = input.lines().map(|l| {
        let opp = l.chars().next().unwrap();
        let me = l.chars().nth(2).unwrap();
        let me_num = get_move_score(&me);
        let opp_num = get_move_score(&opp);
        if (me_num + 1) % 3 == opp_num {
            me_num
        } else if me_num == opp_num {
            3 + me_num
        } else {
            6 + me_num
        }
    });

    let mut my_sum: u128 = 0;
    for n in resualt {
        my_sum += n
    }
    my_sum.to_string()
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
A Z
C Z";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "18");
    }

    // #[test]
    // fn part2_works() {
    //     let result = process_part2(INPUT);
    //     assert_eq!(result, "45000");
    // }
}
