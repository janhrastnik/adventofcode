use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn solve_first() {
    if let Ok(lines) = read_lines("../input/day1.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut big_num_vec = Vec::new();
        let mut small_num_vec = Vec::new();
        for line in lines {
            if let Ok(ip) = line {
                let n: usize = ip.parse().expect("not a number.");
                if n > 1009 {
                    big_num_vec.push(n);
                    continue;
                }
                small_num_vec.push(n);
            }
        }
        big_num_vec.sort();
        small_num_vec.sort();
        for big_n in big_num_vec.into_iter().rev() {
            for small_n in &small_num_vec {
                if big_n + small_n == 2020 {
                    println!("{:?}", big_n * small_n);
                } else if big_n + small_n > 2020 {
                    break;
                }
            }
        }
    }
}

pub fn solve_second() {
    if let Ok(lines) = read_lines("../input/day1.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut num_vec = Vec::new();
        for line in lines {
            if let Ok(ip) = line {
                let n: usize = ip.parse().expect("not a number.");
                num_vec.push(n);
            }
        }
        num_vec.sort();
        'first: for n in 0..num_vec.len() {
            let n1 = num_vec.get(n).unwrap();
            for i in 0..num_vec.len() {
                let n2 = num_vec.get(i).unwrap();
                if n2 + n1 >= 2020 {
                    break;
                }
                if i != n {
                    for j in 0..num_vec.len() {
                        let n3 = num_vec.get(j).unwrap();
                        if n2 + n3 + n1 == 2020 && n != j && i != j {
                            println!("{:?}", n1 * n2 * n3);
                            break 'first;
                        }
                    }
                }
            }
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
