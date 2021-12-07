use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

pub fn solve(days: usize) -> usize {
    let file = File::open("input/day6-1.txt").expect("some error when reading file");
    let line: Vec<usize> = io::BufReader::new(file)
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let mut final_fish_count = 0;

    let mut cache: HashMap<usize, Option<usize>> = HashMap::from([
        (0, None),
        (1, None),
        (2, None),
        (3, None),
        (4, None),
        (5, None),
        (6, None),
        (7, None),
        (8, None),
    ]);

    for starting_fish in line {
        if cache[&starting_fish].is_some() {
            final_fish_count += cache[&starting_fish].unwrap();
            continue;
        }
        let mut fishes: HashMap<usize, usize> = HashMap::from([
            (0, 0),
            (1, 0),
            (2, 0),
            (3, 0),
            (4, 0),
            (5, 0),
            (6, 0),
            (7, 0),
            (8, 0),
        ]);
        *fishes.get_mut(&starting_fish).unwrap() += 1;
        let mut day_count = days;
        while day_count > 0 {
            day_count -= 1;
            let fishes_day_later: HashMap<usize, usize> = HashMap::from([
                (0, fishes[&1]),
                (1, fishes[&2]),
                (2, fishes[&3]),
                (3, fishes[&4]),
                (4, fishes[&5]),
                (5, fishes[&6]),
                (6, fishes[&7] + fishes[&0]),
                (7, fishes[&8]),
                (8, fishes[&0]),
            ]);
            fishes = fishes_day_later;
        }
        let mut fish_count = 0;
        for fishes_age_group in &fishes {
            fish_count += fishes_age_group.1
        }
        *cache.get_mut(&starting_fish).unwrap() = Some(fish_count);
        final_fish_count += fish_count;
    }

    final_fish_count
}

pub fn part_one() -> usize {
    solve(80)
}
pub fn part_two() -> usize {
    solve(256)
}
