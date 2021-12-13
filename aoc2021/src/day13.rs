use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

pub fn part_one() -> usize {
    let file = File::open("input/day13.txt").expect("some error when reading file");
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .collect();
    let mut points: Vec<(usize, usize)> = Vec::new();
    let mut folds: Vec<(char, usize)> = Vec::new();
    let mut are_points = true;
    for line in lines {
        if are_points {
            if line == "" {
                are_points = false;
            } else {
                let temp: Vec<usize> = line
                    .split(",")
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect();
                points.push((temp[0], temp[1]));
            }
        } else {
            let temp: Vec<&str> = line.split("=").collect();
            folds.push((
                temp[0].chars().last().unwrap(),
                temp[1].parse::<usize>().unwrap(),
            ));
        }
    }
    let fold = folds.first().unwrap();
    let mut translated_points: Vec<(usize, usize)> = Vec::new();
    for point in points {
        if fold.0 == 'x' {
            if point.0 > fold.1 {
                translated_points.push((point.0 - 2 * (point.0 - fold.1), point.1));
            } else if point.0 < fold.1 {
                translated_points.push(point);
            }
        }
        if fold.0 == 'y' {
            if point.1 > fold.1 {
                translated_points.push((point.0, point.1 - 2 * (point.1 - fold.1)));
            } else if point.1 < fold.1 {
                translated_points.push(point);
            }
        }
    }
    let hash_set: HashSet<(usize, usize)> = translated_points.into_iter().collect();
    hash_set.len()
}

pub fn part_two() -> usize {
    let file = File::open("input/day13.txt").expect("some error when reading file");
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .collect();
    let mut points: Vec<(usize, usize)> = Vec::new();
    let mut folds: Vec<(char, usize)> = Vec::new();
    let mut are_points = true;
    for line in lines {
        if are_points {
            if line == "" {
                are_points = false;
            } else {
                let temp: Vec<usize> = line
                    .split(",")
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect();
                points.push((temp[0], temp[1]));
            }
        } else {
            let temp: Vec<&str> = line.split("=").collect();
            folds.push((
                temp[0].chars().last().unwrap(),
                temp[1].parse::<usize>().unwrap(),
            ));
        }
    }
    for fold in folds {
        let mut translated_points: Vec<(usize, usize)> = Vec::new();
        for point in &points {
            if fold.0 == 'x' {
                if point.0 > fold.1 {
                    translated_points.push((point.0 - 2 * (point.0 - fold.1), point.1));
                } else if point.0 < fold.1 {
                    translated_points.push(point.clone());
                }
            }
            if fold.0 == 'y' {
                if point.1 > fold.1 {
                    translated_points.push((point.0, point.1 - 2 * (point.1 - fold.1)));
                } else if point.1 < fold.1 {
                    translated_points.push(point.clone());
                }
            }
        }
        points = translated_points;
    }
    let hash_set: HashSet<(usize, usize)> = points.into_iter().collect();
    let mut l = vec![vec!['_'; 100]; 20];
    for point in hash_set {
        l[point.1][point.0] = '*';
    }
    for row in l {
        let row_str: String = row.into_iter().map(|i| i.to_string()).collect::<String>();
        println!("{:?}", row_str);
    }
    0
}
