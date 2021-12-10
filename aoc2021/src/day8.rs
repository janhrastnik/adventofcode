use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

fn concat(vec: Vec<usize>) -> usize {
    vec.iter().fold(0, |acc, elem| acc * 10 + elem)
}

pub fn part_one() -> usize {
    let file = File::open("input/day8-1.txt").expect("some error when reading file");
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .collect();
    let mut count = 0;
    for line in lines {
        // line: [signals, output]
        let line: Vec<&str> = line.split(" | ").collect();
        let line: Vec<&str> = line[1].split(" ").collect();
        for signal in line {
            if [2, 3, 4, 7].contains(&signal.len()) {
                count += 1;
            }
        }
    }
    count
}

pub fn part_two() -> usize {
    let file = File::open("input/day8-1.txt").expect("some error when reading file");
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .collect();
    let mut count = 0;
    for line in lines {
        // line: [signals, output]
        let line: Vec<&str> = line.split(" |").collect();
        let output: Vec<&str> = line[1].split(" ").collect();
        let line_str = format!("{}{}", line[0], line[1]);
        let line: Vec<&str> = line_str.split(" ").collect();
        let mut found_numbers: HashMap<usize, HashSet<char>> = HashMap::from([
            (0, HashSet::new()),
            (1, HashSet::new()),
            (2, HashSet::new()),
            (3, HashSet::new()),
            (4, HashSet::new()),
            (5, HashSet::new()),
            (6, HashSet::new()),
            (7, HashSet::new()),
            (8, HashSet::new()),
            (9, HashSet::new()),
        ]);

        let mut candidates: Vec<HashSet<char>> = Vec::new();

        for signal in line {
            if signal.len() == 2 {
                *found_numbers.get_mut(&1).unwrap() = signal.chars().collect::<HashSet<char>>();
            }
            if signal.len() == 3 {
                *found_numbers.get_mut(&7).unwrap() = signal.chars().collect::<HashSet<char>>();
            }
            if signal.len() == 4 {
                *found_numbers.get_mut(&4).unwrap() = signal.chars().collect::<HashSet<char>>();
            }
            if signal.len() == 6 {
                candidates.push(signal.chars().collect::<HashSet<char>>());
            }
            if signal.len() == 7 {
                *found_numbers.get_mut(&8).unwrap() = signal.chars().collect::<HashSet<char>>();
            }
        }

        // determine a
        let diff = &found_numbers[&7] - &found_numbers[&1];
        let a = *diff.iter().next().unwrap();

        // determine bd
        let bd = &found_numbers[&4] - &found_numbers[&1];

        // determine c, b
        let mut c = '0';
        let mut b = '0';
        for candidate in &mut candidates {
            let diff = &found_numbers[&1] - &candidate;
            if diff.len() == 1 {
                *found_numbers.get_mut(&6).unwrap() = candidate.clone();
                c = *diff.iter().next().unwrap();
            }
            let diff = &found_numbers[&8] - &candidate;
            let bd_diff = &bd - &diff;
            if bd_diff.len() == 1 {
                *found_numbers.get_mut(&0).unwrap() = candidate.clone();
                b = *bd_diff.iter().next().unwrap();
            }
        }

        // determine d
        let d = *(&bd - &HashSet::from([b])).iter().next().unwrap();

        // determine f
        let f = *(&found_numbers[&7] - &HashSet::from([c, a]))
            .iter()
            .next()
            .unwrap();

        // determine e
        let mut e = 'a';
        for candidate in &mut candidates {
            let diff = &found_numbers[&8] - &candidate;
            let diff_char = *diff.iter().next().unwrap();
            if diff_char != d && diff_char != c {
                e = diff_char;
                *found_numbers.get_mut(&9).unwrap() = candidate.clone();
            }
        }

        // determine g
        let g = *(&HashSet::from(['a', 'b', 'c', 'd', 'e', 'f', 'g'])
            - &HashSet::from([a, b, c, d, e, f]))
            .iter()
            .next()
            .unwrap();

        // we now know all seven segments, thus we can define the missing numbers
        // the missing numbers are 2, 3, 5,
        *found_numbers.get_mut(&2).unwrap() = HashSet::from([a, c, d, e, g]);
        *found_numbers.get_mut(&3).unwrap() = HashSet::from([a, c, d, f, g]);
        *found_numbers.get_mut(&5).unwrap() = HashSet::from([a, b, d, f, g]);

        let mut digits: Vec<usize> = Vec::new();
        for output_number in output {
            for i in 0..10 {
                if found_numbers[&i] == output_number.chars().collect::<HashSet<char>>() {
                    digits.push(i);
                }
            }
        }
        let final_output_number = concat(digits);
        count += final_output_number;
    }
    count
}
