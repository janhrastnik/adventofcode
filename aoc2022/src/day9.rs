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
                            if (head.x - tail.x).abs() > 1 {
                                if head.y != tail.y {
                                    tail.y = head.y;
                                }
                                tail.x += 1;
                            }
                            visited.insert(tail.clone(), true);
                        }
                    }
                    "L" => {
                        let count = command_vec[1].parse::<isize>().unwrap();
                        for _i in 0..count {
                            // move head
                            head.x -= 1;

                            // move tail
                            if (head.x - tail.x).abs() > 1 {
                                if head.y != tail.y {
                                    tail.y = head.y;
                                }
                                tail.x -= 1;
                            }
                            visited.insert(tail.clone(), true);
                        }
                    }
                    "U" => {
                        let count = command_vec[1].parse::<isize>().unwrap();
                        for _i in 0..count {
                            // move head
                            head.y += 1;

                            // move tail
                            if (head.y - tail.y).abs() > 1 {
                                if head.x != tail.x {
                                    tail.x = head.x;
                                }
                                tail.y += 1;
                            }
                            visited.insert(tail.clone(), true);
                        }
                    }
                    "D" => {
                        let count = command_vec[1].parse::<isize>().unwrap();
                        for _i in 0..count {
                            // move head
                            head.y -= 1;

                            // move tail
                            if (head.y - tail.y).abs() > 1 {
                                if head.x != tail.x {
                                    tail.x = head.x;
                                }
                                tail.y -= 1;
                            }
                            visited.insert(tail.clone(), true);
                        }
                    }
                    _ => unreachable!(),
                }
            };
            //println!("{:?}", visited.keys().len());
        }
        println!("PART ONE: {:?}", visited.keys().len());
    }
}

fn part_two() {
    if let Ok(lines) = files::shared::read_lines("./input/day9test2.txt") {
        let mut visited: HashMap<Point, bool> = HashMap::new();
        let mut head = Point { x: 0, y: 0 };
        let mut tail = Point { x: 0, y: 0 };
        let mut rope: Vec<Point> = vec![];
        for _i in 0..10 {
            rope.push(Point { x: 0, y: 0 });
        }
        visited.insert(rope.last().unwrap().clone(), true);

        for line in lines {
            if let Ok(command) = line {
                println!("{:?}", command);
                let command_vec: Vec<&str> = command.split_whitespace().collect::<Vec<&str>>();
                match command_vec[0] {
                    "R" => {
                        let count = command_vec[1].parse::<isize>().unwrap();
                        for _i in 0..count {
                            // move head
                            rope[0].x += 1;

                            for knot in 0..(rope.len() - 1) {
                                // move tail
                                if ((rope[knot].x - rope[knot + 1].x).abs() == 2)
                                    && ((rope[knot].y - rope[knot + 1].y).abs() == 2)
                                {
                                    rope[knot + 1].x += 1;
                                    rope[knot + 1].y += (rope[knot].y - rope[knot + 1].y) / 2;
                                } else if (rope[knot].x - rope[knot + 1].x).abs() == 2 {
                                    rope[knot + 1].x += 1;
                                }
                            }

                            visited.insert(rope.last().unwrap().clone(), true);
                        }
                    }
                    "L" => {
                        let count = command_vec[1].parse::<isize>().unwrap();
                        for _i in 0..count {
                            // move head
                            rope[0].x -= 1;

                            for knot in 0..(rope.len() - 1) {
                                // move tail
                                if ((rope[knot].x - rope[knot + 1].x).abs() == 2)
                                    && ((rope[knot].y - rope[knot + 1].y).abs() == 2)
                                {
                                    rope[knot + 1].x -= 1;
                                    rope[knot + 1].y -= (rope[knot].y - rope[knot + 1].y) / 2;
                                } else if (rope[knot].x - rope[knot + 1].x).abs() == 2 {
                                    rope[knot + 1].x -= 1;
                                }
                            }

                            visited.insert(rope.last().unwrap().clone(), true);
                        }
                    }
                    "U" => {
                        let count = command_vec[1].parse::<isize>().unwrap();
                        for _i in 0..count {
                            // move head
                            rope[0].y += 1;

                            for knot in 0..(rope.len() - 1) {
                                // move tail

                                if ((rope[knot].y - rope[knot + 1].y).abs() == 2)
                                    && (rope[knot].x - rope[knot + 1].x).abs() == 1
                                {
                                    rope[knot + 1].y += 1;
                                    rope[knot + 1].x += 1;
                                } else if (rope[knot].x - rope[knot + 1].x).abs() == 2 {
                                    rope[knot + 1].x += 1;
                                } else if (rope[knot].y - rope[knot + 1].y).abs() == 2 {
                                    rope[knot + 1].y += 1;
                                }
                            }

                            visited.insert(rope.last().unwrap().clone(), true);
                        }
                    }
                    "D" => {
                        let count = command_vec[1].parse::<isize>().unwrap();
                        for _i in 0..count {
                            // move head
                            head.y -= 1;
                            rope[0].y -= 1;

                            for knot in 0..(rope.len() - 1) {
                                // move tail
                                if ((rope[knot].x - rope[knot + 1].x).abs() == 2)
                                    && ((rope[knot].y - rope[knot + 1].y).abs() == 2)
                                {
                                    rope[knot + 1].y -= 1;
                                    rope[knot + 1].x -= (rope[knot].x - rope[knot + 1].x) / 2;
                                } else if (rope[knot].y - rope[knot + 1].y).abs() == 2 {
                                    rope[knot + 1].y -= 1;
                                }
                            }
                            visited.insert(rope.last().unwrap().clone(), true);
                        }
                    }
                    _ => unreachable!(),
                }
            };
            println!("{:?}", rope);
            println!("{:?}", visited.keys().len());
        }
        println!("PART ONE:")
    }
}
