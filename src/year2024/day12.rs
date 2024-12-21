use std::collections::HashMap;


pub fn part1(input: &str) -> u32 {
    solve(input, true)
}

fn solve(input: &str, part: bool) -> u32 {
    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let regions = get_regions(&grid);
    let mut total = 0;

    for ((_cell, _start_x, _start_y), region) in regions.iter() {
        let mut perimeter = 0;
        for (x, y) in region.iter() {
            let mut sides = 4;
            for (nx, ny) in find_neighbors(*x as usize, *y as usize, &grid) {
                if !region.contains(&(nx as u32, ny as u32)) {
                    perimeter += 1;
                }
                sides -= 1;
            }
            perimeter += sides;
        }
        let region_i32 = region
            .iter()
            .map(|(x, y)| (*x as i32, *y as i32))
            .collect::<Vec<_>>();
        let sides = calculate_region_sides(&region_i32);
        if part {
            total += region.len() as u32 * perimeter as u32;
        } else {
            total += region.len() as u32 * sides as u32;
        }
    }

    total
}

fn get_regions(grid: &Vec<Vec<char>>) -> HashMap<(char, usize, usize), Vec<(u32, u32)>> {
    let mut regions: HashMap<(char, usize, usize), Vec<(u32, u32)>> = HashMap::new();
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];

    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if visited[y][x] {
                continue;
            }
            let mut stack = vec![(x, y)];
            let mut region = Vec::new();
            while let Some((x, y)) = stack.pop() {
                if region.contains(&(x as u32, y as u32)) {
                    continue;
                }
                visited[y][x] = true;
                region.push((x as u32, y as u32));
                for (nx, ny) in find_neighbors(x, y, &grid) {
                    if grid[ny][nx] == *cell {
                        stack.push((nx, ny));
                    }
                }
            }
            for (x_r, y_r) in region.iter() {
                regions
                    .entry((*cell, x, y))
                    .or_insert(Vec::new())
                    .push((*x_r, *y_r));
            }
        }
    }

    regions
}

fn calculate_region_sides(region: &Vec<(i32, i32)>) -> u32 {
    region.iter().map(|&(x, y)| is_corner(x, y, region)).sum()
}

fn is_corner(x: i32, y: i32, region: &Vec<(i32, i32)>) -> u32 {
    let dirs = vec![
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ];
    let mut corners = 0;
    for dir in dirs.iter() {
        // if the dir has a block, it cant be a corner
        let corner = match dir {
            Direction::Up => !region.contains(&(x, y - 1)),
            Direction::Down => !region.contains(&(x, y + 1)),
            Direction::Left => !region.contains(&(x - 1, y)),
            Direction::Right => !region.contains(&(x + 1, y)),
        };
        if !corner {
            continue;
        }

        // if the rotation clockwise has a block, it cant be a corner
        let corner1 = match dir {
            Direction::Up => !region.contains(&(x + 1, y)),
            Direction::Down => !region.contains(&(x - 1, y)),
            Direction::Left => !region.contains(&(x, y - 1)),
            Direction::Right => !region.contains(&(x, y + 1)),
        };

        // but if the diagonal has a block it can be an interior corner still
        let corner2 = match dir {
            Direction::Up => region.contains(&(x + 1, y - 1)),
            Direction::Down => region.contains(&(x - 1, y + 1)),
            Direction::Left => region.contains(&(x - 1, y - 1)),
            Direction::Right => region.contains(&(x + 1, y + 1)),
        };

        if corner1 || corner2 {
            corners += 1;
        }
    }
    corners
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn find_neighbors(x: usize, y: usize, grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    if x > 0 {
        neighbors.push((x - 1, y));
    }
    if y > 0 {
        neighbors.push((x, y - 1));
    }
    if x < grid[0].len() - 1 {
        neighbors.push((x + 1, y));
    }
    if y < grid.len() - 1 {
        neighbors.push((x, y + 1));
    }
    neighbors
}

pub fn part2(input: &str) -> u32 {
    solve(input, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part2() {
        let result = part2(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }

    #[test]
    fn test_part2_a() {
        let result = part2("AAAA\nBBCD\nBBCC\nEEEC");
        assert_eq!(result, Some(80));
    }
}
