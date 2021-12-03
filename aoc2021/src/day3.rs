use std::convert::TryInto;
use std::fs::File;
use std::io::{self, BufRead};

pub fn part_one() -> usize {
    let file = File::open("input/day3-1.txt").expect("some error when reading file");
    let lines = io::BufReader::new(file).lines();
    let mut one_count: [usize; 12] = [0; 12]; // hardcoded, not ideal
    let mut line_count: usize = 0;

    for line in lines {
        line_count += 1;
        if let Ok(digits) = line {
            for (i, character) in digits.chars().enumerate() {
                if character == '1' {
                    one_count[i] = one_count[i] + 1
                }
            }
        }
    }

    let mut gamma: usize = 0;
    let mut epsilon: usize = 0;

    for i in usize::MIN..12 {
        let mut pad_number = 2;
        pad_number = usize::pow(pad_number, i.try_into().unwrap());
        if line_count - one_count[11 - i] < one_count[11 - i] {
            gamma = gamma | pad_number;
        } else {
            epsilon = epsilon | pad_number;
        }
    }

    gamma * epsilon
}

pub fn part_two() -> usize {
    let file = File::open("input/day3-1.txt").expect("some error when reading file");
    let lines = io::BufReader::new(file).lines();

    let numbers_result: Result<Vec<String>, _> = lines.collect();
    let numbers = numbers_result.unwrap();

    let mut oxygen_candidates: Vec<usize> = (0..numbers.len()).collect();
    let mut co2_candidates: Vec<usize> = (0..numbers.len()).collect();

    for digit in 0..12 {
        let mut one_count = 0;
        for candidate in &oxygen_candidates {
            if numbers.get(*candidate).unwrap().chars().nth(digit).unwrap() == '1' {
                one_count += 1;
            }
        }
        if oxygen_candidates.len() == 1 {
            break;
        }
        let mut most_common_digit = '0';
        if oxygen_candidates.len() - one_count <= one_count {
            most_common_digit = '1';
        }
        let mut marked_for_deletion_candidates: Vec<usize> = Vec::new();
        for candidate_index in &oxygen_candidates {
            if numbers
                .get(*candidate_index)
                .unwrap()
                .chars()
                .nth(digit)
                .unwrap()
                != most_common_digit
            {
                marked_for_deletion_candidates.push(*candidate_index);
            }
        }
        for filtered_out_candidate in marked_for_deletion_candidates {
            let index = oxygen_candidates
                .iter()
                .position(|x| *x == filtered_out_candidate)
                .unwrap();
            oxygen_candidates.remove(index);
        }
    }

    for digit in 0..12 {
        let mut one_count = 0;
        for candidate in &co2_candidates {
            if numbers.get(*candidate).unwrap().chars().nth(digit).unwrap() == '1' {
                one_count += 1;
            }
        }
        if co2_candidates.len() == 1 {
            break;
        }
        let mut least_common_digit = '0';
        if co2_candidates.len() - one_count > one_count {
            least_common_digit = '1';
        }
        let mut marked_for_deletion_candidates: Vec<usize> = Vec::new();
        for candidate_index in &co2_candidates {
            if numbers
                .get(*candidate_index)
                .unwrap()
                .chars()
                .nth(digit)
                .unwrap()
                != least_common_digit
            {
                marked_for_deletion_candidates.push(*candidate_index);
            }
        }
        for filtered_out_candidate in marked_for_deletion_candidates {
            let index = co2_candidates
                .iter()
                .position(|x| *x == filtered_out_candidate)
                .unwrap();
            co2_candidates.remove(index);
        }
    }

    let oxygen = numbers.get(*oxygen_candidates.get(0).unwrap()).unwrap();
    let co2 = numbers.get(*co2_candidates.get(0).unwrap()).unwrap();
    let oxygen = isize::from_str_radix(oxygen, 2).unwrap();
    let co2 = isize::from_str_radix(co2, 2).unwrap();
    (oxygen * co2) as usize
}
