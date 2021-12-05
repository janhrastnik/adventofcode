use std::io;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() {
    println!("Hello, world!");
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
        _ => println!("Wrong input!"),
    }
}
