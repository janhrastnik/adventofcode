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
    let mut indexes: Vec<usize> = (0..150).collect();
    let mut message = vec![2; 150];
    for s in &subs {
        let v: Vec<char> = s.chars().collect();
        let mut removables: Vec<usize> = vec![];
        for i in &mut indexes {
            match v[*i] {
                '1' => {
                    removables.push(*i);
                    message[*i] = 1;
                }
                '0' => {
                    removables.push(*i);
                    message[*i] = 0;
                }
                '2' => {
                    continue;
                }
                _ => unreachable!(),
            }
        }
        removables.sort();
        removables.reverse();
        for removable in removables {
            &indexes.retain(|x| *x != removable);
        }
    }
    let s: String = message
        .into_iter()
        .map(|i| i.to_string())
        .collect::<String>()
        .replace("0", " ")
        .replace("1", "&");

    let result = s
        .trim_end()
        .as_bytes()
        .chunks(25)
        .map(str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap();

    for line in result {
        println!("{}", line);
    }
}
