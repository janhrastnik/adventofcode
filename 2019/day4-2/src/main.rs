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
        }
        if (d > 0) & (d < digits.len() - 2) {
            if (digits[d - 1] != digits[d])
                & (digits[d] == digits[d + 1])
                & (digits[d] != digits[d + 2])
            {
                flag = true;
            }
        } else if d == 0 {
            if (digits[d] == digits[d + 1]) & (digits[d] != digits[d + 2]) {
                flag = true;
            }
        } else if d == digits.len() - 2 {
            if (digits[d - 1] != digits[d]) & (digits[d] == digits[d + 1]) {
                flag = true;
            }
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
        assert_eq!(check(112233), 1);
    }

    #[test]
    fn test2() {
        assert_eq!(check(123444), 0);
    }

    #[test]
    fn test3() {
        assert_eq!(check(111122), 1);
    }
}
