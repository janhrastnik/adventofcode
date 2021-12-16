use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

pub fn part_one() -> usize {
    let file = File::open("input/day15.txt").expect("some error when reading file");
    let grid: Vec<Vec<isize>> = io::BufReader::new(file)
        .lines()
        .map(|x| {
            x.unwrap()
                .chars()
                .map(|c| c as isize - '0' as isize)
                .collect()
        })
        .collect();
    let mut node_distances: HashMap<(isize, isize), isize> = HashMap::new();
    let mut current_pos = (0, 0);
    let column_len = grid.len() as isize;
    let row_len = grid[0].len() as isize;
    let mut visited_nodes: HashMap<(isize, isize), bool> = HashMap::new();

    // inflate node_distances with 'infinity' distance for each node
    for j in 0..column_len {
        for i in 0..row_len {
            node_distances.insert((j, i), isize::MAX);
        }
    }

    for j in 0..column_len {
        for i in 0..row_len {
            visited_nodes.insert((j, i), false);
        }
    }

    *node_distances.get_mut(&(0, 0)).unwrap() = 0;

    let mut visitable_map: HashMap<(isize, isize), isize> = HashMap::new();
    loop {
        // find neighbours of current node, check up, down, left, right
        let mut neighbours = Vec::new();
        *visited_nodes.get_mut(&current_pos).unwrap() = true;

        if current_pos.0 > 0 && visited_nodes[&(current_pos.0 - 1, current_pos.1)] == false {
            // UP
            neighbours.push((current_pos.0 - 1, current_pos.1));
        }
        if current_pos.0 < column_len - 1
            && visited_nodes[&(current_pos.0 + 1, current_pos.1)] == false
        {
            // DOWN
            neighbours.push((current_pos.0 + 1, current_pos.1));
        }
        if current_pos.1 > 0 && visited_nodes[&(current_pos.0, current_pos.1 - 1)] == false {
            // LEFT
            neighbours.push((current_pos.0, current_pos.1 - 1));
        }
        if current_pos.1 < row_len - 1
            && visited_nodes[&(current_pos.0, current_pos.1 + 1)] == false
        {
            // RIGHT
            neighbours.push((current_pos.0, current_pos.1 + 1));
        }

        // assess distance to each neighbour, save it to node_distances
        for neighbour in &neighbours {
            let distance =
                node_distances[&current_pos] + grid[neighbour.0 as usize][neighbour.1 as usize];
            let potential = visitable_map.get(&neighbour);
            if potential.is_some() {
                if distance < visitable_map[&neighbour] {
                    visitable_map.insert(*neighbour, distance);
                }
            } else {
                visitable_map.insert(*neighbour, distance);
            }
            if distance < node_distances[&neighbour] {
                *node_distances.get_mut(&neighbour).unwrap() = distance;
            }
        }
        if visitable_map.is_empty() {
            break;
        }
        let next_node = visitable_map
            .iter()
            .min_by_key(|entry| entry.1)
            .unwrap()
            .0
            .clone();
        visitable_map.remove(&next_node);
        current_pos = next_node;
    }

    node_distances[&(column_len - 1, row_len - 1)] as usize
}

pub fn part_two() -> usize {
    let file = File::open("input/day15.txt").expect("some error when reading file");
    let mut grid: Vec<Vec<isize>> = io::BufReader::new(file)
        .lines()
        .map(|x| {
            x.unwrap()
                .chars()
                .map(|c| c as isize - '0' as isize)
                .collect()
        })
        .collect();

    for row in &mut grid {
        let mut row2: Vec<isize> = row
            .clone()
            .iter()
            .map(|x| if *x >= 9 { *x - 8 } else { *x + 1 })
            .collect();

        let mut row3: Vec<isize> = row
            .clone()
            .iter()
            .map(|x| if *x >= 8 { *x - 7 } else { *x + 2 })
            .collect();

        let mut row4: Vec<isize> = row
            .clone()
            .iter()
            .map(|x| if *x >= 7 { *x - 6 } else { *x + 3 })
            .collect();
        let mut row5: Vec<isize> = row
            .clone()
            .iter()
            .map(|x| if *x >= 6 { *x - 5 } else { *x + 4 })
            .collect();

        row.append(&mut row2);
        row.append(&mut row3);
        row.append(&mut row4);
        row.append(&mut row5);
    }
    let original_grid = grid.clone();
    for i in 1..5 {
        let grid2 = original_grid.clone();
        for row in &grid2 {
            let row = row
                .iter()
                .map(|x| if *x >= 10 - i { *x - 9 + i } else { *x + i })
                .collect();
            grid.push(row);
        }
    }

    let mut node_distances: HashMap<(isize, isize), isize> = HashMap::new();
    let mut current_pos = (0, 0);
    let column_len = grid.len() as isize;
    let row_len = grid[0].len() as isize;
    let mut visited_nodes: HashMap<(isize, isize), bool> = HashMap::new();

    // inflate node_distances with 'infinity' distance for each node
    for j in 0..column_len {
        for i in 0..row_len {
            node_distances.insert((j, i), isize::MAX);
        }
    }

    for j in 0..column_len {
        for i in 0..row_len {
            visited_nodes.insert((j, i), false);
        }
    }

    *node_distances.get_mut(&(0, 0)).unwrap() = 0;

    let mut visitable_map: HashMap<(isize, isize), isize> = HashMap::new();
    loop {
        // find neighbours of current node, check up, down, left, right
        let mut neighbours = Vec::new();
        *visited_nodes.get_mut(&current_pos).unwrap() = true;

        if current_pos.0 > 0 && visited_nodes[&(current_pos.0 - 1, current_pos.1)] == false {
            // UP
            neighbours.push((current_pos.0 - 1, current_pos.1));
        }
        if current_pos.0 < column_len - 1
            && visited_nodes[&(current_pos.0 + 1, current_pos.1)] == false
        {
            // DOWN
            neighbours.push((current_pos.0 + 1, current_pos.1));
        }
        if current_pos.1 > 0 && visited_nodes[&(current_pos.0, current_pos.1 - 1)] == false {
            // LEFT
            neighbours.push((current_pos.0, current_pos.1 - 1));
        }
        if current_pos.1 < row_len - 1
            && visited_nodes[&(current_pos.0, current_pos.1 + 1)] == false
        {
            // RIGHT
            neighbours.push((current_pos.0, current_pos.1 + 1));
        }

        // assess distance to each neighbour, save it to node_distances
        for neighbour in &neighbours {
            let distance =
                node_distances[&current_pos] + grid[neighbour.0 as usize][neighbour.1 as usize];
            let potential = visitable_map.get(&neighbour);
            if potential.is_some() {
                if distance < visitable_map[&neighbour] {
                    visitable_map.insert(*neighbour, distance);
                }
            } else {
                visitable_map.insert(*neighbour, distance);
            }
            if distance < node_distances[&neighbour] {
                *node_distances.get_mut(&neighbour).unwrap() = distance;
            }
        }
        if visitable_map.is_empty() {
            break;
        }
        let next_node = visitable_map
            .iter()
            .min_by_key(|entry| entry.1)
            .unwrap()
            .0
            .clone();
        visitable_map.remove(&next_node);
        current_pos = next_node;
    }

    node_distances[&(column_len - 1, row_len - 1)] as usize
}
