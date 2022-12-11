use std::cmp::Ordering;

pub fn solve() {
    part_one();
    part_two();
}

#[derive(Debug, Clone, Eq)]
struct Monkey {
    items: Vec<u128>,
    coefficient: usize,
    is_addition: bool,
    divisor: usize,
    true_monkey: usize,
    false_monkey: usize,
    count: usize,
}

impl Ord for Monkey {
    fn cmp(&self, other: &Self) -> Ordering {
        self.count.cmp(&other.count)
    }
}

impl PartialOrd for Monkey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Monkey {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count
    }
}

fn part_one() {
    let mut monkeys = get_monkeys();
    for _i in 0..20 {
        round(&mut monkeys, true);
    }
    monkeys.sort();
    monkeys.reverse();
    println!("PART ONE: {:?}", monkeys[0].count * monkeys[1].count)
}

fn round(monkeys: &mut Vec<Monkey>, is_part_one: bool) {
    for j in 0..monkeys.len() {
        for i in 0..monkeys[j].items.len() {
            // increase worry level by operation, divide by 3, division check, move item to other monkey
            if monkeys[j].coefficient == 0 {
                // edge case -> represent old * old operation
                monkeys[j].items[i] *= monkeys[j].items[i];
            } else {
                if monkeys[j].is_addition {
                    monkeys[j].items[i] += monkeys[j].coefficient as u128;
                } else {
                    monkeys[j].items[i] *= monkeys[j].coefficient as u128;
                }
            }
            if is_part_one {
                monkeys[j].items[i] = monkeys[j].items[i] / 3;
            } else {
                let n = 2 * 3 * 5 * 7 * 11 * 13 * 17 * 19;
                if monkeys[j].items[i] > n {
                    let k = monkeys[j].items[i] / n;
                    monkeys[j].items[i] = monkeys[j].items[i] - n * k;
                }
            }
            if monkeys[j].items[i] % monkeys[j].divisor as u128 == 0 {
                let true_monkey = monkeys[j].true_monkey;
                let item = monkeys[j].items[i];
                monkeys[true_monkey].items.push(item);
            } else {
                let false_monkey = monkeys[j].false_monkey;
                let item = monkeys[j].items[i];
                monkeys[false_monkey].items.push(item);
            }
            monkeys[j].count += 1;
        }
        monkeys[j].items = vec![];
    }
}

fn part_two() {
    let mut monkeys = get_monkeys();
    for _i in 0..10000 {
        round(&mut monkeys, false);
    }
    monkeys.sort();
    monkeys.reverse();
    println!("PART ONE: {:?}", monkeys[0].count * monkeys[1].count)
}

fn get_monkeys() -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();
    monkeys.push(Monkey {
        items: vec![66, 59, 64, 51],
        coefficient: 3,
        is_addition: false,
        divisor: 2,
        true_monkey: 1,
        false_monkey: 4,
        count: 0,
    });
    monkeys.push(Monkey {
        items: vec![67, 61],
        coefficient: 19,
        is_addition: false,
        divisor: 7,
        true_monkey: 3,
        false_monkey: 5,
        count: 0,
    });
    monkeys.push(Monkey {
        items: vec![86, 93, 80, 70, 71, 81, 56],
        coefficient: 2,
        is_addition: true,
        divisor: 11,
        true_monkey: 4,
        false_monkey: 0,
        count: 0,
    });
    monkeys.push(Monkey {
        items: vec![94],
        coefficient: 0,
        is_addition: false,
        divisor: 19,
        true_monkey: 7,
        false_monkey: 6,
        count: 0,
    });
    monkeys.push(Monkey {
        items: vec![71, 92, 64],
        coefficient: 8,
        is_addition: true,
        divisor: 3,
        true_monkey: 5,
        false_monkey: 1,
        count: 0,
    });
    monkeys.push(Monkey {
        items: vec![58, 81, 92, 75, 56],
        coefficient: 6,
        is_addition: true,
        divisor: 5,
        true_monkey: 3,
        false_monkey: 6,
        count: 0,
    });
    monkeys.push(Monkey {
        items: vec![82, 98, 77, 94, 86, 81],
        coefficient: 7,
        is_addition: true,
        divisor: 17,
        true_monkey: 7,
        false_monkey: 2,
        count: 0,
    });
    monkeys.push(Monkey {
        items: vec![54, 95, 70, 93, 88, 93, 63, 50],
        coefficient: 4,
        is_addition: true,
        divisor: 13,
        true_monkey: 2,
        false_monkey: 0,
        count: 0,
    });
    monkeys
}

fn get_monkeys_example() -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();
    monkeys.push(Monkey {
        items: vec![79, 98],
        coefficient: 19,
        is_addition: false,
        divisor: 23,
        true_monkey: 2,
        false_monkey: 3,
        count: 0,
    });
    monkeys.push(Monkey {
        items: vec![54, 65, 75, 74],
        coefficient: 6,
        is_addition: true,
        divisor: 19,
        true_monkey: 2,
        false_monkey: 0,
        count: 0,
    });
    monkeys.push(Monkey {
        items: vec![79, 60, 97],
        coefficient: 0,
        is_addition: false,
        divisor: 13,
        true_monkey: 1,
        false_monkey: 3,
        count: 0,
    });
    monkeys.push(Monkey {
        items: vec![74],
        coefficient: 3,
        is_addition: true,
        divisor: 17,
        true_monkey: 0,
        false_monkey: 1,
        count: 0,
    });
    monkeys
}
