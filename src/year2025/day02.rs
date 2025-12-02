//! # Day 2: [Title]

pub fn part1(input: &str) -> usize {
    let ranges: Vec<(usize, usize)> = input
        .split(',')
        .map(|range| {
            let parts: Vec<&str> = range.trim().split('-').collect();
            let first = parts[0].parse::<usize>().unwrap();
            let last = parts[1].parse::<usize>().unwrap();
            (first, last)
        })
        .collect();
    let mut invalid_ids = vec![];
    for range in ranges {
        for id in range.0..=range.1 {
            if !is_valid(id) {
                invalid_ids.push(id);
            }
        }
    }

    invalid_ids.into_iter().sum()
}

fn is_valid(id: usize) -> bool {
    // If the first half equals the second half when split down the middle, it's invalid
    // Only even-length numbers can be invalid
    let s = id.to_string();
    let len = s.len();

    // Odd-length numbers are always valid
    if len % 2 == 1 {
        return true;
    }

    let mid = len / 2;
    let first_half = &s[..mid];
    let second_half = &s[mid..];

    first_half != second_half
}

pub fn part2(input: &str) -> usize {
    let ranges: Vec<(usize, usize)> = input
        .split(',')
        .map(|range| {
            let parts: Vec<&str> = range.trim().split('-').collect();
            let first = parts[0].parse::<usize>().unwrap();
            let last = parts[1].parse::<usize>().unwrap();
            (first, last)
        })
        .collect();
    let mut invalid_ids = vec![];
    for range in ranges {
        for id in range.0..=range.1 {
            if !is_valid_2(id) {
                invalid_ids.push(id);
            }
        }
    }

    invalid_ids.into_iter().sum()
}
fn is_valid_2(id: usize) -> bool {
    // Return false (invalid) if it's a repeating pattern entirely, with at least 1 repeat
    let s = id.to_string();
    let len = s.len();

    // Try all possible pattern lengths from 1 to len/2
    for pattern_len in 1..=len/2 {
        // Check if the length is divisible by the pattern length
        if len % pattern_len == 0 {
            let pattern = &s[0..pattern_len];
            let repeats = len / pattern_len;

            // Check if repeating the pattern gives us the original string
            if pattern.repeat(repeats) == s {
                // It's a repeating pattern, so invalid
                return false;
            }
        }
    }

    // No repeating pattern found, so valid
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_is_valid() {
        // Even-length numbers
        assert_eq!(is_valid(11), false); // "1" == "1", invalid
        assert_eq!(is_valid(12), true);  // "1" != "2", valid
        assert_eq!(is_valid(22), false); // "2" == "2", invalid
        assert_eq!(is_valid(99), false); // "9" == "9", invalid
        assert_eq!(is_valid(1212), false); // "12" == "12", invalid
        assert_eq!(is_valid(1211), true);  // "12" != "11", valid
        assert_eq!(is_valid(1010), false); // "10" == "10", invalid

        // Odd-length numbers are always valid
        assert_eq!(is_valid(101), true);
        assert_eq!(is_valid(111), true);
        assert_eq!(is_valid(123), true);
    }

    #[test]
    fn test_is_valid_2() {
        // Repeating patterns are invalid
        assert_eq!(is_valid_2(11), false);     // "1" repeated 2 times
        assert_eq!(is_valid_2(111), false);    // "1" repeated 3 times
        assert_eq!(is_valid_2(1212), false);   // "12" repeated 2 times
        assert_eq!(is_valid_2(121212), false); // "12" repeated 3 times
        assert_eq!(is_valid_2(123123), false); // "123" repeated 2 times
        assert_eq!(is_valid_2(9999), false);   // "9" repeated 4 times

        // Non-repeating patterns are valid
        assert_eq!(is_valid_2(12), true);      // no repeat
        assert_eq!(is_valid_2(123), true);     // no repeat
        assert_eq!(is_valid_2(1234), true);    // no repeat
        assert_eq!(is_valid_2(1211), true);    // not a repeating pattern
    }

    #[test]
    fn test_small_range() {
        assert_eq!(part1("11-22"), 33); // 11 + 22
        assert_eq!(part1("95-115"), 99); // just 99
        assert_eq!(part1("998-1012"), 1010); // just 1010
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 1227775554);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 4174379265);
    }
}
