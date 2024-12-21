
pub fn part1(input: &str) -> u32 {
    let mut count = 0;
    let re = regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    for cap in re.captures_iter(input) {
        let a = cap[1].parse::<u32>().unwrap();
        let b = cap[2].parse::<u32>().unwrap();
        count += a * b;
    }
    count
}

pub fn part2(input: &str) -> u32 {
    let mut count = 0;
    let re = regex::Regex::new(r"mul\((?P<a>\d+),(?P<b>\d+)\)|(?P<do>do\(\))|(?P<dont>don't\(\))")
        .unwrap();
    let mut enabled = true;

    for cap in re.captures_iter(input) {
        match cap.name("do") {
            Some(_) => enabled = true,
            _ => {}
        }

        match cap.name("dont") {
            Some(_) => enabled = false,
            _ => {}
        }

        if enabled {
            match (cap.name("a"), cap.name("b")) {
                (Some(a), Some(b)) => {
                    count +=
                        a.as_str().parse::<u32>().unwrap() * b.as_str().parse::<u32>().unwrap();
                }
                _ => {}
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part2() {
        let result =
            part2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
        assert_eq!(result, Some(48));
    }
}
