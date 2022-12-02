use crate::shared::files;

pub fn solve() {
    part_one();
    part_two();
}

fn part_one() {
    if let Ok(lines) = files::shared::read_lines("./../input/day2.txt") {
        let mut score = 0;
        for line in lines {
            if let Ok(round) = line {
                let splitted: Vec<&str> = round.split_whitespace().collect();
                match splitted[..] {
                    ["A", "X"] => score += 3 + 1,
                    ["A", "Y"] => score += 6 + 2,
                    ["A", "Z"] => score += 0 + 3,
                    ["B", "X"] => score += 0 + 1,
                    ["B", "Y"] => score += 3 + 2,
                    ["B", "Z"] => score += 6 + 3,
                    ["C", "X"] => score += 6 + 1,
                    ["C", "Y"] => score += 0 + 2,
                    ["C", "Z"] => score += 3 + 3,
                    _ => unreachable!(),
                }
            }
        }
        println!("PART ONE: {} ", score);
    }
}

/*
"Anyway, the second column says how the round needs to end: X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win. Good luck!"
*/
fn part_two() {
    if let Ok(lines) = files::shared::read_lines("./../input/day2.txt") {
        let mut score = 0;
        for line in lines {
            if let Ok(round) = line {
                let splitted: Vec<&str> = round.split_whitespace().collect();
                match splitted[..] {
                    ["A", "X"] => score += 0 + 3,
                    ["A", "Y"] => score += 3 + 1,
                    ["A", "Z"] => score += 6 + 2,
                    ["B", "X"] => score += 0 + 1,
                    ["B", "Y"] => score += 3 + 2,
                    ["B", "Z"] => score += 6 + 3,
                    ["C", "X"] => score += 0 + 2,
                    ["C", "Y"] => score += 3 + 3,
                    ["C", "Z"] => score += 6 + 1,
                    _ => unreachable!(),
                }
            }
        }
        println!("PART TWO: {} ", score);
    }
}
