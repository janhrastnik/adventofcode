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
                            visited.insert(tail.clone(), true);
                        }
                    }
                    "U" => {
                        let count = command_vec[1].parse::<isize>().unwrap();
                        for _i in 0..count {
                            // move head
                            head.y += 1;

                            tail_check(&mut head, &mut tail);
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
                            visited.insert(tail.clone(), true);
                        }
                    }
                    _ => unreachable!(),
                }
            };
        }
        println!("PART ONE: {:?}", visited.keys().len());
    }
}

fn part_two() {
    if let Ok(lines) = files::shared::read_lines("./input/day9.txt") {
        let mut visited: HashMap<Point, bool> = HashMap::new();
        let mut rope: Vec<Point> = Vec::new();
        for _i in 0..10 {
            rope.push(Point { x: 0, y: 0 });
        }
        visited.insert(rope[9].clone(), true);

        for line in lines {
            if let Ok(command) = line {
                let command_vec: Vec<&str> = command.split_whitespace().collect::<Vec<&str>>();
                match command_vec[0] {
                    "R" => {
                        let count = command_vec[1].parse::<isize>().unwrap();
                        for _i in 0..count {
                            // move head
                            rope[0].x += 1;

                            // move tail
                            for i in 0..rope.len() - 1 {
                                tail_check_two(&mut rope, i);
                            }
                            visited.insert(rope[9].clone(), true);
                        }
                    }
                    "L" => {
                        let count = command_vec[1].parse::<isize>().unwrap();
                        for _i in 0..count {
                            // move head
                            rope[0].x -= 1;

                            // move tail
                            for i in 0..rope.len() - 1 {
                                tail_check_two(&mut rope, i);
                            }
                            visited.insert(rope[9].clone(), true);
                        }
                    }
                    "U" => {
                        let count = command_vec[1].parse::<isize>().unwrap();
                        for _i in 0..count {
                            // move head
                            rope[0].y += 1;

                            // move tail
                            for i in 0..rope.len() - 1 {
                                tail_check_two(&mut rope, i);
                            }
                            visited.insert(rope[9].clone(), true);
                        }
                    }
                    "D" => {
                        let count = command_vec[1].parse::<isize>().unwrap();
                        for _i in 0..count {
                            // move head
                            rope[0].y -= 1;

                            // move tail
                            for i in 0..rope.len() - 1 {
                                tail_check_two(&mut rope, i);
                            }
                            visited.insert(rope[9].clone(), true);
                        }
                    }
                    _ => unreachable!(),
                }
            };
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

fn tail_check_two(rope: &mut Vec<Point>, index: usize) {
    if ((rope[index].x - rope[index + 1].x).abs() == 2)
        && ((rope[index].y - rope[index + 1].y).abs() == 1)
    {
        rope[index + 1].x += (rope[index].x - rope[index + 1].x) / 2;
        rope[index + 1].y += rope[index].y - rope[index + 1].y;
    } else if ((rope[index].x - rope[index + 1].x).abs() == 1)
        && ((rope[index].y - rope[index + 1].y).abs() == 2)
    {
        rope[index + 1].x += rope[index].x - rope[index + 1].x;
        rope[index + 1].y += (rope[index].y - rope[index + 1].y) / 2;
    } else if ((rope[index].x - rope[index + 1].x).abs() == 2)
        && ((rope[index].y - rope[index + 1].y).abs() == 2)
    {
        rope[index + 1].x += (rope[index].x - rope[index + 1].x) / 2;
        rope[index + 1].y += (rope[index].y - rope[index + 1].y) / 2;
    } else if (rope[index].x - rope[index + 1].x).abs() == 2 {
        rope[index + 1].x += (rope[index].x - rope[index + 1].x) / 2;
    } else if (rope[index].y - rope[index + 1].y).abs() == 2 {
        rope[index + 1].y += (rope[index].y - rope[index + 1].y) / 2;
    } else if (rope[index].x - rope[index + 1].x).abs() > 1
        || (rope[index].y - rope[index + 1].y).abs() > 1
    {
        unreachable!(
            "x: {}, y: {}, index: {}, rope: {:?}",
            rope[index].x - rope[index + 1].x,
            rope[index].y - rope[index + 1].y,
            index,
            rope
        );
    }
}
