use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn into_character_map(word: &str) -> HashMap<char, i32> {
    word.chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    })
}

fn most_frequent_char<K, V>(a_hash_map: &HashMap<K, V>) -> Option<&K>
where
    V: Ord,
{
    a_hash_map
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(k, _v)| k)
}

fn least_frequent_char<K, V>(a_hash_map: &HashMap<K, V>) -> Option<&K>
where
    V: Ord,
{
    a_hash_map
        .iter()
        .min_by(|a, b| a.1.cmp(&b.1))
        .map(|(k, _v)| k)
}

fn solve(steps: usize) -> usize {
    let file = File::open("input/day14.txt").expect("some error when reading file");
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .collect();
    let mut polymer_template: String = lines[0].to_string();
    let mut rules: HashMap<String, char> = HashMap::new();
    for line in lines.iter().skip(2) {
        let rule: Vec<&str> = line.split(" -> ").collect();
        rules.insert(rule[0].to_string(), rule[1].chars().next().unwrap());
    }
    println!("{:?}", rules);

    for step in 0..steps {
        println!("{}", step);
        let mut matched_pairs: HashMap<usize, char> = HashMap::new();
        for (i, char_tuple) in polymer_template
            .chars()
            .zip(polymer_template.chars().skip(1))
            .enumerate()
        {
            let mut polymer_slice = String::from("");
            polymer_slice.push(char_tuple.0);
            polymer_slice.push(char_tuple.1);
            if rules.contains_key(&polymer_slice) {
                matched_pairs.insert(i + 1 + matched_pairs.len(), rules[&polymer_slice[..]]);
            }
        }
        for key in matched_pairs.keys().sorted() {
            polymer_template.insert(*key, matched_pairs[key]);
        }
    }
    let char_map = into_character_map(&polymer_template[..]);
    let max = char_map[most_frequent_char(&char_map).unwrap()];
    let min = char_map[least_frequent_char(&char_map).unwrap()];
    println!("{}, {}, {}", polymer_template.len(), max, min);
    // println!("{:?}", matched_pairs);

    (max - min) as usize
}

pub fn part_one() -> usize {
    solve(10)
}

pub fn part_two() -> usize {
    solve(40)
}
