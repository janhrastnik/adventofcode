use std::io;

pub mod shared;

mod day1;
mod day10;
mod day11;
mod day12;
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
    io::stdin().read_line(&mut input).expect("user input error");
    match input.trim() {
        "1" => day1::solve(),
        "2" => day2::solve(),
        "3" => day3::solve(),
        "4" => day4::solve(),
        "5" => day5::solve(),
        "6" => day6::solve(),
        "7" => day7::solve(),
        "8" => day8::solve(),
        "9" => day9::solve(),
        "10" => day10::solve(),
        "11" => day11::solve(),
        "12" => day12::solve(),
        _ => unimplemented!(),
    }
}
