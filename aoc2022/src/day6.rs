use crate::shared::files;

pub fn solve() {
    part_one();
    part_two();
}

fn part_one() {
    if let Ok(mut lines) = files::shared::read_lines("./input/day6.txt") {
        let input = lines.next().unwrap().unwrap();
        let mut marker: usize = 0;
        'outer: for i in 0..input.len() {
            let end_index: usize;
            if i + 4 < input.len() {
                end_index = i + 4;
            } else {
                end_index = input.len();
            }
            let slice = &input[i..end_index];
            for char in slice.chars() {
                if slice.matches(char).count() > 1 {
                    continue 'outer;
                }
            }
            marker = i;
            break;
        }
        println!("PART ONE: {:?}", marker + 4);
    }
}

fn part_two() {
    if let Ok(mut lines) = files::shared::read_lines("./input/day6.txt") {
        let input = lines.next().unwrap().unwrap();
        let mut marker: usize = 0;
        'outer: for i in 0..input.len() {
            let end_index: usize;
            if i + 14 < input.len() {
                end_index = i + 14;
            } else {
                end_index = input.len();
            }
            let slice = &input[i..end_index];
            for char in slice.chars() {
                if slice.matches(char).count() > 1 {
                    continue 'outer;
                }
            }
            marker = i;
            break;
        }
        println!("PART TWO: {:?}", marker + 14);
    }
}
