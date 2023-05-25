fn get_move_score(letter: char) -> u64 {
    match letter {
        'A' | 'X' => 1,
        'B' | 'Y' => 2,
        _ => 3,
    }
}

pub fn process_part1(input: &str) -> String {
    let resualt: u64 = input
        .lines()
        .map(|l| {
            let opp = l.chars().next().unwrap();
            let me = l.chars().nth(2).unwrap();
            let me_num = get_move_score(me);
            let opp_num = get_move_score(opp);
            if (me_num % 3) + 1 == opp_num {
                me_num
            } else if me_num == opp_num {
                3 + me_num
            } else {
                6 + me_num
            }
        })
        .sum();

    resualt.to_string()
}

pub fn process_part2(input: &str) -> String {
    let resualt: u64 = input
        .lines()
        .map(|l| {
            let opp = l.chars().next().unwrap();
            let me = l.chars().nth(2).unwrap();
            let opp_num = get_move_score(opp);
            match me {
                'X' => {
                    if opp_num == 2 {
                        1
                    } else {
                        opp_num % 3 + 2
                    }
                }
                'Y' => 3 + opp_num,
                'Z' => (opp_num % 3) + 7,
                _ => 0,
            }
        })
        .sum();

    resualt.to_string()
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

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "12");
    }
}
