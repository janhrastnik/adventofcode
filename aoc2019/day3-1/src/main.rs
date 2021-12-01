use std::fs;

fn main() {
    let data = fs::read_to_string("input").expect("Err");
    let vec: Vec<&str> = data.trim_end().split_whitespace().collect();
    let path1: Vec<&str> = vec[0].split(",").collect();
    let path2: Vec<&str> = vec[1].split(",").collect();
    let mut nodes1 = path_map(path1);
    nodes1.sort_unstable();
    nodes1.dedup();
    let mut nodes2 = path_map(path2);
    nodes2.sort_unstable();
    nodes2.dedup();
    println!("{:?}", nodes2[1009]);
    println!("Found nodes");
    for node1 in nodes1 {
        if nodes2.contains(&node1) {
            println!("{:?}", node1);
        }
    }
    println!("Program finished");
}

fn path_map(vec: Vec<&str>) -> Vec<Vec<i32>> {
    fn change_coor(x: i32, y: i32, curr_x: &mut i32, curr_y: &mut i32, nodes: &mut Vec<Vec<i32>>) {
        for _i in 0..i32::abs(x) {
            if x < 0 {
                *curr_x = *curr_x - 1;
            } else {
                *curr_x += 1;
            }
            let v = vec![*curr_x, *curr_y];
            nodes.push(v);
        }
        for _i in 0..i32::abs(y) {
            if y < 0 {
                *curr_y = *curr_y - 1;
            } else {
                *curr_y += 1;
            }
            let v = vec![*curr_x, *curr_y];
            nodes.push(v);
        }
    }

    let mut curr_x = 0;
    let mut curr_y = 0;
    let mut nodes: Vec<Vec<i32>> = Vec::new();
    for coor in vec {
        let indexes: Vec<_> = coor.split_terminator("").skip(1).collect();
        let num: i32 = indexes[1..].join("").to_string().parse().unwrap();
        match indexes[0] {
            "R" => change_coor(num, 0, &mut curr_x, &mut curr_y, &mut nodes),
            "L" => change_coor(-num, 0, &mut curr_x, &mut curr_y, &mut nodes),
            "U" => change_coor(0, num, &mut curr_x, &mut curr_y, &mut nodes),
            "D" => change_coor(0, -num, &mut curr_x, &mut curr_y, &mut nodes),
            _ => println!("not found"),
        }
    }
    nodes
}
