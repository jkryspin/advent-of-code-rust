use Dir::{Left, Right};
use regex::Regex;
use std::collections::HashMap;

pub fn part1(input: &str) -> u32 {
    //RL
    //
    // AAA = (BBB, CCC)
    // BBB = (DDD, EEE)
    // CCC = (ZZZ, GGG)
    // DDD = (DDD, DDD)
    // EEE = (EEE, EEE)
    // GGG = (GGG, GGG)
    // ZZZ = (ZZZ, ZZZ)
    let steps = get_steps(input);
    let map = get_map(input);
    let mut key = "AAA".to_string();
    let mut i = 0;
    let mut count = 0;
    while key != "ZZZ".to_string() {
        let dir = if let Some(s) = steps.get(i) {
            s
        } else {
            i = 0;
            steps.get(i).unwrap()
        };
        let (left, right) = map.get(&key).unwrap();
        match dir {
            Left => {
                key = left.to_string();
            }
            Right => {
                key = right.to_string();
            }
        }
        count += 1;
        i += 1;
    }

    count
}

pub fn part2(input: &str) -> u128 {
    let steps = get_steps(input);
    let map = get_map(input);
    let mut keys = map
        .keys()
        .filter(|key| key.ends_with("A"))
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let mut first_z = HashMap::<String, u128>::new();
    for key in keys.iter_mut() {
        let mut i = 0;
        let mut count = 0;
        while !key.ends_with("Z") {
            let dir = if let Some(s) = steps.get(i) {
                s
            } else {
                i = 0;
                steps.get(i).unwrap()
            };
            let (left, right) = map.get(&key.to_string()).unwrap();
            match dir {
                Left => {
                    *key = left.to_string();
                }
                Right => {
                    *key = right.to_string();
                }
            }
            count += 1;
            i += 1;
        }
        first_z.insert(key.clone(), count);
    }
    let g = lcm(first_z
        .values()
        .map(|s| s.clone())
        .collect::<Vec<u128>>()
        .as_slice());

    g
}

fn get_map(input: &str) -> HashMap<String, (String, String)> {
    let re = Regex::new(r"\w+").unwrap();
    let mut map = HashMap::<String, (String, String)>::new();

    input
        .split("\n\n")
        .skip(1)
        .next()
        .unwrap()
        .lines()
        .for_each(|l| {
            let mut iter = re.find_iter(l);
            let key = iter.next().unwrap().as_str();
            let left = iter.next().unwrap().as_str();
            let right = iter.next().unwrap().as_str();
            map.insert(key.to_string(), (left.to_string(), right.to_string()));
        });
    return map;
}

fn get_steps(input: &str) -> Vec<Dir> {
    input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| if c == 'L' { Left } else { Right })
        .collect()
}

enum Dir {
    Left,
    Right,
}

pub fn lcm(nums: &[u128]) -> u128 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: u128, b: u128) -> u128 {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        let result = part1(input);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_part2() {
        let result = part2(
            "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)",
        );
        assert_eq!(result, 6);
    }
}
