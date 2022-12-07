use crate::shared::files;
use std::collections::HashMap;

pub fn solve() {
    part_one();
    part_two();
}

fn part_one() {
    if let Ok(lines) = files::shared::read_lines("./input/day7.txt") {
        let mut pc: HashMap<String, usize> = HashMap::new();
        pc.insert("".to_string(), 0);
        let mut path: Vec<String> = Vec::new();
        let mut sum = 0;
        let mut size = 0;
        for line_option in lines {
            if let Ok(line) = line_option {
                let words: Vec<&str> = line.split_whitespace().collect::<Vec<&str>>();
                if words[0].parse::<usize>().is_err() {
                    let mut buf = String::new();
                    pc.entry("".to_string())
                        .and_modify(|val| *val += size)
                        .or_insert(size);
                    for i in path.clone() {
                        buf.push_str(i.as_str());
                        buf.push_str("/");
                        pc.entry(buf.clone())
                            .and_modify(|val| *val += size)
                            .or_insert(size);
                    }
                    size = 0;
                }
                if words[1] == "cd" {
                    if words[2] == "/" {
                        path = vec![];
                    } else if words[2] == ".." {
                        path.pop();
                    } else {
                        path.push(words[2].to_string());
                    }
                }
                if words[0].parse::<usize>().is_ok() {
                    size += words[0].parse::<usize>().unwrap();
                } else {
                }
            }
        }
        let mut buf = String::new();
        pc.entry("".to_string())
            .and_modify(|val| *val += size)
            .or_insert(size);
        for i in path.clone() {
            buf.push_str(i.as_str());
            buf.push_str("/");
            pc.entry(buf.clone())
                .and_modify(|val| *val += size)
                .or_insert(size);
        }

        for val in pc.values() {
            if *val < 100000 {
                sum += val;
            }
        }

        println!("PART ONE: {:?}", sum)
    }
}

fn part_two() {
    if let Ok(lines) = files::shared::read_lines("./input/day7.txt") {
        let mut pc: HashMap<String, usize> = HashMap::new();
        pc.insert("".to_string(), 0);
        let mut path: Vec<String> = Vec::new();
        let mut sum = 0;
        let mut size = 0;
        for line_option in lines {
            if let Ok(line) = line_option {
                let words: Vec<&str> = line.split_whitespace().collect::<Vec<&str>>();
                if words[0].parse::<usize>().is_err() {
                    let mut buf = String::new();
                    pc.entry("".to_string())
                        .and_modify(|val| *val += size)
                        .or_insert(size);
                    for i in path.clone() {
                        buf.push_str(i.as_str());
                        buf.push_str("/");
                        pc.entry(buf.clone())
                            .and_modify(|val| *val += size)
                            .or_insert(size);
                    }
                    size = 0;
                }
                if words[1] == "cd" {
                    if words[2] == "/" {
                        path = vec![];
                    } else if words[2] == ".." {
                        path.pop();
                    } else {
                        path.push(words[2].to_string());
                    }
                }
                if words[0].parse::<usize>().is_ok() {
                    size += words[0].parse::<usize>().unwrap();
                } else {
                }
            }
        }
        let mut buf = String::new();
        pc.entry("".to_string())
            .and_modify(|val| *val += size)
            .or_insert(size);
        for i in path.clone() {
            buf.push_str(i.as_str());
            buf.push_str("/");
            pc.entry(buf.clone())
                .and_modify(|val| *val += size)
                .or_insert(size);
        }
        let unused = 70000000 - pc.get("").unwrap();
        let diff = 30000000 - unused;
        let dir = pc.values().filter(|x| **x >= diff).min().unwrap();

        println!("PART ONE: {:?}", dir)
    }
}
