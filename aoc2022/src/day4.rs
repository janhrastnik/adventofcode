use crate::shared::files;

pub fn solve() {
    part_one();
    part_two();
}

fn part_one() {
    if let Ok(lines) = files::shared::read_lines("./input/day4.txt") {
        let mut count = 0;
        for line in lines {
            if let Ok(sectors_line) = line {
                let mut sectors = sectors_line.split(",");
                let sector_one = sectors
                    .next()
                    .unwrap()
                    .split("-")
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();
                let sector_two = sectors
                    .next()
                    .unwrap()
                    .split("-")
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();
                if (sector_one[0]..sector_one[1] + 1).contains(&sector_two[0])
                    && (sector_one[0]..sector_one[1] + 1).contains(&sector_two[1])
                {
                    count += 1
                } else if (sector_two[0]..sector_two[1] + 1).contains(&sector_one[0])
                    && (sector_two[0]..sector_two[1] + 1).contains(&sector_one[1])
                {
                    count += 1
                }
            }
        }
        println!("PART ONE: {:?}", count);
    }
}

fn part_two() {
    if let Ok(lines) = files::shared::read_lines("./input/day4.txt") {
        let mut count = 0;
        for line in lines {
            if let Ok(sectors_line) = line {
                let mut sectors = sectors_line.split(",");
                let sector_one = sectors
                    .next()
                    .unwrap()
                    .split("-")
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();
                let sector_two = sectors
                    .next()
                    .unwrap()
                    .split("-")
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();
                if (sector_one[0]..sector_one[1] + 1).contains(&sector_two[0])
                    || (sector_one[0]..sector_one[1] + 1).contains(&sector_two[1])
                {
                    count += 1
                } else if (sector_two[0]..sector_two[1] + 1).contains(&sector_one[0])
                    || (sector_two[0]..sector_two[1] + 1).contains(&sector_one[1])
                {
                    count += 1
                }
            }
        }
        println!("PART TWO: {:?}", count);
    }
}
