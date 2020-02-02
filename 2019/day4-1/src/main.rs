fn main() {
    let mut count = 0;
    for i in 156218..652528 {
        count += check(i);
    }
    println!("{}", count);
}

fn check(i: usize) -> usize {
    let digits: Vec<_> = i
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect();
    let mut flag = false;
    for d in 0..digits.len() - 1 {
        if digits[d] > digits[d + 1] {
            flag = false;
            break;
        } else if digits[d] == digits[d + 1] {
            flag = true;
        }
    }
    if flag {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use crate::check;

    #[test]
    fn test1() {
        assert_eq!(check(111111), 1);
    }

    #[test]
    fn test2() {
        assert_eq!(check(223450), 0);
    }

    #[test]
    fn test3() {
        assert_eq!(check(123789), 0);
    }

    #[test]
    fn test4() {
        assert_eq!(check(122345), 1);
    }
}
