use std::cmp;
use std::fs::File;
use std::io::{self, BufRead};

pub fn part_one() -> usize {
    let file = File::open("input/day5-1.txt").expect("some error when reading file");
    let lines = io::BufReader::new(file).lines();
    let mut canvas: Vec<Vec<usize>> = Vec::new();
    for _ in 0..1000 {
        canvas.push(vec![0; 1000]);
    }

    for line in lines {
        if let Ok(coords) = line {
            let mut is_horizontal = false;
            let coords: Vec<&str> = coords.split(" -> ").collect();
            let xy1: Vec<usize> = coords[0]
                .split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            let xy2: Vec<usize> = coords[1]
                .split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            if xy1[0] != xy2[0] && xy1[1] != xy2[1] {
                // line is nor horizontal nor vertical
                continue;
            }
            if xy1[0] != xy2[0] {
                is_horizontal = true;
            }
            if is_horizontal {
                for i in cmp::min(xy1[0], xy2[0])..cmp::max(xy1[0], xy2[0]) + 1 {
                    canvas[xy1[1]][i] += 1;
                }
            } else {
                for j in cmp::min(xy1[1], xy2[1])..cmp::max(xy1[1], xy2[1]) + 1 {
                    canvas[j][xy1[0]] += 1;
                }
            }
        }
    }
    let mut intersection_count = 0;

    for row in canvas {
        intersection_count += row.iter().filter(|x| **x >= 2).count();
    }

    intersection_count
}

pub fn part_two() -> usize {
    let file = File::open("input/day5-1.txt").expect("some error when reading file");
    let lines = io::BufReader::new(file).lines();
    let mut canvas: Vec<Vec<usize>> = Vec::new();
    for _ in 0..1000 {
        canvas.push(vec![0; 1000]);
    }

    for line in lines {
        if let Ok(coords) = line {
            let mut is_horizontal = false;
            let coords: Vec<&str> = coords.split(" -> ").collect();
            let xy1: Vec<usize> = coords[0]
                .split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            let xy2: Vec<usize> = coords[1]
                .split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            if xy1[0] != xy2[0] && xy1[1] != xy2[1] {
                // line is diagonal, lets make the calculations here
                let delta = (xy1[0] as isize - xy2[0] as isize).abs() as usize;
                let x_ascending = xy1[0] < xy2[0];
                let y_ascending = xy1[1] < xy2[1];
                for k in 0..delta + 1 {
                    if x_ascending && y_ascending {
                        canvas[xy1[1] + k][xy1[0] + k] += 1;
                    } else if !x_ascending && !y_ascending {
                        canvas[xy1[1] - k][xy1[0] - k] += 1;
                    } else if x_ascending && !y_ascending {
                        canvas[xy1[1] - k][xy1[0] + k] += 1;
                    } else if !x_ascending && y_ascending {
                        canvas[xy1[1] + k][xy1[0] - k] += 1;
                    }
                }
                continue;
            }
            if xy1[0] != xy2[0] {
                is_horizontal = true;
            }
            if is_horizontal {
                for i in cmp::min(xy1[0], xy2[0])..cmp::max(xy1[0], xy2[0]) + 1 {
                    canvas[xy1[1]][i] += 1;
                }
            } else {
                for j in cmp::min(xy1[1], xy2[1])..cmp::max(xy1[1], xy2[1]) + 1 {
                    canvas[j][xy1[0]] += 1;
                }
            }
        }
    }
    let mut intersection_count = 0;

    for row in canvas {
        intersection_count += row.iter().filter(|x| **x >= 2).count();
    }

    intersection_count
}
