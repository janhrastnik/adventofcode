use std::fs::File;
use std::io::{self, BufRead};

fn median(numbers: &mut Vec<isize>) -> isize {
    numbers.sort();
    let mid = numbers.len() / 2;
    numbers[mid]
}

fn average(numbers: &Vec<isize>) -> isize {
    numbers.iter().sum::<isize>() as isize / numbers.len() as isize
}

pub fn part_one() -> isize {
    let file = File::open("input/day7-1.txt").expect("some error when reading file");
    let mut crabs: Vec<isize> = io::BufReader::new(file)
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<isize>().unwrap())
        .collect();
    let mut final_fuel_count = 0;
    let crab_median = median(&mut crabs);
    for crab in &crabs {
        final_fuel_count += (crab_median - crab).abs()
    }
    final_fuel_count
}
pub fn part_two() -> isize {
    let file = File::open("input/day7-1.txt").expect("some error when reading file");
    let mut crabs: Vec<isize> = io::BufReader::new(file)
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<isize>().unwrap())
        .collect();
    let crab_average = average(&crabs);
    let epsilon = crabs.len() as isize;
    let crab_min = crab_average - 2;
    let crab_max = crab_average + 2;
    crabs.sort();
    let mut fuel_count_guesses: Vec<isize> = Vec::new();

    for guess in crab_min..crab_max {
        let mut guess_fuel_count = 0;
        for crab in &crabs {
            let distance = (guess - crab).abs();
            guess_fuel_count += distance * (distance + 1) / 2
        }
        fuel_count_guesses.push(guess_fuel_count);
    }

    *fuel_count_guesses.iter().min().unwrap()
}
