//! # Day 5: [Title]

pub fn part1(input: &str) -> usize {
    let sections: Vec<&str> = input.split("\n\n").collect();

    if sections.len() != 2 {
        return 0;
    }

    // Parse ranges
    let ranges: Vec<(usize, usize)> = sections[0]
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let parts: Vec<&str> = line.split('-').collect();
            let start = parts[0].parse::<usize>().unwrap();
            let end = parts[1].parse::<usize>().unwrap();
            (start, end)
        })
        .collect();

    // Parse ingredient IDs
    let ingredient_ids: Vec<usize> = sections[1]
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<usize>().unwrap())
        .collect();

    // Count fresh ingredients (those in at least one range)
    let mut fresh_count = 0;
    for id in ingredient_ids {
        let is_fresh = ranges.iter().any(|(start, end)| id >= *start && id <= *end);
        if is_fresh {
            fresh_count += 1;
        }
    }

    fresh_count
}

pub fn part2(input: &str) -> usize {
    let sections: Vec<&str> = input.split("\n\n").collect();

    if sections.is_empty() {
        return 0;
    }

    // Parse ranges
    let mut ranges: Vec<(usize, usize)> = sections[0]
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let parts: Vec<&str> = line.split('-').collect();
            let start = parts[0].parse::<usize>().unwrap();
            let end = parts[1].parse::<usize>().unwrap();
            (start, end)
        })
        .collect();

    if ranges.is_empty() {
        return 0;
    }

    // Sort ranges by start position
    ranges.as_mut_slice().sort_by_key(|r| r.0);


    // Merge overlapping ranges and count total size
    let mut merged: Vec<(usize, usize)> = vec![ranges[0]];

    for i in 1..ranges.len() {
        let last_idx = merged.len() - 1;
        let (last_start, last_end) = merged[last_idx];
        let (curr_start, curr_end) = ranges[i];

        // If current range overlaps or is adjacent to last merged range
        if curr_start <= last_end + 1 {
            // Merge by extending the end
            merged[last_idx] = (last_start, std::cmp::max(last_end, curr_end));
        } else {
            // No overlap, add as new range
            merged.push((curr_start, curr_end));
        }
    }

    // Calculate total size of merged ranges
    merged.iter()
        .map(|(start, end)| end - start + 1)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 14);
    }
}
