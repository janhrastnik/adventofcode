use std::fs::File;
use std::io::{self, BufRead};

pub fn part_one() -> usize {
    let file = File::open("input/day1-1.txt").expect("some error when reading file");
    let lines = io::BufReader::new(file).lines();
    let mut count = 0;
    let mut prev_number: Option<usize> = None;

    for line in lines {
        if let Ok(number) = line.unwrap().parse::<usize>() {
            if prev_number.is_none() {
                prev_number = Some(number);
                continue;
            }
            if prev_number.unwrap() < number {
                count += 1;
            }
            prev_number = Some(number)
        }
    }
    count
}

pub fn part_two() -> usize {
    let file = File::open("input/day1-1.txt").expect("some error when reading file");
    let lines = io::BufReader::new(file).lines();
    let mut count = 0;
    let mut window = Vec::new();
    let mut prev_window_number: Option<usize> = None;

    for line in lines {
        if let Ok(number) = line.unwrap().parse::<usize>() {
            if window.len() < 3 {
                window.push(number);
                continue;
            }
            if prev_window_number.is_none() {
                prev_window_number = Some(window.iter().sum());
            }
            window.remove(0);
            window.push(number);
            if prev_window_number.unwrap() < window.iter().sum() {
                count += 1;
            }
            prev_window_number = Some(window.iter().sum());
        }
    }

    count
}
