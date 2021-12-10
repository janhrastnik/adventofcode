use std::fs::File;
use std::io::{self, BufRead};

pub fn part_one() -> usize {
    let file = File::open("input/day9-1.txt").expect("some error when reading file");
    let lines: Vec<Vec<usize>> = io::BufReader::new(file)
        .lines()
        .map(|x| {
            x.unwrap()
                .chars()
                .map(|c| c as usize - '0' as usize)
                .collect()
        })
        .collect();
    let mut count = 0;
    let mut minimums: Vec<usize> = Vec::new();
    for j in 0..lines.len() {
        for i in 0..lines[0].len() {
            let mut is_minimum = true;
            let candidate = lines.get(j).unwrap().get(i).unwrap();
            let up_check = lines.get(j.saturating_sub(1)).unwrap().get(i);
            let down_check = lines.get(j + 1).and_then(|x| x.get(i));
            let left_check = lines.get(j).unwrap().get(i.saturating_sub(1));
            let right_check = lines.get(j).unwrap().get(i + 1);
            if up_check.is_some() && j != 0 {
                if candidate >= up_check.unwrap() {
                    is_minimum = false;
                }
            }
            if down_check.is_some() {
                if candidate >= down_check.unwrap() {
                    is_minimum = false;
                }
            }
            if left_check.is_some() && i != 0 {
                if candidate >= left_check.unwrap() {
                    is_minimum = false;
                }
            }
            if right_check.is_some() {
                if candidate >= right_check.unwrap() {
                    is_minimum = false;
                }
            }
            if is_minimum {
                minimums.push(*candidate);
                count += 1 + candidate
            }
        }
    }
    count
}

pub fn part_two() -> usize {
    let file = File::open("input/day9-1.txt").expect("some error when reading file");
    let lines: Vec<Vec<usize>> = io::BufReader::new(file)
        .lines()
        .map(|x| {
            x.unwrap()
                .chars()
                .map(|c| c as usize - '0' as usize)
                .collect()
        })
        .collect();
    let mut minimums: Vec<(usize, usize)> = Vec::new(); // the indices for the minimum
    for j in 0..lines.len() {
        for i in 0..lines[0].len() {
            let mut is_minimum = true;
            let candidate = lines.get(j).unwrap().get(i).unwrap();
            let up_check = lines.get(j.saturating_sub(1)).unwrap().get(i);
            let down_check = lines.get(j + 1).and_then(|x| x.get(i));
            let left_check = lines.get(j).unwrap().get(i.saturating_sub(1));
            let right_check = lines.get(j).unwrap().get(i + 1);
            if up_check.is_some() && j != 0 {
                if candidate >= up_check.unwrap() {
                    is_minimum = false;
                }
            }
            if down_check.is_some() {
                if candidate >= down_check.unwrap() {
                    is_minimum = false;
                }
            }
            if left_check.is_some() && i != 0 {
                if candidate >= left_check.unwrap() {
                    is_minimum = false;
                }
            }
            if right_check.is_some() {
                if candidate >= right_check.unwrap() {
                    is_minimum = false;
                }
            }
            if is_minimum {
                minimums.push((j, i));
            }
        }
    }

    let row_count = lines.len();
    let col_count = lines[0].len();

    let mut basins: Vec<usize> = Vec::new();

    // find the basins
    for minimum in minimums {
        let mut candidate_list: Vec<(usize, usize)> = Vec::new();
        let mut checked_elements: Vec<(usize, usize)> = Vec::new();
        let mut basin_count = 0;
        candidate_list.push(minimum);
        while candidate_list.len() > 0 {
            // start of loop, remove an element from candidate list, add it to checked elements
            let candidate = candidate_list.pop().unwrap();
            if lines[candidate.0][candidate.1] != 9 && !checked_elements.contains(&candidate) {
                basin_count += 1;
                // up
                if candidate.0 != 0 {
                    let indices = (candidate.0 - 1, candidate.1);
                    candidate_list.push(indices);
                }
                // down
                if candidate.0 != row_count - 1 {
                    let indices = (candidate.0 + 1, candidate.1);
                    candidate_list.push(indices);
                }
                // left
                if candidate.1 != 0 {
                    let indices = (candidate.0, candidate.1 - 1);
                    candidate_list.push(indices);
                }
                // right
                if candidate.1 != col_count - 1 {
                    let indices = (candidate.0, candidate.1 + 1);
                    candidate_list.push(indices);
                }
            }
            checked_elements.push(candidate);
        }
        basins.push(basin_count);
    }
    basins.sort();
    let one = basins.pop().unwrap();
    let two = basins.pop().unwrap();
    let three = basins.pop().unwrap();
    one * two * three
}
