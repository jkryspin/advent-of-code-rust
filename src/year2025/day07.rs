//! # Day 7: [Title]

use std::collections::HashMap;

pub fn part1(input: &str) -> usize {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let height = grid.len();
    let width = grid[0].len();

    // Find starting position 'S'
    let mut start_pos = (0, 0);
    for (row, line) in grid.iter().enumerate() {
        for (col, &ch) in line.iter().enumerate() {
            if ch == 'S' {
                start_pos = (row, col);
                break;
            }
        }
    }

    let mut memo = HashMap::new();
    let split_count = process_beam(&mut grid, start_pos.0 + 1, start_pos.1, height, width, &mut memo);

    // Print the final grid
    for row in &grid {
        println!("{}", row.iter().collect::<String>());
    }

    split_count
}

fn process_beam(
    grid: &mut Vec<Vec<char>>,
    start_row: usize,
    col: usize,
    height: usize,
    width: usize,
    memo: &mut HashMap<(usize, usize), usize>,
) -> usize {
    let mut row = start_row;
    let mut split_count = 0;

    while row < height {
        let current = grid[row][col];

        match current {
            '.' => {
                // Convert to beam
                grid[row][col] = '|';
                row += 1;
            }
            '|' => {
                // Already a beam, check if we've cached results from this position
                if let Some(&cached) = memo.get(&(row, col)) {
                    split_count += cached;
                }
                break;
            }
            '^' => {
                // Hit a splitter, create beams to left and right
                split_count += 1;

                // Process left beam
                if col > 0 {
                    split_count += process_beam(grid, row, col - 1, height, width, memo);
                }

                // Process right beam
                if col < width - 1 {
                    split_count += process_beam(grid, row, col + 1, height, width, memo);
                }

                // Cache result from this splitter position
                memo.insert((row, col), split_count - 1); // -1 because we don't count the splitter itself in cache
                break;
            }
            _ => {
                // Hit something else (like 'S'), stop
                break;
            }
        }
    }

    split_count
}

pub fn part2(input: &str) -> usize {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let height = grid.len();
    let width = grid[0].len();

    // Find starting position 'S'
    let mut start_pos = (0, 0);
    for (row, line) in grid.iter().enumerate() {
        for (col, &ch) in line.iter().enumerate() {
            if ch == 'S' {
                start_pos = (row, col);
                break;
            }
        }
    }

    let mut memo = HashMap::new();
    let bottom_hits = process_beam_p2(&mut grid, start_pos.0 + 1, start_pos.1, height, width, &mut memo);

    // Print the final grid
    for row in &grid {
        println!("{}", row.iter().collect::<String>());
    }

    bottom_hits
}

fn process_beam_p2(
    grid: &mut Vec<Vec<char>>,
    start_row: usize,
    col: usize,
    height: usize,
    width: usize,
    memo: &mut HashMap<(usize, usize), usize>,
) -> usize {
    // Check memo at entry for this position
    if let Some(&cached) = memo.get(&(start_row, col)) {
        return cached;
    }

    let mut row = start_row;
    let mut bottom_hits = 0;

    while row < height {
        let current = grid[row][col];

        // Check if we've reached the bottom row
        if row == height - 1 {
            bottom_hits += 1;
            if current == '.' {
                grid[row][col] = '|';
            }
            break;
        }

        match current {
            '.' => {
                // Convert to beam
                grid[row][col] = '|';
                row += 1;
            }
            '|' => {
                // Already a beam, check if next position is cached
                if let Some(&cached) = memo.get(&(row + 1, col)) {
                    bottom_hits += cached;
                    break;
                }
                row += 1;
            }
            '^' => {
                // Hit a splitter, create beams to left and right
                let mut splitter_hits = 0;

                // Process left beam
                if col > 0 {
                    splitter_hits += process_beam_p2(grid, row, col - 1, height, width, memo);
                }

                // Process right beam
                if col < width - 1 {
                    splitter_hits += process_beam_p2(grid, row, col + 1, height, width, memo);
                }

                bottom_hits += splitter_hits;
                break;
            }
            _ => {
                // Hit something else (like 'S'), stop
                break;
            }
        }
    }

    // Cache the result for this starting position
    memo.insert((start_row, col), bottom_hits);
    bottom_hits
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 40);
    }
}
