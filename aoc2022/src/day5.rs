use crate::shared::files;

pub fn solve() {
    part_one();
    part_two();
}

fn part_one() {
    let mut stack: Vec<Vec<char>> = vec![
        vec!['R', 'N', 'P', 'G'],
        vec!['T', 'J', 'B', 'L', 'C', 'S', 'V', 'H'],
        vec!['T', 'D', 'B', 'M', 'N', 'L'],
        vec!['R', 'V', 'P', 'S', 'B'],
        vec!['G', 'C', 'Q', 'S', 'W', 'M', 'V', 'H'],
        vec!['W', 'Q', 'S', 'C', 'D', 'B', 'J'],
        vec!['F', 'Q', 'L'],
        vec!['W', 'M', 'H', 'T', 'D', 'L', 'F', 'V'],
        vec!['L', 'P', 'B', 'V', 'M', 'J', 'F'],
    ];

    if let Ok(lines) = files::shared::read_lines("./input/day5.txt") {
        let mut letters: String = String::new();
        for line_result in lines {
            if let Ok(line) = line_result {
                if line.starts_with("move") {
                    // [1] is amount, [3] is start stack, [5] is end stack
                    let instructions: Vec<Result<usize, _>> = line
                        .split_whitespace()
                        .map(|x| x.parse::<usize>())
                        .collect();
                    let n = instructions[1].as_ref().unwrap().clone();
                    let start = instructions[3].as_ref().unwrap().clone() - 1;
                    let end = instructions[5].as_ref().unwrap().clone() - 1;
                    for i in 0..n {
                        let removed = stack[start].pop().unwrap();
                        stack[end].push(removed);
                    }
                }
            }
        }
        for i in 0..stack.len() {
            letters.push(stack[i].pop().unwrap());
        }
        println!("PART ONE: {:?}", letters);
    }
}

fn part_two() {
    let mut stack: Vec<Vec<char>> = vec![
        vec!['R', 'N', 'P', 'G'],
        vec!['T', 'J', 'B', 'L', 'C', 'S', 'V', 'H'],
        vec!['T', 'D', 'B', 'M', 'N', 'L'],
        vec!['R', 'V', 'P', 'S', 'B'],
        vec!['G', 'C', 'Q', 'S', 'W', 'M', 'V', 'H'],
        vec!['W', 'Q', 'S', 'C', 'D', 'B', 'J'],
        vec!['F', 'Q', 'L'],
        vec!['W', 'M', 'H', 'T', 'D', 'L', 'F', 'V'],
        vec!['L', 'P', 'B', 'V', 'M', 'J', 'F'],
    ];

    if let Ok(lines) = files::shared::read_lines("./input/day5.txt") {
        let mut letters: String = String::new();
        for line_result in lines {
            if let Ok(line) = line_result {
                if line.starts_with("move") {
                    // [1] is amount, [3] is start stack, [5] is end stack
                    let instructions: Vec<Result<usize, _>> = line
                        .split_whitespace()
                        .map(|x| x.parse::<usize>())
                        .collect();
                    let n = instructions[1].as_ref().unwrap().clone();
                    let start = instructions[3].as_ref().unwrap().clone() - 1;
                    let end = instructions[5].as_ref().unwrap().clone() - 1;
                    let mut temp_stack: Vec<char> = vec![];
                    for i in 0..n {
                        let removed = stack[start].pop().unwrap();
                        temp_stack.push(removed);
                    }
                    stack[end].extend(temp_stack.iter().rev());
                }
            }
        }
        for i in 0..stack.len() {
            letters.push(stack[i].pop().unwrap());
        }
        println!("PART TWO: {:?}", letters);
    }
}
