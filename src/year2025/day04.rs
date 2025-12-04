//! # Day 4: [Title]

pub fn part1(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input
        .trim()  // Remove trailing newline
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    if grid.is_empty() {
        return 0;
    }

    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    // Check all 8 surrounding directions
    let directions = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1),  (1, 0),  (1, 1),
    ];

    for row in 0..rows {
        for col in 0..cols {
            // Only check @ cells
            if grid[row][col] != '@' {
                continue;
            }

            let mut at_sign_neighbors = 0;

            // Count surrounding @ signs
            for (dr, dc) in directions.iter() {
                let new_row = row as i32 + dr;
                let new_col = col as i32 + dc;

                // Check bounds
                if new_row >= 0 && new_row < rows as i32
                    && new_col >= 0 && new_col < cols as i32 {
                    if grid[new_row as usize][new_col as usize] == '@' {
                        at_sign_neighbors += 1;
                    }
                }
            }

            // If fewer than 4 @ neighbors, increment count
            if at_sign_neighbors < 4 {
                count += 1;
            }
        }
    }

    count
}

pub fn part2(input: &str) -> usize {
    let mut grid: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    if grid.is_empty() {
        return 0;
    }

    let rows = grid.len();
    let cols = grid[0].len();
    let mut total_count = 0;

    let directions = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1),  (1, 0),  (1, 1),
    ];

    loop {
        let mut to_remove = Vec::new();

        // Find all @ signs with fewer than 4 @ neighbors
        for row in 0..rows {
            for col in 0..cols {
                if grid[row][col] != '@' {
                    continue;
                }

                let mut at_sign_neighbors = 0;

                for (dr, dc) in directions.iter() {
                    let new_row = row as i32 + dr;
                    let new_col = col as i32 + dc;

                    if new_row >= 0 && new_row < rows as i32
                        && new_col >= 0 && new_col < cols as i32 {
                        if grid[new_row as usize][new_col as usize] == '@' {
                            at_sign_neighbors += 1;
                        }
                    }
                }

                if at_sign_neighbors < 4 {
                    to_remove.push((row, col));
                }
            }
        }

        // If no @ signs to remove, we're done
        if to_remove.is_empty() {
            break;
        }

        // Mark them as '.' and count them
        for (row, col) in to_remove {
            grid[row][col] = '.';
            total_count += 1;
        }
    }

    total_count
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 43);
    }
}
