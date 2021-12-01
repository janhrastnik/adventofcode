use std::io;

mod day1;

fn main() {
    println!("Hello, world!");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");
    match input.trim() {
        "1-1" => println!("{}", day1::part_one()),
        "1-2" => println!("{}", day1::part_two()),
        _ => println!("Wrong input!"),
    }
}
