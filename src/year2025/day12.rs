//! # Day 12: [Title]

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
        self.cells
            .iter()
            .flat_map(|row| row.iter())
            .filter(|&&cell| cell)
            .count()
    }

    fn width(&self) -> usize {
        self.cells.first().map_or(0, |row| row.len())
    }

    fn height(&self) -> usize {
        self.cells.len()
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
        let cells = self
            .cells
            .iter()
            .map(|row| row.iter().rev().copied().collect())
            .collect();
        Shape { cells }
    }

    fn all_orientations(&self) -> Vec<Shape> {
        let mut orientations = Vec::new();
        let mut current = self.clone();

        // 4 rotations
        for _ in 0..4 {
            orientations.push(current.clone());
            current = current.rotate_90();
        }

        // 4 rotations of flipped version
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
            continue;
        }

        // Check if it's a shape definition (ends with ':' and is just a number)
        if line.ends_with(':') && !line.contains('x') {
            let idx = line.trim_end_matches(':').parse::<usize>().unwrap();
            i += 1;

            // Collect shape lines until we hit an empty line or container line
            let mut shape_lines = Vec::new();
            while i < lines.len() {
                let shape_line = lines[i];
                if shape_line.trim().is_empty() || shape_line.contains('x') || shape_line.ends_with(':') {
                    break;
                }
                shape_lines.push(shape_line);
                i += 1;
            }

            shapes.insert(idx, Shape::from_lines(&shape_lines));
        }
        // Check if it's a container definition (contains 'x')
        else if line.contains('x') && line.contains(':') {
            let parts: Vec<&str> = line.split(':').collect();
            let dims: Vec<&str> = parts[0].split('x').collect();
            let width = dims[0].parse::<usize>().unwrap();
            let height = dims[1].parse::<usize>().unwrap();
            let counts: Vec<usize> = parts[1]
                .split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect();

            // Convert counts to (shape_index, count) pairs
            let shapes: Vec<(usize, usize)> = counts
                .iter()
                .enumerate()
                .map(|(idx, &count)| (idx, count))
                .collect();

            containers.push(Container {
                width,
                height,
                shapes,
            });
            i += 1;
        } else {
            i += 1;
        }
    }

    (shapes, containers)
}

fn shapes_form_rectangle(shape1: &Shape, shape2: &Shape) -> Option<(usize, usize)> {
    // Try all orientations of both shapes to see if they form a rectangle
    for s1 in shape1.all_orientations() {
        for s2 in shape2.all_orientations() {
            // Try placing s2 in all 4 directions relative to s1
            let h1 = s1.height();
            let w1 = s1.width();
            let h2 = s2.height();
            let w2 = s2.width();

            // Try horizontally adjacent
            if h1 == h2 {
                let total_width = w1 + w2;
                let mut combined = vec![vec![false; total_width]; h1];

                // Place s1 on left
                for y in 0..h1 {
                    for x in 0..w1 {
                        combined[y][x] = s1.cells[y][x];
                    }
                }
                // Place s2 on right
                for y in 0..h2 {
                    for x in 0..w2 {
                        combined[y][w1 + x] = s2.cells[y][x];
                    }
                }

                // Check if fully filled
                if combined.iter().all(|row| row.iter().all(|&cell| cell)) {
                    return Some((total_width, h1));
                }
            }

            // Try vertically adjacent
            if w1 == w2 {
                let total_height = h1 + h2;
                let mut combined = vec![vec![false; w1]; total_height];

                // Place s1 on top
                for y in 0..h1 {
                    for x in 0..w1 {
                        combined[y][x] = s1.cells[y][x];
                    }
                }
                // Place s2 on bottom
                for y in 0..h2 {
                    for x in 0..w2 {
                        combined[h1 + y][x] = s2.cells[y][x];
                    }
                }

                // Check if fully filled
                if combined.iter().all(|row| row.iter().all(|&cell| cell)) {
                    return Some((w1, total_height));
                }
            }
        }
    }
    None
}

fn can_fit_shapes(
    container: &Container,
    shapes: &HashMap<usize, Shape>,
) -> bool {
    // First check: total filled cells
    let mut total_filled = 0;
    for &(shape_idx, count) in &container.shapes {
        if let Some(shape) = shapes.get(&shape_idx) {
            total_filled += shape.count_filled() * count;
        }
    }

    let container_area = container.width * container.height;

    if total_filled > container_area {
        return false;
    }

    // Check if pairs of shapes form rectangles and can tile perfectly
    if total_filled == container_area {
        let num_shape_types = container.shapes.len();
        for i in 0..num_shape_types {
            for j in i+1..num_shape_types {
                let (idx1, count1) = container.shapes[i];
                let (idx2, count2) = container.shapes[j];

                if let (Some(s1), Some(s2)) = (shapes.get(&idx1), shapes.get(&idx2)) {
                    if let Some((rect_w, rect_h)) = shapes_form_rectangle(s1, s2) {
                        // These two shapes form a rectangle!
                        let pairs_available = count1.min(count2);

                        // Check if using all pairs of these shapes fills the container exactly
                        let total_used = pairs_available * 2;
                        let total_shapes: usize = container.shapes.iter().map(|(_, c)| c).sum();

                        // Simple check: can we tile the container with these rectangles?
                        if container.width % rect_w == 0 && container.height % rect_h == 0 {
                            let rects_needed = (container.width / rect_w) * (container.height / rect_h);
                            if rects_needed == pairs_available && total_used == total_shapes {
                                return true;
                            }
                        }
                        if container.width % rect_h == 0 && container.height % rect_w == 0 {
                            let rects_needed = (container.width / rect_h) * (container.height / rect_w);
                            if rects_needed == pairs_available && total_used == total_shapes {
                                return true;
                            }
                        }
                    }
                }
            }
        }
    }

    // Fall back to backtracking (for small cases or when rectangle optimization doesn't apply)
    let mut shape_list = Vec::new();
    for &(shape_idx, count) in &container.shapes {
        for _ in 0..count {
            shape_list.push(shape_idx);
        }
    }

    // Try backtracking with a reasonable limit
    if shape_list.len() <= 300 {
        let mut grid = vec![vec![false; container.width]; container.height];
        return solve_placement(&shape_list, shapes, &mut grid, container.width, container.height, 0);
    }

    false
}

fn solve_placement(
    shape_indices: &[usize],
    shapes: &HashMap<usize, Shape>,
    grid: &mut Vec<Vec<bool>>,
    width: usize,
    height: usize,
    shape_pos: usize,
) -> bool {
    // Base case: all shapes placed
    if shape_pos >= shape_indices.len() {
        return true;
    }

    let shape_idx = shape_indices[shape_pos];
    let Some(shape) = shapes.get(&shape_idx) else {
        return false;
    };

    // Try all orientations (but use a smaller set - remove duplicates)
    let orientations = shape.all_orientations();
    let mut tried_orientations = std::collections::HashSet::new();

    for oriented_shape in &orientations {
        // Create a hash of the shape to avoid trying identical orientations
        let shape_hash: Vec<Vec<bool>> = oriented_shape.cells.clone();
        if !tried_orientations.insert(format!("{:?}", shape_hash)) {
            continue;
        }

        // Try positions in a more limited way - scan from top-left
        for y in 0..=height.saturating_sub(oriented_shape.height()) {
            for x in 0..=width.saturating_sub(oriented_shape.width()) {
                if can_place_shape(oriented_shape, grid, x, y, width, height) {
                    // Place the shape
                    place_shape(oriented_shape, grid, x, y, true);

                    // Recursively try to place remaining shapes
                    if solve_placement(shape_indices, shapes, grid, width, height, shape_pos + 1) {
                        return true;
                    }

                    // Backtrack
                    place_shape(oriented_shape, grid, x, y, false);
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
