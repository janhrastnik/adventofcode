use std::fs::File;
use std::io::{self, BufRead};

pub fn part_one() -> usize {
    let file = File::open("input/day2-1.txt").expect("some error when reading file");
    let lines = io::BufReader::new(file).lines();
    let mut depth: usize = 0;
    let mut distance: usize = 0;

    for line in lines {
        if let Ok(instruction) = line {
            let instruction_vec = instruction.split(" ").collect::<Vec<&str>>();
            match instruction_vec.as_slice() {
                ["forward", x] => distance += x.parse::<usize>().unwrap(),
                ["down", x] => depth += x.parse::<usize>().unwrap(),
                ["up", x] => depth -= x.parse::<usize>().unwrap(),
                _ => println!("f"),
            }
        }
    }

    depth * distance
}

pub fn part_two() -> usize {
    let file = File::open("input/day2-1.txt").expect("some error when reading file");
    let lines = io::BufReader::new(file).lines();
    let mut depth: usize = 0;
    let mut distance: usize = 0;
    let mut aim: usize = 0;

    for line in lines {
        if let Ok(instruction) = line {
            let instruction_vec = instruction.split(" ").collect::<Vec<&str>>();
            match instruction_vec.as_slice() {
                ["forward", x] => {
                    distance += x.parse::<usize>().unwrap();
                    depth += aim * x.parse::<usize>().unwrap();
                    println!("aaa");
                }
                ["down", x] => aim += x.parse::<usize>().unwrap(),
                ["up", x] => aim -= x.parse::<usize>().unwrap(),
                _ => println!("f"),
            }
        }
    }

    depth * distance
}
