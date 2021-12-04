use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
struct BingoTable {
    combinations: [Vec<usize>; 10],
    steps: usize,
}

impl BingoTable {
    fn new() -> BingoTable {
        let bingo_table = BingoTable {
            combinations: Default::default(),
            steps: 0,
        };
        bingo_table
    }

    fn check_number(&mut self, drawn_number: usize) {
        for i in 0..10 {
            self.combinations[i].retain(|x| *x != drawn_number);
        }
        self.steps += 1;
    }

    fn check_bingo(&mut self) -> bool {
        for i in 0..10 {
            if self.combinations[i].len() == 0 {
                return true;
            }
        }
        false
    }

    fn summation(&mut self, winning_number: usize) -> (usize, usize) {
        let mut count: usize = 0;
        for i in 0..5 {
            count += self.combinations[i].iter().sum::<usize>();
        }
        (count * winning_number, self.steps)
    }
}

pub fn part_one() -> usize {
    let file = File::open("input/day4-1.txt").expect("some error when reading file");
    let mut lines = io::BufReader::new(file).lines();
    // take first line, that contains the numbers drawn
    let drawn_numbers = lines.next().unwrap().unwrap();
    let drawn_numbers: Vec<usize> = drawn_numbers
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    println!("{:?}", drawn_numbers);
    let mut tables: Vec<BingoTable> = Vec::new();
    let mut temp_table: BingoTable = BingoTable::new();
    let mut row_count = 0;

    // iterate through the rest of the file, define the bingo tables
    for line_result in lines {
        if let Ok(line) = line_result {
            if line == "" {
                // empty line indicates next five lines will be a new bingo table
                for drawn_number in &drawn_numbers {
                    temp_table.check_number(*drawn_number);
                    if temp_table.check_bingo() {
                        println!("{:?}", temp_table.summation(*drawn_number));
                        break;
                    }
                }
                tables.push(temp_table);
                temp_table = BingoTable::new();
                row_count = 0;
            } else {
                // line here is a row of the bingo table
                // we need to add the row itself once as a combination, then we iterate the row to
                // add each element to a column combination
                let row: Vec<usize> = line
                    .split_whitespace()
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect();

                // we add the row
                temp_table.combinations[row_count] = row.clone();

                for (i, number) in row.iter().enumerate() {
                    temp_table.combinations[5 + i].push(*number);
                }

                // println!("{:?}", row);
                row_count += 1;
            }
        }
    }
    for drawn_number in &drawn_numbers {
        temp_table.check_number(*drawn_number);
        if temp_table.check_bingo() {
            println!("{:?}", temp_table.summation(*drawn_number));
            break;
        }
    }
    tables.push(temp_table);

    0
}

pub fn part_two() -> usize {
    part_one()
}
