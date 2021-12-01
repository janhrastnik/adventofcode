use intcode::Intcode;
use std::fs;

fn main() {
    let data = fs::read_to_string("input").expect("Error");
    let mut vec: Vec<usize> = data
        .trim_end()
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    vec[1] = 12;
    vec[2] = 2;
    let mut intcode = Intcode {
        memory: &mut vec,
        counter: 0,
    };
    while intcode.exec_cycle() {}
    println!("{}", intcode.memory[0]);
}
