use crate::shared::files;

pub fn solve() {
    part_one();
    part_two();
}

fn part_one() {
    if let Ok(lines) = files::shared::read_lines("./input/day8.txt") {
        let mut grid: Vec<Vec<isize>> = Vec::new();
        for line in lines {
            if let Ok(row) = line {
                grid.push(
                    row.chars()
                        .map(|x| x.to_digit(10).unwrap() as isize)
                        .collect::<Vec<isize>>(),
                );
            }
        }
        let n_row = grid[0].len();
        let n_column = grid.len();
        let mut bool_grid: Vec<Vec<bool>> = (0..n_column)
            .map(|_y| (0..n_row).map(|_x| false).collect())
            .collect();
        // check from 4 directions, up down left right
        //left
        for row in 0..grid.len() {
            let mut highest = -1;
            for tree in 0..grid[row].len() {
                if grid[row][tree] > highest {
                    highest = grid[row][tree];
                    bool_grid[row][tree] = true;
                }
                if grid[row][tree] == 9 {
                    break;
                }
            }
        }
        // right
        for row in (0..grid.len()).rev() {
            let mut highest = -1;
            for tree in (0..grid[row].len()).rev() {
                if grid[row][tree] > highest {
                    highest = grid[row][tree];
                    bool_grid[row][tree] = true;
                }
                if grid[row][tree] == 9 {
                    break;
                }
            }
        }
        // up
        for row in 0..grid[0].len() {
            let mut highest = -1;
            for tree in 0..grid.len() {
                if grid[tree][row] > highest {
                    highest = grid[tree][row];
                    bool_grid[tree][row] = true;
                }
                if grid[tree][row] == 9 {
                    break;
                }
            }
        }
        //down
        for row in (0..grid[0].len()).rev() {
            let mut highest = -1;
            for tree in (0..grid.len()).rev() {
                if grid[tree][row] > highest {
                    highest = grid[tree][row];
                    bool_grid[tree][row] = true;
                }
                if grid[tree][row] == 9 {
                    break;
                }
            }
        }
        let mut sum = 0;
        for row in bool_grid {
            row.iter().for_each(|x| {
                if *x {
                    sum += 1;
                }
            })
        }

        println!("PART ONE: {:?}", sum);
    }
}

fn part_two() {
    if let Ok(lines) = files::shared::read_lines("./input/day8.txt") {
        let mut grid: Vec<Vec<isize>> = Vec::new();
        for line in lines {
            if let Ok(row) = line {
                grid.push(
                    row.chars()
                        .map(|x| x.to_digit(10).unwrap() as isize)
                        .collect::<Vec<isize>>(),
                );
            }
        }
        let n_row = grid[0].len();
        let n_column = grid.len();
        let mut highest = 0;
        for j in 0..n_column {
            for i in 0..n_row {
                let tree = grid[j][i];

                let mut left_count: usize = 1;
                let mut left_score = 0;
                loop {
                    if (i as isize - left_count as isize) < 0 {
                        break;
                    }
                    let left = grid[j].get(i - left_count);

                    if left.is_none() {
                        break;
                    } else {
                        if left.unwrap() < &tree {
                            left_score += 1;
                        } else {
                            left_score += 1;
                            break;
                        }
                    }
                    left_count += 1;
                }

                let mut right_count: usize = 1;
                let mut right_score = 0;
                loop {
                    let right = grid[j].get(i + right_count);

                    if right.is_none() {
                        break;
                    } else {
                        if right.unwrap() < &tree {
                            right_score += 1;
                        } else {
                            right_score += 1;
                            break;
                        }
                    }
                    right_count += 1;
                }

                let mut down_count: usize = 1;
                let mut down_score = 0;
                loop {
                    let down = grid.get(j + down_count);

                    if down.is_none() {
                        break;
                    } else {
                        if down.unwrap()[i] < tree {
                            down_score += 1;
                        } else {
                            down_score += 1;
                            break;
                        }
                    }
                    down_count += 1;
                }

                let mut up_count: usize = 1;
                let mut up_score = 0;
                loop {
                    if (j as isize - up_count as isize) < 0 {
                        break;
                    }

                    let up = grid.get(j - up_count);

                    if up.is_none() {
                        break;
                    } else {
                        if up.unwrap()[i] < tree {
                            up_score += 1;
                        } else {
                            up_score += 1;
                            break;
                        }
                    }
                    up_count += 1;
                }

                let scenic_score = left_score * right_score * up_score * down_score;
                if scenic_score > highest {
                    highest = scenic_score;
                }
            }
        }

        println!("PART TWO: {:?}", highest);
    }
}
