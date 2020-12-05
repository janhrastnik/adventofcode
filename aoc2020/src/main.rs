mod day1;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day_option = args.get(1);
    let day = day_option
        .expect("error when parsing day number")
        .parse()
        .expect("not a number!");
    match day {
        1 => {
            day1::solve_first();
            day1::solve_second();
        }
        _ => println!("day not found"),
    }
}
