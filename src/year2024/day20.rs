use std::collections::{HashSet, VecDeque};

pub fn part1(input: &str) -> u32 {
    solve(input, 2).unwrap()
}

pub fn part2(input: &str) -> u32 {
    solve(input, 20).unwrap()
}

fn solve(input: &str, max_len: usize) -> Option<u32> {
    let solver = Solver::from(input);
    let start = find_position(&solver.grid, 'S')?;
    let end = find_position(&solver.grid, 'E')?;
    let costs = bfs(&solver.grid, start, end)?;
    let moves_saved = find_moves_saved(&costs, max_len);
    let costs_calculated = calculate_costs(&costs, &moves_saved);
    Some(costs_calculated.iter().filter(|&&x| x >= 100).count() as u32)
}

struct Solver {
    grid: Vec<Vec<char>>,
}

impl From<&str> for Solver {
    fn from(s: &str) -> Self {
        let grid: Vec<Vec<char>> = s.lines().map(|l| l.chars().collect()).collect();
        Self { grid }
    }
}

fn find_position(grid: &[Vec<char>], target: char) -> Option<(usize, usize)> {
    grid.iter()
        .enumerate()
        .find_map(|(y, row)| row.iter().position(|&c| c == target).map(|x| (x, y)))
}

fn bfs(grid: &[Vec<char>], start: (usize, usize), end: (usize, usize)) -> Option<Vec<Vec<u32>>> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((start, 1));
    visited.insert(start);
    let mut costs = vec![vec![0; grid[0].len()]; grid.len()];
    while let Some((pos, steps)) = queue.pop_front() {
        let (x, y) = pos;
        costs[y][x] = steps;
        if pos == end {
            return Some(costs);
        }
        for (dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let (nx, ny) = (x as i32 + dx, y as i32 + dy);
            if nx < 0 || ny < 0 || nx >= grid[0].len() as i32 || ny >= grid.len() as i32 {
                continue;
            }
            let (nx, ny) = (nx as usize, ny as usize);
            if grid[ny][nx] == '#' || visited.contains(&(nx, ny)) {
                continue;
            }
            visited.insert((nx, ny));
            queue.push_back(((nx, ny), steps + 1));
        }
    }
    None
}

fn find_moves_saved(costs: &[Vec<u32>], max_len: usize) -> Vec<((usize, usize), (usize, usize), usize)> {
    let mut moves_saved = vec![];
    for y in 0..costs.len() {
        for x in 0..costs[0].len() {
            if costs[y][x] != 0 {
                if let Some(moves) = bfs_with_cap(costs, (x, y), max_len) {
                    moves_saved.extend(moves);
                }
            }
        }
    }
    moves_saved.sort();
    moves_saved.dedup();
    moves_saved
}

fn bfs_with_cap(grid: &[Vec<u32>], start: (usize, usize), max_len: usize) -> Option<Vec<((usize, usize), (usize, usize), usize)>> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((start, vec![start]));
    visited.insert(start);
    let mut moves_saved = vec![];
    while let Some((pos, steps)) = queue.pop_front() {
        let (x, y) = pos;
        if steps.len() - 1 >= 2 && steps.len() - 1 <= max_len && grid[y][x] != 0 {
            if steps.iter().any(|&(x, y)| grid[y][x] == 0) {
                moves_saved.push((start, pos, steps.len() - 1));
            }
        }
        if steps.len() - 1 > max_len {
            continue;
        }
        for (dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let (nx, ny) = (x as i32 + dx, y as i32 + dy);
            if nx < 0 || ny < 0 || nx >= grid[0].len() as i32 || ny >= grid.len() as i32 {
                continue;
            }
            let (nx, ny) = (nx as usize, ny as usize);
            if visited.contains(&(nx, ny)) {
                continue;
            }
            visited.insert((nx, ny));
            let mut new_steps = steps.clone();
            new_steps.push((nx, ny));
            queue.push_back(((nx, ny), new_steps));
        }
    }
    Some(moves_saved)
}

fn calculate_costs(costs: &[Vec<u32>], moves_saved: &[((usize, usize), (usize, usize), usize)]) -> Vec<u32> {
    moves_saved.iter().filter_map(|&(start, end, steps)| {
        let starting_cost = costs[start.1][start.0];
        let ending_cost = costs[end.1][end.0];
        if starting_cost > ending_cost {
            return None;
        }
        let steps_saved = ending_cost as i32 - starting_cost as i32 - steps as i32;
        Some(steps_saved as u32)
    }).collect()
}