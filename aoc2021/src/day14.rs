use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

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
    let polymer_template: Vec<char> = lines[0].chars().collect();
    let mut rules: HashMap<String, char> = HashMap::new();
    for line in lines.iter().skip(2) {
        let rule: Vec<&str> = line.split(" -> ").collect();
        rules.insert(rule[0].to_string(), rule[1].chars().next().unwrap());
    }
    // get the starting pairs, store them in a hashmap
    let mut starting_pairs: HashMap<String, usize> = HashMap::new();
    for i in 0..polymer_template.len() {
        let b = polymer_template.get(i + 1);
        if b.is_some() {
            let a = polymer_template.get(i);
            let mut pair = String::new();
            pair.push(*a.unwrap());
            pair.push(*b.unwrap());
            *starting_pairs.entry(pair).or_insert(0) += 1;
        }
    }

    // get the count of characters for starting template
    let mut character_count_map = polymer_template.into_iter().counts();

    for _step in 0..steps {
        // iterate over the pairs, match them with rules
        let mut new_pairs = starting_pairs.clone();
        for pair in starting_pairs.keys() {
            if rules.contains_key(pair) {
                // on match, remove the matched pair, add the two new pairs
                // increase character count by the matched character
                let count = starting_pairs.get(pair).unwrap();
                let x = *rules.get(pair).unwrap();
                *new_pairs.entry(pair.clone()).or_insert(0) -= count;
                *character_count_map.entry(x).or_insert(0) += count;
                let a = pair.chars().next().unwrap();
                let b = pair.chars().last().unwrap();
                let mut ax: String = String::new();
                ax.push(a);
                ax.push(x);
                let mut xb: String = String::new();
                xb.push(x);
                xb.push(b);
                *new_pairs.entry(ax).or_insert(0) += count;
                *new_pairs.entry(xb).or_insert(0) += count;
            }
        }
        starting_pairs = new_pairs.clone();
    }
    let max = character_count_map[most_frequent_char(&character_count_map).unwrap()];
    let min = character_count_map[least_frequent_char(&character_count_map).unwrap()];

    (max - min) as usize
}

pub fn part_one() -> usize {
    solve(10)
}

pub fn part_two() -> usize {
    solve(40)
}
