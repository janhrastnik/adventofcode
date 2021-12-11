use std::io;

mod day1;
mod day10;
mod day11;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");
    match input.trim() {
        "1-1" => println!("{}", day1::part_one()),
        "1-2" => println!("{}", day1::part_two()),
        "2-1" => println!("{}", day2::part_one()),
        "2-2" => println!("{}", day2::part_two()),
        "3-1" => println!("{}", day3::part_one()),
        "3-2" => println!("{}", day3::part_two()),
        "4-1" => println!("{}", day4::part_one()),
        "4-2" => println!("{}", day4::part_two()),
        "5-1" => println!("{}", day5::part_one()),
        "5-2" => println!("{}", day5::part_two()),
        "6-1" => println!("{}", day6::part_one()),
        "6-2" => println!("{}", day6::part_two()),
        "7-1" => println!("{}", day7::part_one()),
        "7-2" => println!("{}", day7::part_two()),
        "8-1" => println!("{}", day8::part_one()),
        "8-2" => println!("{}", day8::part_two()),
        "9-1" => println!("{}", day9::part_one()),
        "9-2" => println!("{}", day9::part_two()),
        "10-1" => println!("{}", day10::part_one()),
        "10-2" => println!("{}", day10::part_two()),
        "11-1" => println!("{}", day11::part_one()),
        "11-2" => println!("{}", day11::part_two()),
        _ => println!("Wrong input!"),
    }
}
