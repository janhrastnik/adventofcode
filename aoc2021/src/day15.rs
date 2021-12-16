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

    // inflate node_distances with 'infinity' distance for each node
    for j in 0..column_len {
        for i in 0..row_len {
            node_distances.insert((j, i), isize::MAX);
        }
    }

    let mut unvisited_nodes: Vec<(isize, isize)> = Vec::new();
    for j in 0..column_len {
        for i in 0..row_len {
            unvisited_nodes.push((j, i));
        }
    }

    *node_distances.get_mut(&(0, 0)).unwrap() = 0;

    while true {
        // find neighbours of current node, check up, down, left, right
        let mut neighbours = Vec::new();

        if unvisited_nodes.is_empty() {
            break;
        }

        let index = unvisited_nodes
            .iter()
            .position(|x| *x == current_pos)
            .unwrap();
        unvisited_nodes.remove(index);
        if current_pos.0 > 0 && unvisited_nodes.contains(&(current_pos.0 - 1, current_pos.1)) {
            // UP
            neighbours.push((current_pos.0 - 1, current_pos.1));
        }
        if current_pos.0 < column_len - 1
            && unvisited_nodes.contains(&(current_pos.0 + 1, current_pos.1))
        {
            // DOWN
            neighbours.push((current_pos.0 + 1, current_pos.1));
        }
        if current_pos.1 > 0 && unvisited_nodes.contains(&(current_pos.0, current_pos.1 - 1)) {
            // LEFT
            neighbours.push((current_pos.0, current_pos.1 - 1));
        }
        if current_pos.1 < row_len - 1
            && unvisited_nodes.contains(&(current_pos.0, current_pos.1 + 1))
        {
            // RIGHT
            neighbours.push((current_pos.0, current_pos.1 + 1));
        }
        println!("{:?}", neighbours);

        // assess distance to each neighbour, save it to node_distances
        let mut min_distance = ((0, 0), isize::MAX);
        for neighbour in &neighbours {
            let distance = node_distances[&(current_pos.0, current_pos.1)]
                + grid[neighbour.0 as usize][neighbour.1 as usize];
            println!(
                "{}, {}",
                distance,
                node_distances[&(neighbour.0, neighbour.1)]
            );

            if distance < node_distances[&(neighbour.0, neighbour.1)] {
                println!("this should run");
                *node_distances.get_mut(&(neighbour.0, neighbour.1)).unwrap() = distance;
            }
        }
        // find unvisited node with smallest distance
        for node in &unvisited_nodes {
            let distance = node_distances[&(node.0, node.1)];
            if distance < min_distance.1 {
                min_distance = ((node.0, node.1), distance);
            }
        }
        current_pos = (min_distance.0 .0, min_distance.0 .1);
    }

    println!("{:?}", node_distances);
    println!("{:?}", grid);
    node_distances[&(column_len - 1, row_len - 1)] as usize
}

pub fn part_two() -> usize {
    0
}
