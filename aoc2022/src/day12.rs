use crate::shared::files;
use petgraph::algo::dijkstra;
use petgraph::prelude::*;
use petgraph::Graph;

pub fn solve() {
    part_one();
    part_two();
}

fn part_one() {
    if let Ok(lines) = files::shared::read_lines("./input/day12.txt") {
        let mut grid: Vec<Vec<u32>> = vec![];
        let mut start_row = 0;
        let mut count = 0;
        let mut nodes: Vec<Vec<NodeIndex>> = vec![];
        let mut graph: Graph<(), (), Directed> = Graph::new();
        let mut start_node: Option<NodeIndex> = None;
        let mut end_node: Option<NodeIndex> = None;
        for line in lines {
            if let Ok(row) = line {
                if row.starts_with('S') {
                    start_row = count;
                }
                grid.push(row.chars().map(|x| x as u32).collect::<Vec<u32>>());
                count += 1;
            }
        }
        // graph construction
        for j in 0..grid.len() {
            let mut row: Vec<NodeIndex> = vec![];
            for i in 0..grid[j].len() {
                row.push(graph.add_node(()));
                if grid[j][i] == 'S' as u32 {
                    start_node = Some(row[i]);
                    grid[j][i] = 'a' as u32;
                } else if grid[j][i] == 'E' as u32 {
                    end_node = Some(row[i]);
                    grid[j][i] = 'z' as u32;
                }
                if i > 0 {
                    // check left neighbour
                    if grid[j][i - 1] < grid[j][i] {
                        if grid[j][i] - grid[j][i - 1] == 1 {
                            // "ab"
                            graph.update_edge(row[i - 1], row[i], ());
                            graph.update_edge(row[i], row[i - 1], ());
                        } else {
                            // "ac"
                            graph.update_edge(row[i], row[i - 1], ());
                        }
                    } else {
                        // "ba", "ca" or "aa"
                        if grid[j][i - 1] - grid[j][i] > 1 {
                            // "ca"
                            graph.update_edge(row[i - 1], row[i], ());
                        } else {
                            // "ba" or "aa"
                            graph.update_edge(row[i - 1], row[i], ());
                            graph.update_edge(row[i], row[i - 1], ());
                        }
                    }
                }
                if j > 0 {
                    // check upper neighbour
                    if grid[j - 1][i] < grid[j][i] {
                        if grid[j][i] - grid[j - 1][i] == 1 {
                            // "a"
                            // "b"
                            graph.update_edge(nodes[j - 1][i], row[i], ());
                            graph.update_edge(row[i], nodes[j - 1][i], ());
                        } else {
                            // "a"
                            // "c"
                            graph.update_edge(row[i], nodes[j - 1][i], ());
                        }
                    } else {
                        // "ba", "ca" or "aa"
                        if grid[j - 1][i] - grid[j][i] > 1 {
                            // "c"
                            // "a"
                            graph.update_edge(nodes[j - 1][i], row[i], ());
                        } else {
                            // "a" or "b"
                            // "a" or "a"
                            graph.update_edge(nodes[j - 1][i], row[i], ());
                            graph.update_edge(row[i], nodes[j - 1][i], ());
                        }
                    }
                }
            }
            nodes.push(row);
        }
        let res = dijkstra(&graph, start_node.unwrap(), end_node, |_| 1);
        println!("PART ONE: {:?}", res.get(&end_node.unwrap()).unwrap());
    }
}

fn part_two() {
    if let Ok(lines) = files::shared::read_lines("./input/day12.txt") {
        let mut grid: Vec<Vec<u32>> = vec![];
        let mut start_row = 0;
        let mut count = 0;
        let mut nodes: Vec<Vec<NodeIndex>> = vec![];
        let mut graph: Graph<(), (), Directed> = Graph::new();
        let mut start_node: Option<NodeIndex> = None;
        let mut end_node: Option<NodeIndex> = None;
        let mut candidates: Vec<NodeIndex> = vec![];
        for line in lines {
            if let Ok(row) = line {
                if row.starts_with('S') {
                    start_row = count;
                }
                grid.push(row.chars().map(|x| x as u32).collect::<Vec<u32>>());
                count += 1;
            }
        }
        // graph construction
        for j in 0..grid.len() {
            let mut row: Vec<NodeIndex> = vec![];
            for i in 0..grid[j].len() {
                row.push(graph.add_node(()));
                if grid[j][i] == 'a' as u32 {
                    candidates.push(row[i]);
                } else if grid[j][i] == 'S' as u32 {
                    candidates.push(row[i]);
                    grid[j][i] = 'a' as u32;
                } else if grid[j][i] == 'E' as u32 {
                    end_node = Some(row[i]);
                    grid[j][i] = 'z' as u32;
                }
                if i > 0 {
                    // check left neighbour
                    if grid[j][i - 1] < grid[j][i] {
                        if grid[j][i] - grid[j][i - 1] == 1 {
                            // "ab"
                            graph.update_edge(row[i - 1], row[i], ());
                            graph.update_edge(row[i], row[i - 1], ());
                        } else {
                            // "ac"
                            graph.update_edge(row[i], row[i - 1], ());
                        }
                    } else {
                        // "ba", "ca" or "aa"
                        if grid[j][i - 1] - grid[j][i] > 1 {
                            // "ca"
                            graph.update_edge(row[i - 1], row[i], ());
                        } else {
                            // "ba" or "aa"
                            graph.update_edge(row[i - 1], row[i], ());
                            graph.update_edge(row[i], row[i - 1], ());
                        }
                    }
                }
                if j > 0 {
                    // check upper neighbour
                    if grid[j - 1][i] < grid[j][i] {
                        if grid[j][i] - grid[j - 1][i] == 1 {
                            // "a"
                            // "b"
                            graph.update_edge(nodes[j - 1][i], row[i], ());
                            graph.update_edge(row[i], nodes[j - 1][i], ());
                        } else {
                            // "a"
                            // "c"
                            graph.update_edge(row[i], nodes[j - 1][i], ());
                        }
                    } else {
                        // "ba", "ca" or "aa"
                        if grid[j - 1][i] - grid[j][i] > 1 {
                            // "c"
                            // "a"
                            graph.update_edge(nodes[j - 1][i], row[i], ());
                        } else {
                            // "a" or "b"
                            // "a" or "a"
                            graph.update_edge(nodes[j - 1][i], row[i], ());
                            graph.update_edge(row[i], nodes[j - 1][i], ());
                        }
                    }
                }
            }
            nodes.push(row);
        }
        let mut min = 1000;
        for candidate in candidates {
            let res = dijkstra(&graph, candidate, end_node, |_| 1);
            let x = res.get(&end_node.unwrap());
            if x.is_some() {
                let val = *x.unwrap();
                if val < min {
                    min = val;
                }
            }
        }
        println!("PART TWO: {:?}", min);
    }
}
