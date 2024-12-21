use std::collections::HashSet;


pub fn part1(input: &str) -> u32 {
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // get x and y coordinates of all posistions = 0
    let mut positions = Vec::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == 0 {
                positions.push((x, y));
            }
        }
    }

    let mut count = 0;
    for (x, y) in positions {
        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
        let mut queue = Vec::new();
        queue.push((x, y));
        visited[y][x] = true;
        while !queue.is_empty() {
            let (x, y) = queue.remove(0);
            if grid[y][x] == 9 {
                count += 1;
                continue;
            }
            for (nx, ny) in get_valid_neighbours(x, y, &grid) {
                if !is_one_higher(x, y, nx, ny, &grid) {
                    continue;
                }
                if !visited[ny][nx] {
                    visited[ny][nx] = true;
                    queue.push((nx, ny));
                }
            }
        }
    }

    count
}
fn is_one_higher(x: usize, y: usize, nx: usize, ny: usize, grid: &Vec<Vec<u32>>) -> bool {
    grid[y][x] + 1 == grid[ny][nx]
}

fn get_valid_neighbours(x: usize, y: usize, grid: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let mut neighbours = Vec::new();
    if x > 0 {
        neighbours.push((x - 1, y));
    }
    if y > 0 {
        neighbours.push((x, y - 1));
    }
    if x < grid[0].len() - 1 {
        neighbours.push((x + 1, y));
    }
    if y < grid.len() - 1 {
        neighbours.push((x, y + 1));
    }
    neighbours
}

pub fn part2(input: &str) -> u32 {
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut positions = Vec::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == 0 {
                positions.push((x, y));
            }
        }
    }

    let mut unique_paths = HashSet::new();

    for (x, y) in positions {
        let mut queue = Vec::new();
        queue.push((x, y, vec![(x, y)]));
        while !queue.is_empty() {
            let (x, y, path) = queue.remove(0);
            if grid[y][x] == 9 {
                unique_paths.insert(path);
                continue;
            }
            for (nx, ny) in get_valid_neighbours(x, y, &grid) {
                if !is_one_higher(x, y, nx, ny, &grid) {
                    continue;
                }
                if !path.contains(&(nx, ny)) {
                    let mut new_path = path.clone();
                    new_path.push((nx, ny));
                    queue.push((nx, ny, new_path));
                }
            }
        }
    }

    unique_paths.len() as u32
}
