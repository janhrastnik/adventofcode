use std::collections::HashMap;
use std::fs;

fn get_node_distance(
    curr_node: &str,
    target_node: &str,
    mut counter: usize,
    nodesmap: &mut HashMap<&str, Vec<&str>>,
) -> usize {
    println!("{}", curr_node);
    if curr_node == target_node {
        return counter;
    } else {
        if let Some(nodesvec) = nodesmap.get_mut(curr_node) {
            counter += 1;
            println!("{:?}", nodesvec);
            if nodesvec.contains(&target_node) {
                println!("found it");
                return counter;
            } else {
                for node in nodesvec {
                    return get_node_distance(node, target_node, counter, nodesmap);
                }
                counter
            }
        } else {
            println!("{}", curr_node);
            counter
        }
    }
}

fn main() {
    let data = fs::read_to_string("input").expect("Error");
    let vec: Vec<&str> = data.split_whitespace().collect();
    let mut nodes: HashMap<&str, Vec<_>> = HashMap::new();
    for instruction in vec {
        let orbit: Vec<&str> = instruction.split(")").collect();
        if !nodes.contains_key(orbit[0]) {
            let neighbours = vec![orbit[1]];
            nodes.insert(orbit[0], neighbours);
        } else {
            nodes.get_mut(orbit[0]).unwrap().push(orbit[1]);
        }
    }
    println!("{:?}", nodes);
    let distance = get_node_distance("COM", "H", 0, &mut nodes);
    println!("{}", distance);
}
