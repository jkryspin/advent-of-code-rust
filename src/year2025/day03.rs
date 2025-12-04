//! # Day 3: [Title]

pub fn part1(input: &str) -> usize {
    let mut sum = 0;

    for line in input.lines() {
        let mut max_number = 0;
        let digits: Vec<u32> = line.chars()
            .filter_map(|c| c.to_digit(10))
            .collect();

        if digits.len() < 2 {
            continue;
        }

        // left_max tracks the highest digit seen so far
        let mut left_max = digits[0];

        // Iterate right pointer through remaining digits
        for right in 1..digits.len() {
            // Form a two-digit number with left_max and current right digit
            let number = left_max * 10 + digits[right];
            max_number = std::cmp::max(max_number, number);

            // Update left_max for next iteration
            left_max = std::cmp::max(left_max, digits[right]);
        }

        sum += max_number;
    }

    sum as usize
}

pub fn part2(input: &str) -> usize {
    let mut sum = 0;

    for line in input.lines() {
        let digits: Vec<u32> = line.chars()
            .filter_map(|c| c.to_digit(10))
            .collect();

        if digits.len() < 12 {
            continue;
        }

        // Greedy approach: select 12 digits that form the maximum number
        let mut selected: Vec<u32> = Vec::new();
        let target_len = 12;

        for i in 0..digits.len() {
            // While we can still remove digits from selected and current digit is larger
            // and we have enough remaining digits to reach target_len
            while !selected.is_empty()
                && selected.last().unwrap() < &digits[i]
                && selected.len() + (digits.len() - i) > target_len {
                selected.pop();
            }

            if selected.len() < target_len {
                selected.push(digits[i]);
            }
        }

        // Convert selected digits to a number
        let mut number: u64 = 0;
        for &digit in &selected[..12] {
            number = number * 10 + digit as u64;
        }

        sum += number;
    }

    sum as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_simple() {
        assert_eq!(part1("91112"), 92);
    }

    #[test]
    fn test_simple_part2() {
        // From "987654321111111" we should select the 12 largest digits
        // Greedy: 9,8,7,6,5,4,3,2,1,1,1,1 = 987654321111
        assert_eq!(part2("987654321111111"), 987654321111);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 357);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 3121910778619);
    }
}
