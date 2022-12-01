use hex;
use std::fs::File;
use std::io::{self, BufRead};

pub fn part_one() -> usize {
    let file = File::open("input/day16.txt").expect("some error when reading file");
    let sample_line = "D2FE28";
    let lines: String = io::BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .next()
        .unwrap();
    let test = &sample_line[..3];
    let hex_to_decimal = hex::decode(sample_line).unwrap();
    let mut binary_string = String::new();
    for decimal in hex_to_decimal {
        let decimal_to_binary = format!("{:b}", decimal);
        binary_string.push_str(&decimal_to_binary);
    }
    println!("{}", binary_string);
    println!("{}", test);
    let version = &binary_string[..3];
    let type_id = &binary_string[3..6];
    println!("{}, {}", version, type_id);
    0
}

pub fn part_two() -> usize {
    0
}
