use crate::shared::files;
use std::collections::HashMap;

pub fn solve() {
    part_one();
    part_two();
}

#[derive(Debug, Clone, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

fn part_one() {
    if let Ok(lines) = files::shared::read_lines("./input/day9.txt") {
        let mut visited: HashMap<Point, bool> = HashMap::new();
        let mut head = Point { x: 0, y: 0 };
        let mut tail = Point { x: 0, y: 0 };
        visited.insert(tail.clone(), true);

        for line in lines {
            if let Ok(command) = line {
                let command_vec: Vec<&str> = command.split_whitespace().collect::<Vec<&str>>();
                match command_vec[0] {
                    "R" => {
                        let count = command_vec[1].parse::<isize>().unwrap();
                        for _i in 0..count {
                            // move head
                            head.x += 1;

                            // move tail
                            tail_check(&mut head, &mut tail);
                            println!("{:?}", tail);
                            visited.insert(tail.clone(), true);
                        }
                    }
                    "L" => {
                        let count = command_vec[1].parse::<isize>().unwrap();
                        for _i in 0..count {
                            // move head
                            head.x -= 1;

                            // move tail
                            tail_check(&mut head, &mut tail);
                            println!("{:?}", tail);
                            visited.insert(tail.clone(), true);
                        }
                    }
                    "U" => {
                        let count = command_vec[1].parse::<isize>().unwrap();
                        for _i in 0..count {
                            // move head
                            head.y += 1;

                            tail_check(&mut head, &mut tail);
                            println!("{:?}", tail);
                            visited.insert(tail.clone(), true);
                        }
                    }
                    "D" => {
                        let count = command_vec[1].parse::<isize>().unwrap();
                        for _i in 0..count {
                            // move head
                            head.y -= 1;

                            // move tail
                            tail_check(&mut head, &mut tail);
                            println!("{:?}", tail);
                            visited.insert(tail.clone(), true);
                        }
                    }
                    _ => unreachable!(),
                }
            };
            //println!("{:?}", visited.keys().len());
        }
        println!("PART TWO: {:?}", visited.keys().len());
    }
}

fn part_two() {
    if let Ok(lines) = files::shared::read_lines("./input/day9.txt") {
        let mut visited: HashMap<Point, bool> = HashMap::new();
        let mut head = Point { x: 0, y: 0 };
        let mut tail = Point { x: 0, y: 0 };
        visited.insert(tail.clone(), true);

        for line in lines {
            if let Ok(command) = line {
                let command_vec: Vec<&str> = command.split_whitespace().collect::<Vec<&str>>();
                match command_vec[0] {
                    "R" => {
                        let count = command_vec[1].parse::<isize>().unwrap();
                        for _i in 0..count {
                            // move head
                            head.x += 1;

                            // move tail
                            tail_check(&mut head, &mut tail);
                            println!("{:?}", tail);
                            visited.insert(tail.clone(), true);
                        }
                    }
                    "L" => {
                        let count = command_vec[1].parse::<isize>().unwrap();
                        for _i in 0..count {
                            // move head
                            head.x -= 1;

                            // move tail
                            tail_check(&mut head, &mut tail);
                            println!("{:?}", tail);
                            visited.insert(tail.clone(), true);
                        }
                    }
                    "U" => {
                        let count = command_vec[1].parse::<isize>().unwrap();
                        for _i in 0..count {
                            // move head
                            head.y += 1;

                            tail_check(&mut head, &mut tail);
                            println!("{:?}", tail);
                            visited.insert(tail.clone(), true);
                        }
                    }
                    "D" => {
                        let count = command_vec[1].parse::<isize>().unwrap();
                        for _i in 0..count {
                            // move head
                            head.y -= 1;

                            // move tail
                            tail_check(&mut head, &mut tail);
                            println!("{:?}", tail);
                            visited.insert(tail.clone(), true);
                        }
                    }
                    _ => unreachable!(),
                }
            };
            //println!("{:?}", visited.keys().len());
        }
        println!("PART TWO: {:?}", visited.keys().len());
    }
}

fn tail_check(head: &mut Point, tail: &mut Point) {
    if ((head.x - tail.x).abs() == 2) && ((head.y - tail.y).abs() == 1) {
        tail.x += (head.x - tail.x) / 2;
        tail.y += head.y - tail.y;
    } else if ((head.x - tail.x).abs() == 1) && ((head.y - tail.y).abs() == 2) {
        tail.x += head.x - tail.x;
        tail.y += (head.y - tail.y) / 2;
    } else if (head.x - tail.x).abs() == 2 {
        tail.x += (head.x - tail.x) / 2;
    } else if (head.y - tail.y).abs() == 2 {
        tail.y += (head.y - tail.y) / 2;
    }
}
