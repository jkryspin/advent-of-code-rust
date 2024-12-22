
pub fn part1(input: &str) -> i64 {
    let (left, right) = input.split_once("-").map(|(l, r)| (l.trim().parse::<i64>().unwrap(), r.trim().parse::<i64>().unwrap())).unwrap();
    let mut count = 0;
    for i in left..=right {
        if increasing_digits(i) && has_double(i) {
            count += 1;
        }
    }
    count
}

pub fn part2(input: &str) -> u32 {
    let (left, right) = input.split_once("-").map(|(l, r)| (l.trim().parse::<i64>().unwrap(), r.trim().parse::<i64>().unwrap())).unwrap();
    let mut count = 0;
    for i in left..=right {
        if increasing_digits(i) && has_double2(i) {
            count += 1;
        }
    }
    count
}

fn increasing_digits(num: i64) -> bool {
    let mut prev = 0;
    for c in num.to_string().chars() {
        let digit = c.to_digit(10).unwrap();
        if digit < prev {
            return false;
        }
        prev = digit;
    }
    true
}
fn has_double(num: i64) -> bool {
    let mut prev = 0;
    for c in num.to_string().chars() {
        let digit = c.to_digit(10).unwrap();
        if digit == prev {
            return true;
        }
        prev = digit;
    }
    false
}

fn has_double2(num: i64) -> bool {
    let mut prev = 0;
    let mut count = 1;
    for c in num.to_string().chars() {
        let digit = c.to_digit(10).unwrap();
        if digit == prev {
            count += 1;
        } else {
            if count == 2 {
                return true;
            }
            count = 1;
        }
        prev = digit;
    }
    count == 2
}
