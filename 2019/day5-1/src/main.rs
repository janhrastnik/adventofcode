use std::fs;

fn main() {
    let data = fs::read_to_string("input").expect("Error");
    let mut vec: Vec<i32> = data
        .trim_end()
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    vec[1] = 12;
    vec[2] = 2;
    let mut counter = 0;
    loop {
        let op = vec[counter];
        let i1 = vec[counter + 1];
        let i2 = vec[counter + 2];
        let i3 = vec[counter + 3];
        match op {
            1 => vec[i3] = vec[i1] + vec[i2],
            2 => vec[i3] = vec[i1] * vec[i2],
            99 => break,
            _ => println!("something went wrong"),
        }
        counter += 4;
    }
    println!("{:?}", vec);
    println!("{}", vec[0])
}
