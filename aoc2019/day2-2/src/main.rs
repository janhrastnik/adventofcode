use intcode::Intcode;
use std::fs;

fn main() {
    let data = fs::read_to_string("input").expect("Error");
    'outer: for i in 0..100 {
        'inner: for j in 0..100 {
            let mut vec: Vec<usize> = data
                .trim_end()
                .split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            vec[1] = i;
            vec[2] = j;
            let mut intcode = Intcode {
                memory: &mut vec,
                counter: 0,
            };
            while intcode.exec_cycle() {}
            if vec[0] == 19690720 {
                let num = 100 * i + j;
                println!("{}", num);
                break 'outer;
            }
        }
    }
}
