use std::fs::File;
use std::io::{self, BufRead};

pub fn part_one() -> usize {
    let file = File::open("input/day10.txt").expect("some error when reading file");
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .collect();
    let mut count = 0;
    for line in lines {
        let mut delimiter_stack: Vec<char> = Vec::new();
        for character in line.chars() {
            if ['(', '[', '{', '<'].contains(&character) {
                delimiter_stack.push(character);
            } else if character == ')' {
                let last_delimiter = delimiter_stack.pop().unwrap();
                if last_delimiter != '(' {
                    count += 3;
                    break;
                }
            } else if character == ']' {
                let last_delimiter = delimiter_stack.pop().unwrap();
                if last_delimiter != '[' {
                    count += 57;
                    break;
                }
            } else if character == '}' {
                let last_delimiter = delimiter_stack.pop().unwrap();
                if last_delimiter != '{' {
                    count += 1197;
                    break;
                }
            } else if character == '>' {
                let last_delimiter = delimiter_stack.pop().unwrap();
                if last_delimiter != '<' {
                    count += 25137;
                    break;
                }
            }
        }
    }
    count
}

pub fn part_two() -> usize {
    let file = File::open("input/day10.txt").expect("some error when reading file");
    let mut lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .collect();
    let mut scores: Vec<usize> = Vec::new();
    lines.retain(|line| {
        let mut delimiter_stack: Vec<char> = Vec::new();
        let mut should_retain = true;
        for character in line.chars() {
            if ['(', '[', '{', '<'].contains(&character) {
                delimiter_stack.push(character);
            } else if character == ')' {
                let last_delimiter = delimiter_stack.pop().unwrap();
                if last_delimiter != '(' {
                    should_retain = false;
                    break;
                }
            } else if character == ']' {
                let last_delimiter = delimiter_stack.pop().unwrap();
                if last_delimiter != '[' {
                    should_retain = false;
                    break;
                }
            } else if character == '}' {
                let last_delimiter = delimiter_stack.pop().unwrap();
                if last_delimiter != '{' {
                    should_retain = false;
                    break;
                }
            } else if character == '>' {
                let last_delimiter = delimiter_stack.pop().unwrap();
                if last_delimiter != '<' {
                    should_retain = false;
                    break;
                }
            }
        }
        should_retain
    });
    for line in lines {
        let mut delimiter_stack: Vec<char> = Vec::new();
        let mut count = 0;
        for character in line.chars() {
            if ['(', '[', '{', '<'].contains(&character) {
                delimiter_stack.push(character);
            } else {
                delimiter_stack.pop().unwrap();
            }
        }
        while delimiter_stack.len() > 0 {
            let delimiter = delimiter_stack.pop().unwrap();
            let delimiter_score: usize;
            match delimiter {
                '(' => delimiter_score = 1,
                '[' => delimiter_score = 2,
                '{' => delimiter_score = 3,
                _ => delimiter_score = 4,
            }
            count = count * 5 + delimiter_score;
        }
        scores.push(count);
    }
    scores.sort();
    scores[scores.len() / 2]
}
