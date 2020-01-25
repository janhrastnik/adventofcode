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
    println!("{:?}", vec);
    let mut counter = 0;
    loop {
        println!("{}", counter);
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
            println!("Ended succesfully.");
            break;
        } else {
            println!("Something went wrong.");
            break;
        }
    }
    println!("{:?}", vec);
    println!("{}", vec[0])
}
