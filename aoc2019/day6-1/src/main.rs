use std::collections::HashMap;
use std::fs;

fn get_node_distance(
    curr_node: &str,
    target_node: &str,
    mut counter: usize,
    nodesmap: &mut HashMap<&str, Vec<&str>>,
) -> usize {
    if curr_node == target_node {
        return counter;
    } else {
        if let Some(nodesvec) = nodesmap.get_mut(curr_node) {
            let newnodesvec = nodesvec.clone();
            counter += 1;
            for node in newnodesvec {
                let distance = get_node_distance(node, target_node, counter, nodesmap);
                if distance == 0 {
                    continue;
                } else {
                    return distance;
                }
            }
            0
        } else {
            0
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
    for node in keys {
        counter += get_node_distance("COM", node, 0, &mut graph);
    }
    println!("{}", counter);
}
