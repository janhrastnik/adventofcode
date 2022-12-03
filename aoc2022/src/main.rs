use std::io;

pub mod shared;

mod day1;
mod day2;
mod day3;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("user input error");
    match input.trim() {
        "1" => day1::solve(),
        "2" => day2::solve(),
        "3" => day3::solve(),
        _ => unimplemented!(),
    }
}
