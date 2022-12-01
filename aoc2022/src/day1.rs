use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn solve() {
    println!("- - - SOLUTIONS - - -");
    part_one();
    part_two();
}

fn part_one() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input/day1.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut counted_calories: Vec<usize> = Vec::new();
        let mut calorie_count = 0;
        for line in lines {
            if let Ok(calorie) = line {
                if calorie == "" {
                    counted_calories.push(calorie_count);
                    calorie_count = 0;
                } else {
                    calorie_count += calorie.parse::<usize>().unwrap();
                }
            }
        }
        println!("PART ONE: {:?}", counted_calories.iter().max().unwrap());
    }
}

fn part_two() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input/day1.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut counted_calories: Vec<usize> = Vec::new();
        let mut calorie_count = 0;
        for line in lines {
            if let Ok(calorie) = line {
                if calorie == "" {
                    counted_calories.push(calorie_count);
                    calorie_count = 0;
                } else {
                    calorie_count += calorie.parse::<usize>().unwrap();
                }
            }
        }
        counted_calories.push(calorie_count);
        counted_calories.sort();
        println!(
            "PART TWO: {:?}",
            counted_calories.iter().rev().take(3).sum::<usize>()
        );
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
