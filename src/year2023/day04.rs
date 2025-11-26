use std::collections::HashSet;

pub fn part1(input: &str) -> u64 {
    let lines = input.lines();
    let mut total = 0;
    lines.for_each(|l| {
        let (_, winning_numbers, my_numbers) = parse(l);
        let win_hash = HashSet::<u64>::from_iter(winning_numbers);
        let mut sum = 0;
        my_numbers.iter().for_each(|n| {
            if win_hash.contains(n) {
                if sum == 0 {
                    sum = 1;
                } else {
                    sum = sum * 2;
                }
            }
        });
        total += sum;
    });
    total
}

fn parse(input: &str) -> (u64, Vec<u64>, Vec<u64>) {
    let mut parts = input.split("|");
    let left = parts.next().unwrap();
    let right = parts.next().unwrap();
    let mut card_split = left.split(":");
    let id = card_split
        .next()
        .unwrap()
        .split_whitespace()
        .into_iter()
        .nth(1)
        .unwrap()
        .parse::<u64>()
        .unwrap();
    let winning_numbers = parse_numbers(card_split.next().unwrap().trim());
    let my_numbers = parse_numbers(right);
    return (id, winning_numbers, my_numbers);
}
fn parse_numbers(input: &str) -> Vec<u64> {
    return input
        .split_whitespace()
        .map(|w| {
            return w.parse::<u64>().unwrap();
        })
        .collect::<Vec<u64>>();
}

pub fn part2(input: &str) -> u64 {
    let lines = input.lines();
    let cards = lines
        .map(|l| {
            let (id, winning_numbers, my_numbers) = parse(l);
            let win_hash = HashSet::<u64>::from_iter(winning_numbers);
            let my_hash = HashSet::<u64>::from_iter(my_numbers);
            let sum = my_hash.intersection(&win_hash).count();
            return (id as usize, sum);
        })
        .collect::<Vec<(usize, usize)>>();

    let mut totals = vec![1; cards.len()];
    for (card_number, sum) in cards.into_iter() {
        let location = card_number - 1;
        for index in (location)..(location + sum) {
            totals[index + 1] += totals[location];
        }
    }
    totals.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let result = part1(input);
        assert_eq!(result, 13);
    }

    #[test]
    fn test_part2() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let result = part2(input);
        assert_eq!(result, 30);
    }
}
