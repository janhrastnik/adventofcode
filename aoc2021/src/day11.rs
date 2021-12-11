use std::fs::File;
use std::io::{self, BufRead};

pub fn part_one() -> usize {
    let file = File::open("input/day11.txt").expect("some error when reading file");
    let mut grid: Vec<Vec<usize>> = io::BufReader::new(file)
        .lines()
        .map(|x| {
            x.unwrap()
                .chars()
                .map(|c| c as usize - '0' as usize)
                .collect()
        })
        .collect();
    let steps = 100;
    let mut flash_count = 0;
    for _step in 0..steps {
        // first increase each value by 1
        // then search for values equal to 10
        let mut pending_flash_indices: Vec<(isize, isize)> = Vec::new();
        let mut applied_flash_indices: Vec<(isize, isize)> = Vec::new();
        grid = grid
            .iter()
            .map(|row| row.iter().map(|x| x + 1).collect::<Vec<usize>>())
            .collect::<Vec<Vec<usize>>>();
        for j in 0..10 {
            for i in 0..10 {
                if grid[j][i] == 10 {
                    pending_flash_indices.push((j as isize, i as isize));
                    applied_flash_indices.push((j as isize, i as isize));
                }
            }
        }
        while pending_flash_indices.len() > 0 {
            let flash = pending_flash_indices.pop().unwrap();
            grid[flash.0 as usize][flash.1 as usize] = 0;
            applied_flash_indices.push(flash);
            flash_count += 1;
            for j in -1..2 {
                for i in -1..2 {
                    let col = flash.0 + j;
                    let row = flash.1 + i;
                    if col >= 0
                        && row >= 0
                        && col < 10
                        && row < 10
                        && (flash.0, flash.1) != (col, row)
                        && !applied_flash_indices.contains(&(col, row))
                    {
                        if grid[col as usize][row as usize] != 0 {
                            grid[col as usize][row as usize] += 1;
                            if grid[col as usize][row as usize] >= 10
                                && !pending_flash_indices.contains(&(col, row))
                            {
                                pending_flash_indices.push((col, row));
                            }
                        }
                    }
                }
            }
        }
    }
    flash_count
}

pub fn part_two() -> usize {
    let file = File::open("input/day11.txt").expect("some error when reading file");
    let mut grid: Vec<Vec<usize>> = io::BufReader::new(file)
        .lines()
        .map(|x| {
            x.unwrap()
                .chars()
                .map(|c| c as usize - '0' as usize)
                .collect()
        })
        .collect();
    let steps = 1000;
    let mut first_step_with_all_zeros = 0;
    for step in 0..steps {
        // first increase each value by 1
        // then search for values equal to 10
        let mut pending_flash_indices: Vec<(isize, isize)> = Vec::new();
        let mut applied_flash_indices: Vec<(isize, isize)> = Vec::new();
        let mut is_zero = true;
        grid = grid
            .iter()
            .map(|row| row.iter().map(|x| x + 1).collect::<Vec<usize>>())
            .collect::<Vec<Vec<usize>>>();
        for j in 0..10 {
            for i in 0..10 {
                if grid[j][i] == 10 {
                    pending_flash_indices.push((j as isize, i as isize));
                    applied_flash_indices.push((j as isize, i as isize));
                }
            }
        }
        while pending_flash_indices.len() > 0 {
            let flash = pending_flash_indices.pop().unwrap();
            grid[flash.0 as usize][flash.1 as usize] = 0;
            applied_flash_indices.push(flash);
            for j in -1..2 {
                for i in -1..2 {
                    let col = flash.0 + j;
                    let row = flash.1 + i;
                    if col >= 0
                        && row >= 0
                        && col < 10
                        && row < 10
                        && (flash.0, flash.1) != (col, row)
                        && !applied_flash_indices.contains(&(col, row))
                    {
                        if grid[col as usize][row as usize] != 0 {
                            grid[col as usize][row as usize] += 1;
                            if grid[col as usize][row as usize] >= 10
                                && !pending_flash_indices.contains(&(col, row))
                            {
                                pending_flash_indices.push((col, row));
                            }
                        }
                    }
                }
            }
        }
        for j in 0..10 {
            for i in 0..10 {
                if grid[j][i] != 0 {
                    is_zero = false;
                }
            }
        }
        if is_zero {
            first_step_with_all_zeros = step;
            break;
        }
    }
    first_step_with_all_zeros + 1
}
