use std::collections::HashMap;
use std::fs;

fn get_visited_nodes<'a>(
    curr_node: &str,
    target_node: &str,
    visited: Vec<&'a str>,
    nodesmap: &mut HashMap<&str, Vec<&'a str>>,
) -> Vec<&'a str> {
    if curr_node == target_node {
        return visited;
    } else {
        if let Some(nodesvec) = nodesmap.get_mut(curr_node) {
            let newnodesvec = nodesvec.clone();
            for node in newnodesvec {
                visited.push(node);
                let distance: Vec<_> = get_visited_nodes(node, target_node, visited, nodesmap);
                if distance.is_empty() {
                    continue;
                } else {
                    return distance;
                }
            }
            vec![]
        } else {
            vec![]
        }
    }
}

fn main() {
    let data = fs::read_to_string("realinput").expect("Error");
    let vec: Vec<&str> = data.split_whitespace().collect();
    let mut graph: HashMap<&str, Vec<_>> = HashMap::new();
    let mut keys: Vec<&str> = vec![];
    for instruction in vec {
        let orbit: Vec<&str> = instruction.split(")").collect();
        if !keys.contains(&orbit[0]) {
            keys.push(orbit[0]);
        }
        if !keys.contains(&orbit[1]) {
            keys.push(orbit[1]);
        }
        if !graph.contains_key(orbit[0]) {
            let neighbours = vec![orbit[1]];
            graph.insert(orbit[0], neighbours);
        } else {
            graph.get_mut(orbit[0]).unwrap().push(orbit[1]);
        }
    }
    let mut counter = 0;
    println!("{}", counter);
}
