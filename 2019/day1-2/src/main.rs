use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut count = 0;

    for line in reader.lines() {
        let s = line.unwrap();
        let mut fuel: i32 = s.parse().unwrap();
        while fuel > 0 {
            acquire_fuel(&mut fuel);
            count += fuel
        }
    }
    println!("{}", count);

    Ok(())
}

fn acquire_fuel(n: &mut i32) {
    *n = *n / 3 - 2;
    if *n < 0 {
        *n = 0
    }
}
