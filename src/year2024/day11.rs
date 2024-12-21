
pub fn part1(input: &str) -> u128 {
    solve(input, 25)
}

fn solve(input: &str, iterations: usize) -> u128 {
    let stones = input
        .lines().next().unwrap()
        .split(" ")
        .map(|s|{
            s.parse::<u128>().unwrap()
        })
        .collect::<Vec<_>>();
    let mut sum = 0;
    for stone in stones.iter() {
        sum += s(*stone, iterations);
    }
    sum
}

#[cached::proc_macro::cached]
fn s(stone: u128, iterations: usize) -> u128 {
    if iterations == 0 {
        return 1;
    }
    if stone == 0 {
        return s(1, iterations - 1);
    }
    if stone.to_string().chars().count() % 2 == 0 {
        let stone_string = stone.to_string();
        let (left, right) = stone_string.split_at(stone_string.len() / 2);
        return s(left.parse::<u128>().unwrap(), iterations - 1)
            + s(right.parse::<u128>().unwrap(), iterations - 1);
    }
    s(stone * 2024, iterations - 1)
}

pub fn part2(input: &str) -> u128 {
    solve(input, 75)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part2() {
        let result = part2(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
