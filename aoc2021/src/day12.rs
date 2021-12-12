use std::fs::File;
use std::io::{self, BufRead};

struct Graph {
    nodes: Vec<Node>,
    starting_index: usize,
}

#[derive(Clone, Debug)]
struct Node {
    identifier: String,
    edges: Vec<usize>,
}

/*
recursive function that counts the number of walks from a node to the end
first check if node is 'end', if it is then walk is done
else, if node identifier is lowercase, then we add it to our list of visited nodes (uppercase nodes have no bounds on number of visits)
then call this function on nodes that are connected to this one and have not yet been visited
visit limit is used to solve part two, where a walk can (only once) visit a particular node twice
*/
fn find_paths(
    graph: &Graph,
    current_node_index: usize,
    completed_walk_count: &mut usize,
    mut visited_nodes_count: Vec<usize>,
    mut visit_limit: usize,
) {
    // check if we are at the end of the walk
    if graph.nodes[current_node_index].identifier == "end" {
        *completed_walk_count += 1;
    } else {
        // check if node is lowercase
        if !(&graph.nodes[current_node_index].identifier[..]
            == &graph.nodes[current_node_index].identifier[..].to_uppercase())
        {
            // start node can be visited only once, no matter the visit limit
            if graph.nodes[current_node_index].identifier == "start" {
                visited_nodes_count[current_node_index] += visit_limit;
            } else {
                visited_nodes_count[current_node_index] += 1;
                if visited_nodes_count[current_node_index] == 2 {
                    // a node has been visited twice, therefore rest of the walk, all lowercase nodes can be visited once
                    visit_limit = 1
                }
            }
        }
        // repeat for edge nodes
        for edge_node_index in &graph.nodes[current_node_index].edges {
            // check if number of times edge node has been visited is below the visit limit
            if visited_nodes_count[*edge_node_index] < visit_limit {
                find_paths(
                    graph,
                    *edge_node_index,
                    completed_walk_count,
                    visited_nodes_count.clone(),
                    visit_limit,
                );
            }
        }
    }
}

fn solve(visit_limit: usize) -> usize {
    let file = File::open("input/day12.txt").expect("some error when reading file");
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .collect();

    let mut graph = Graph {
        nodes: Vec::new(),
        starting_index: 0,
    };

    for line in lines {
        let mut splitted = line.split("-");
        let a_identifier = splitted.next().unwrap().to_string();
        let b_identifier = splitted.next().unwrap().to_string();
        for identifier in [&a_identifier, &b_identifier] {
            // check if nodes are in graph, add them if they aren't
            if !graph
                .nodes
                .iter()
                .map(|node| &node.identifier)
                .collect::<Vec<&String>>()
                .contains(&identifier)
            {
                graph.nodes.push(Node {
                    identifier: identifier.clone(),
                    edges: Vec::new(),
                });
            };
        }
        // need the indices for edge assignment
        let (mut a_index, mut b_index) = (0, 0);
        for (i, node) in graph.nodes.iter().enumerate() {
            if node.identifier == a_identifier {
                a_index = i;
            } else if node.identifier == b_identifier {
                b_index = i;
            }
        }
        // edges are undirected, so we must assign a -> b and b -> a, we also assign our starting index
        if b_identifier != String::from("start") {
            graph.nodes[a_index].edges.push(b_index);
        } else {
            graph.starting_index = b_index;
        }
        if a_identifier != String::from("start") {
            graph.nodes[b_index].edges.push(a_index);
        } else {
            graph.starting_index = a_index;
        }
    }
    let mut count = 0;
    find_paths(
        &graph,
        graph.starting_index,
        &mut count,
        vec![0; graph.nodes.len()],
        visit_limit,
    );
    count
}

pub fn part_one() -> usize {
    solve(1)
}

pub fn part_two() -> usize {
    solve(2)
}
