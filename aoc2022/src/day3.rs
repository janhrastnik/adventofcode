use crate::shared::files;

pub fn solve() {
    part_one();
    part_two();
}

/*
EVALUATING THE CHARACTERS
'a'.to_digit(36) -> Some(10)
('a'.to_digit(36).unwrap() - 9) -> 1
('A'.to_digit(36).unwrap() - 9) + 'A'.is_uppercase() ? 26 : 0 -> 27
*/

fn part_one() {
    if let Ok(lines) = files::shared::read_lines("./input/day3.txt") {
        let mut score = 0;
        for line in lines {
            if let Ok(rucksack_line) = line {
                let rucksack = rucksack_line.chars().collect::<Vec<char>>().clone();
                let size = rucksack.len();
                let duplicate_item = rucksack
                    .iter()
                    .filter(|item| {
                        rucksack[0..(size / 2)].contains(item)
                            && rucksack[(size / 2)..size].contains(item)
                    })
                    .next()
                    .unwrap();
                score += duplicate_item.to_digit(36).unwrap() - 9;
                if duplicate_item.is_uppercase() {
                    score += 26;
                }

                //for item in rucksack.chars() {}
            }
        }
        println!("PART ONE: {:?}", score);
    }
}

fn part_two() {
    if let Ok(lines) = files::shared::read_lines("./input/day3.txt") {
        let mut score = 0;
        let mut long_boi: Vec<char> = Vec::new();
        let mut lengths: Vec<usize> = Vec::new();
        for line in lines {
            if let Ok(rucksack_line) = line {
                long_boi.extend(rucksack_line.chars());
                lengths.push(*lengths.last().get_or_insert(&0) + rucksack_line.len());
                if lengths.len() == 3 {
                    let badge = long_boi
                        .iter()
                        .filter(|item| {
                            long_boi[0..lengths[0]].contains(item)
                                && long_boi[lengths[0]..lengths[1]].contains(item)
                                && long_boi[lengths[1]..lengths[2]].contains(item)
                        })
                        .next()
                        .unwrap();
                    score += badge.to_digit(36).unwrap() - 9;
                    if badge.is_uppercase() {
                        score += 26;
                    }
                    long_boi = Vec::new();
                    lengths = Vec::new();
                }
            }
        }
        println!("PART TWO: {:?}", score);
    }
}
