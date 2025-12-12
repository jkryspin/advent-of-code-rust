//! # Day 12: Shape Packing
//!
//! Determines how many containers can fit all their assigned shapes using
//! backtracking with orientation deduplication for efficiency.

use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Shape {
    cells: Vec<Vec<bool>>, // true = filled (#), false = empty (.)
}

impl Shape {
    fn from_lines(lines: &[&str]) -> Self {
        let cells = lines
            .iter()
            .map(|line| line.chars().map(|c| c == '#').collect())
            .collect();
        Shape { cells }
    }

    fn count_filled(&self) -> usize {
        self.cells.iter().flatten().filter(|&&c| c).count()
    }

    fn height(&self) -> usize {
        self.cells.len()
    }

    fn width(&self) -> usize {
        self.cells.first().map_or(0, |row| row.len())
    }

    fn rotate_90(&self) -> Shape {
        let h = self.height();
        let w = self.width();
        let mut cells = vec![vec![false; h]; w];
        for y in 0..h {
            for x in 0..w {
                cells[x][h - 1 - y] = self.cells[y][x];
            }
        }
        Shape { cells }
    }

    fn flip_horizontal(&self) -> Shape {
        Shape {
            cells: self.cells.iter()
                .map(|row| row.iter().rev().copied().collect())
                .collect(),
        }
    }

    fn all_orientations(&self) -> Vec<Shape> {
        let mut orientations = Vec::with_capacity(8);
        let mut current = self.clone();

        // Add 4 rotations
        for _ in 0..4 {
            orientations.push(current.clone());
            current = current.rotate_90();
        }

        // Add 4 rotations of flipped version
        current = self.flip_horizontal();
        for _ in 0..4 {
            orientations.push(current.clone());
            current = current.rotate_90();
        }

        orientations
    }
}

#[derive(Debug)]
struct Container {
    width: usize,
    height: usize,
    shapes: Vec<(usize, usize)>, // (shape_index, count)
}

fn parse_input(input: &str) -> (HashMap<usize, Shape>, Vec<Container>) {
    let mut shapes = HashMap::new();
    let mut containers = Vec::new();
    let lines: Vec<&str> = input.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].trim();

        if line.is_empty() {
            i += 1;
        } else if line.ends_with(':') && !line.contains('x') {
            // Shape definition: "N:"
            let idx = line.trim_end_matches(':').parse().unwrap();
            i += 1;

            let mut shape_lines = Vec::new();
            while i < lines.len() && !lines[i].trim().is_empty()
                  && !lines[i].contains('x') && !lines[i].ends_with(':') {
                shape_lines.push(lines[i]);
                i += 1;
            }
            shapes.insert(idx, Shape::from_lines(&shape_lines));
        } else if line.contains('x') && line.contains(':') {
            // Container definition: "WxH: count0 count1 ..."
            let (dims, counts) = line.split_once(':').unwrap();
            let (w, h) = dims.split_once('x').unwrap();

            let shape_counts: Vec<(usize, usize)> = counts
                .split_whitespace()
                .enumerate()
                .map(|(idx, s)| (idx, s.parse().unwrap()))
                .collect();

            containers.push(Container {
                width: w.parse().unwrap(),
                height: h.parse().unwrap(),
                shapes: shape_counts,
            });
            i += 1;
        } else {
            i += 1;
        }
    }

    (shapes, containers)
}

fn can_fit_shapes(container: &Container, shapes: &HashMap<usize, Shape>) -> bool {
    // Quick check: total filled cells must fit in container area
    let total_filled: usize = container.shapes.iter()
        .filter_map(|&(idx, count)| shapes.get(&idx).map(|s| s.count_filled() * count))
        .sum();

    if total_filled > container.width * container.height {
        return false;
    }

    // Expand shape counts into a flat list
    let shape_list: Vec<usize> = container.shapes.iter()
        .flat_map(|&(idx, count)| vec![idx; count])
        .collect();

    // Try backtracking placement
    let mut grid = vec![vec![false; container.width]; container.height];
    solve_placement(&shape_list, shapes, &mut grid, container.width, container.height, 0)
}

fn solve_placement(
    shape_indices: &[usize],
    shapes: &HashMap<usize, Shape>,
    grid: &mut Vec<Vec<bool>>,
    width: usize,
    height: usize,
    shape_pos: usize,
) -> bool {
    if shape_pos >= shape_indices.len() {
        return true; // All shapes successfully placed
    }

    let Some(shape) = shapes.get(&shape_indices[shape_pos]) else {
        return false;
    };

    // Deduplicate orientations (many shapes have symmetry)
    let mut tried = std::collections::HashSet::new();

    for orientation in shape.all_orientations() {
        // Skip if we've already tried this exact pattern
        if !tried.insert(format!("{:?}", orientation.cells)) {
            continue;
        }

        // Try all valid positions for this orientation
        let max_y = height.saturating_sub(orientation.height());
        let max_x = width.saturating_sub(orientation.width());

        for y in 0..=max_y {
            for x in 0..=max_x {
                if can_place_shape(&orientation, grid, x, y, width, height) {
                    place_shape(&orientation, grid, x, y, true);

                    if solve_placement(shape_indices, shapes, grid, width, height, shape_pos + 1) {
                        return true;
                    }

                    place_shape(&orientation, grid, x, y, false); // Backtrack
                }
            }
        }
    }

    false
}

fn can_place_shape(
    shape: &Shape,
    grid: &[Vec<bool>],
    x: usize,
    y: usize,
    width: usize,
    height: usize,
) -> bool {
    let shape_height = shape.height();
    let shape_width = shape.width();

    // Check bounds
    if x + shape_width > width || y + shape_height > height {
        return false;
    }

    // Check for overlaps
    for sy in 0..shape_height {
        for sx in 0..shape_width {
            if shape.cells[sy][sx] && grid[y + sy][x + sx] {
                return false;
            }
        }
    }

    true
}

fn place_shape(
    shape: &Shape,
    grid: &mut [Vec<bool>],
    x: usize,
    y: usize,
    place: bool,
) {
    let shape_height = shape.height();
    let shape_width = shape.width();

    for sy in 0..shape_height {
        for sx in 0..shape_width {
            if shape.cells[sy][sx] {
                grid[y + sy][x + sx] = place;
            }
        }
    }
}

pub fn part1(input: &str) -> usize {
    let (shapes, containers) = parse_input(input);

    let mut count = 0;
    for container in &containers {
        if can_fit_shapes(container, &shapes) {
            count += 1;
        }
    }

    count
}

pub fn part2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 0);
    }
}
