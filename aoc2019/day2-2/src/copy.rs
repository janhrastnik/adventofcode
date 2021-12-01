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
            let mut counter = 0;
            loop {
                if vec[counter] == 1 {
                    let i1 = vec[counter + 1];
                    let i2 = vec[counter + 2];
                    let i3 = vec[counter + 3];
                    let num = vec[i1] + vec[i2];
                    vec[i3] = num;
                    counter += 4;
                } else if vec[counter] == 2 {
                    let i1 = vec[counter + 1];
                    let i2 = vec[counter + 2];
                    let i3 = vec[counter + 3];
                    let num = vec[i1] * vec[i2];
                    vec[i3] = num;
                    counter += 4;
                } else if vec[counter] == 99 {
                    break;
                } else {
                    break;
                }
            }
            if vec[0] == 19690720 {
                let num = 100 * i + j;
                println!("{}", num);
                break 'outer;
            }
        }
    }
}
