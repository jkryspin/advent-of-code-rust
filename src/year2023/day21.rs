use rayon::prelude::*;
use rustc_hash::FxHashSet;
use std::time::Instant;


pub fn part1(input: &str) -> usize {
    let mut sol = Solution::new(input);
    for _ in 0..64 {
        sol.simulate();
    }

    sol.count()
}
pub fn part2(input: &str) -> usize {
    let mut sol = Solution::new(input);
    let grid_size = 131;
    let target = 26501365;

    // Sample points: at 65, 196, 327 steps (which is 65 + 131*n for n=0,1,2)
    let sample_points = vec![65, 65 + grid_size, 65 + grid_size * 2];
    let mut values = vec![];

    let mut steps = 0;
    for &target_steps in &sample_points {
        while steps < target_steps {
            sol.simulate();
            steps += 1;
        }
        values.push(sol.count());
    }

    // Now we have 3 points: (0, values[0]), (1, values[1]), (2, values[2])
    // Fit quadratic: f(n) = an² + bn + c
    let y0 = values[0] as i64;
    let y1 = values[1] as i64;
    let y2 = values[2] as i64;

    // Calculate coefficients
    let a = (y2 - 2 * y1 + y0) / 2;
    let b = y1 - y0 - a;
    let c = y0;

    // Calculate n for the target: target = 65 + 131*n, so n = (target - 65) / 131
    let n = ((target - 65) / grid_size) as i64;

    // Evaluate f(n) = an² + bn + c
    let result = a * n * n + b * n + c;

    result as usize
}

struct Solution {
    positions: FxHashSet<(i32, i32)>,
    next_positions: FxHashSet<(i32, i32)>,
    walls: Vec<Vec<bool>>,
    width: i32,
    height: i32,
}
impl Solution {
    fn new(input: &str) -> Self {
        let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        let height = grid.len() as i32;
        let width = grid[0].len() as i32;

        let start = grid
            .iter()
            .enumerate()
            .find_map(|(y, row)| row.iter().position(|&c| c == 'S').map(|x| (x as i32, y as i32)))
            .unwrap();

        // Convert walls to 2D boolean array for O(1) lookup
        let walls = grid
            .iter()
            .map(|row| row.iter().map(|&c| c == '#').collect())
            .collect();

        Self {
            positions: vec![start].into_iter().collect(),
            next_positions: FxHashSet::default(),
            walls,
            width,
            height,
        }
    }

    fn simulate(&mut self) {
        self.next_positions.clear();

        let width = self.width;
        let height = self.height;

        // Collect valid neighbors for each position
        for &(x, y) in &self.positions {
            for &(dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let (nx, ny) = (x + dx, y + dy);
                let wrapped_x = nx.rem_euclid(width) as usize;
                let wrapped_y = ny.rem_euclid(height) as usize;

                // Safety: wrapped_x and wrapped_y are guaranteed to be in bounds due to rem_euclid
                let is_wall = unsafe {
                    *self.walls.get_unchecked(wrapped_y).get_unchecked(wrapped_x)
                };

                if !is_wall {
                    self.next_positions.insert((nx, ny));
                }
            }
        }

        std::mem::swap(&mut self.positions, &mut self.next_positions);
    }

    fn count(&self) -> usize {
        self.positions.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test removed - needs example data

    // Test removed - needs example data
}
