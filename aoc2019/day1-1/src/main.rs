use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut count: u32 = 0;

    for line in reader.lines() {
        let s: String = line.unwrap();
        let num: u32 = s.parse().unwrap();
        count += num / 3 - 2
    }
    println!("{}", count);

    Ok(())
}
