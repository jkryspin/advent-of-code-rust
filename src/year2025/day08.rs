//! # Day 8: [Title]

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn distance_squared(&self, other: &Point) -> i32 {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)
    }
}

fn parse_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<i32> = line.split(',').map(|s| s.parse().unwrap()).collect();
            Point {
                x: parts[0],
                y: parts[1],
                z: parts[2],
            }
        })
        .collect()
}

struct UnionFind {
    parent: HashMap<usize, usize>,
    size: HashMap<usize, usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let mut parent = HashMap::new();
        let mut size = HashMap::new();
        for i in 0..n {
            parent.insert(i, i);
            size.insert(i, 1);
        }
        UnionFind { parent, size }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[&x] != x {
            let root = self.find(self.parent[&x]);
            self.parent.insert(x, root);
        }
        self.parent[&x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x != root_y {
            if self.size[&root_x] < self.size[&root_y] {
                self.parent.insert(root_x, root_y);
                let new_size = self.size[&root_y] + self.size[&root_x];
                self.size.insert(root_y, new_size);
            } else {
                self.parent.insert(root_y, root_x);
                let new_size = self.size[&root_x] + self.size[&root_y];
                self.size.insert(root_x, new_size);
            }
        }
    }

    fn get_component_sizes(&mut self) -> Vec<usize> {
        let mut components: HashMap<usize, usize> = HashMap::new();
        for i in 0..self.parent.len() {
            let root = self.find(i);
            *components.entry(root).or_insert(0) += 1;
        }
        components.values().copied().collect()
    }
}

pub fn part1(input: &str) -> usize {
    let points = parse_input(input);
    let n = points.len();

    // Find all edges with their distances
    let mut edges: Vec<(i32, usize, usize)> = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            let dist = points[i].distance_squared(&points[j]);
            edges.push((dist, i, j));
        }
    }

    // Sort edges by distance
    edges.sort_by_key(|e| e.0);

    // Add only the 10 shortest edges
    let mut uf = UnionFind::new(n);
    for i in 0..1000.min(edges.len()) {
        uf.union(edges[i].1, edges[i].2);
    }

    // Get all component sizes
    let mut sizes = uf.get_component_sizes();
    sizes.sort_by(|a, b| b.cmp(a)); // Sort descending

    // Multiply the 3 largest component sizes
    sizes.iter().take(3).product()
}

pub fn part2(input: &str) -> usize {
    0
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
        assert_eq!(part1(EXAMPLE), 40);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 0);
    }
}
