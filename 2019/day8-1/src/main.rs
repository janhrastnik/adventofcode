use std::fs;
use std::str;

fn main() {
    let data = fs::read_to_string("input").expect("error");
    let subs = data
        .trim_end()
        .as_bytes()
        .chunks(150)
        .map(str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap();
    println!("{:?}", subs);
    let mut counts: Vec<usize> = vec![];
    for s in &subs {
        let count = s.matches("0").count();
        counts.push(count);
    }
    let min = counts.iter().min().unwrap();
    let index = counts.iter().position(|x| x == min).unwrap();
    println!("{}", index);
    let onecount = subs[index].matches("1").count();
    let twocount = subs[index].matches("2").count();
    let result = onecount * twocount;
    println!("{}", result);
}
