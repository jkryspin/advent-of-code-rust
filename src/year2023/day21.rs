use std::collections::{HashMap, HashSet};


pub fn part1(input: &str) -> usize {
    let mut sol = Solution::new(input);
    for _ in 0..64 {
        sol.simulate();
    }

    sol.count()
}

pub fn part2(input: &str) -> usize {
    let mut sol = Solution::new(input);
    let mut count: usize = 0;
    for _ in 0..6 {
        count += sol.simulate_2() as usize;
    }

    count
}

struct Solution {
    grid: Vec<Vec<char>>,
    positions: HashSet<(usize, usize)>,
    positions_2: HashMap<(usize, usize), usize>,
}
impl Solution {
    fn new(input: &str) -> Self {
        let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        let start = grid
            .iter()
            .enumerate()
            .find_map(|(y, row)| row.iter().position(|&c| c == 'S').map(|x| (x, y)))
            .unwrap();
        let mut map = HashMap::new();
        map.insert(start, 1);

        Self {
            grid,
            positions: vec![start].into_iter().collect(),
            positions_2: map,
        }
    }

    fn simulate(&mut self) {
        let mut new_positions = HashSet::new();
        for (x, y) in &self.positions {
            let x = *x as i32;
            let y = *y as i32;
            // for each dir
            for (dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let (nx, ny) = (x + dx, y + dy);
                if nx < 0 || ny < 0 {
                    continue;
                }
                if nx >= self.grid[0].len() as i32 || ny >= self.grid.len() as i32 {
                    continue;
                }

                if self.grid[ny as usize][nx as usize] == '#' {
                    continue;
                }

                new_positions.insert((nx as usize, ny as usize));
            }
        }
        self.positions = new_positions;
    }

    fn simulate_2(&mut self) -> u32 {
        let mut seen = HashSet::new();
        let mut new_positions = HashMap::new();
        let mut count = 0;
        for ((x, y), z) in &self.positions_2 {
            let x = *x as i32;
            let y = *y as i32;
            // for each dir
            for (dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let (nx, ny) = (x + dx, y + dy);
                // if nx or ny out of bounds, wrap to other side
                let projected_nx = (nx) % self.grid[0].len() as i32;
                let projected_ny = (ny) % self.grid.len() as i32;

                if self.grid[projected_ny as usize][projected_nx as usize] == '#' {
                    continue;
                }

                *new_positions
                    .entry((projected_nx as usize, projected_ny as usize))
                    .or_insert(0) += z;
                if (!seen.contains(&(projected_nx as usize, projected_ny as usize))) {
                    count += 1;
                }
                seen.insert((projected_nx as usize, projected_ny as usize));
            }
        }
        self.positions_2 = new_positions;
        count
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
