use std::str::SplitWhitespace;

use crate::shared::files;

pub fn solve() {
    part_one();
    part_two();
}

fn part_one() {
    if let Ok(lines) = files::shared::read_lines("./input/day10.txt") {
        let interesting_cycles: Vec<usize> = vec![20, 60, 100, 140, 180, 220];
        let mut sum = 0;
        let mut x: isize = 1; // register
        let mut cycle: usize = 1;
        for line in lines {
            if let Ok(command_line) = line {
                let mut command_iter: SplitWhitespace = command_line.split_whitespace();
                let _command = command_iter.next().unwrap();
                let val = command_iter.next(); // is none if command is 'noop'
                if val.is_some() {
                    // command is 'addx'
                    for _i in 0..2 {
                        // takes 2 cycles to complete
                        if interesting_cycles.contains(&cycle) {
                            sum += cycle as isize * x;
                        }
                        cycle += 1;
                    }
                    x += val.unwrap().parse::<isize>().unwrap();
                } else {
                    // command is 'noop'
                    if interesting_cycles.contains(&cycle) {
                        sum += cycle as isize * x;
                    }
                    cycle += 1;
                }
            }
        }
        println!("PART ONE: {:?}", sum);
    }
}

fn part_two() {
    if let Ok(lines) = files::shared::read_lines("./input/day10.txt") {
        let mut x: isize = 1; // register
        let mut cycle: usize = 0;
        let mut crt: Vec<String> = Vec::new();

        let mut row: String = String::new();

        for line in lines {
            if let Ok(command_line) = line {
                let mut command_iter: SplitWhitespace = command_line.split_whitespace();
                let _command = command_iter.next().unwrap();
                let val = command_iter.next(); // is none if command is 'noop'
                if val.is_some() {
                    // command is 'addx'
                    for _i in 0..2 {
                        // takes 2 cycles to complete
                        if (x - 1 == cycle as isize)
                            || (x == cycle as isize)
                            || (x + 1 == cycle as isize)
                        {
                            row.push('#');
                        } else {
                            row.push('.');
                        }
                        if row.len() == 40 {
                            crt.push(row);
                            row = String::new();
                        }
                        cycle += 1;
                        if cycle == 40 {
                            cycle = 0;
                        }
                    }
                    x += val.unwrap().parse::<isize>().unwrap();
                } else {
                    // command is 'noop'
                    if (x - 1 == cycle as isize)
                        || (x == cycle as isize)
                        || (x + 1 == cycle as isize)
                    {
                        row.push('#');
                    } else {
                        row.push('.');
                    }
                    if row.len() == 40 {
                        crt.push(row);
                        row = String::new();
                    }
                    cycle += 1;
                    if cycle == 40 {
                        cycle = 0;
                    }
                }
            }
        }
        println!("- - - PART TWO - - -");
        for row in crt {
            println!("{:?}", row);
        }
    }
}
