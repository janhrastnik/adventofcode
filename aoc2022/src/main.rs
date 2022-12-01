use std::io;

mod day1;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("user input error");
    match input.trim() {
        "1" => day1::solve(),
        _ => unimplemented!(),
    }
}
