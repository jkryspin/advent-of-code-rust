//! # Day 8: Playground

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn distance_squared(&self, other: &Point) -> i64 {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)
    }
}

fn parse_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<i64> = line.split(',').map(|s| s.parse().unwrap()).collect();
            Point {
                x: parts[0],
                y: parts[1],
                z: parts[2],
            }
        })
        .collect()
}

/// Simple component tracking - each junction box has a component ID
struct Components {
    component_id: Vec<usize>,
}

impl Components {
    fn new(n: usize) -> Self {
        // Initially, each junction box is in its own component
        Components {
            component_id: (0..n).collect(),
        }
    }

    fn same_component(&self, i: usize, j: usize) -> bool {
        self.component_id[i] == self.component_id[j]
    }

    fn merge(&mut self, i: usize, j: usize) {
        let old_id = self.component_id[i];
        let new_id = self.component_id[j];

        // If already in same component, nothing to do
        if old_id == new_id {
            return;
        }

        // Update all junction boxes in i's component to j's component
        for id in &mut self.component_id {
            if *id == old_id {
                *id = new_id;
            }
        }
    }

    fn component_sizes(&self) -> Vec<usize> {
        let mut counts: HashMap<usize, usize> = HashMap::new();
        for &id in &self.component_id {
            *counts.entry(id).or_insert(0) += 1;
        }
        counts.values().copied().collect()
    }
}

/// Creates and sorts all possible edges between junction boxes by distance
fn get_sorted_edges(points: &[Point]) -> Vec<(i64, usize, usize)> {
    let mut edges = Vec::new();

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let distance = points[i].distance_squared(&points[j]);
            edges.push((distance, i, j));
        }
    }

    edges.sort_by_key(|&(dist, _, _)| dist);
    edges
}

fn solve_part1(input: &str, num_connections: usize) -> usize {
    let points = parse_input(input);
    let edges = get_sorted_edges(&points);

    let mut components = Components::new(points.len());

    // Connect the specified number of closest pairs
    for (_, i, j) in edges.iter().take(num_connections) {
        components.merge(*i, *j);
    }

    // Get component sizes and multiply the 3 largest
    let mut sizes = components.component_sizes();
    sizes.sort_unstable_by(|a, b| b.cmp(a));
    sizes.iter().take(3).product()
}

pub fn part1(input: &str) -> usize {
    solve_part1(input, 1000)
}

pub fn part2(input: &str) -> usize {
    let points = parse_input(input);
    let edges = get_sorted_edges(&points);

    let mut components = Components::new(points.len());
    let mut num_components = points.len();

    // Keep connecting until all junction boxes are in one circuit
    for (_, i, j) in edges.iter() {
        if !components.same_component(*i, *j) {
            components.merge(*i, *j);
            num_components -= 1;

            // Once we have one circuit, multiply the X coordinates
            if num_components == 1 {
                return (points[*i].x * points[*j].x) as usize;
            }
        }
    }

    unreachable!("Should have connected all components")
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(EXAMPLE, 10), 40);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 25272);
    }
}
