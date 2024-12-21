use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet};


pub fn part1(input: &str) -> u32 {
    let mut grid = Grid::new(input);
    let (x, y) = grid.get_starting_point();
    grid.simulate(x, y, Direction::Up);
    grid.count_visited() as u32
}

pub fn part2(input: &str) -> u32 {
    let mut grid = Grid::new(input);
    let (x_s, y_s) = grid.get_starting_point();
    // add a # to each position and simulate
    let mut count = 0;

    // Brute force every possible blocker
    for y in 0..grid.grid.len() {
        for x in 0..grid.grid[y].len() {
            if grid.grid[y][x] == '^' {
                continue;
            }
            let before = grid.grid[y][x];
            grid.grid[y][x] = '#';
            if grid.simulate(x_s, y_s, Direction::Up) {
                count += 1;
            }
            grid.grid[y][x] = before;
        }
    }
    count
}

#[derive(Clone)]
struct Grid {
    grid: Vec<Vec<char>>,
    visited: Vec<Vec<bool>>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        let visited = vec![vec![false; grid[0].len()]; grid.len()];
        Self { grid, visited }
    }

    fn count_visited(&self) -> usize {
        self.visited
            .iter()
            .map(|row| row.iter().filter(|&&v| v).count())
            .sum()
    }

    fn simulate(&mut self, x: usize, y: usize, mut direction: Direction) -> bool {
        let mut x = x;
        let mut y = y;
        let mut visited_with_dir: HashMap<(usize, usize), HashSet<Direction>> = HashMap::new();
        let mut recently_collided = false;
        loop {
            self.visited[y][x] = true;
            if recently_collided {
                if let Some(dir) = visited_with_dir.get(&(x, y)) {
                    if dir.contains(&direction) {
                        return true;
                    }
                }

                //insert current position and dir, with dir being a hashset of dirs
                visited_with_dir
                    .entry((x, y))
                    .or_insert_with(|| HashSet::with_capacity(4))
                    .insert(direction);
            }

            let prev = (x, y);
            match direction {
                Direction::Up => {
                    if y == 0 {
                        return false;
                    }
                    y -= 1;
                }
                Direction::Down => {
                    if y == self.grid.len() - 1 {
                        return false;
                    }
                    y += 1;
                }
                Direction::Left => {
                    if x == 0 {
                        return false;
                    }
                    x -= 1;
                }
                Direction::Right => {
                    if x == self.grid[y].len() - 1 {
                        return false;
                    }
                    x += 1;
                }
            }

            recently_collided = false;

            if self.grid[y][x] == '#' {
                (x, y) = prev;
                recently_collided = true;
                direction = Grid::get_next_dir(direction);
            }
        }
    }

    fn get_next_dir(direction: Direction) -> Direction {
        match direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn get_starting_point(&self) -> (usize, usize) {
        // Find the starting point where the char is ^
        for (y, row) in self.grid.iter().enumerate() {
            for (x, &c) in row.iter().enumerate() {
                if c == '^' {
                    return (x, y);
                }
            }
        }
        panic!("No starting point found");
    }
}

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
