//! # Day 8: [Title]

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

fn solve(input: &str, num_connections: usize) -> usize {
    let points = parse_input(input);
    let n = points.len();

    // Find all edges with their distances
    let mut edges: Vec<(i64, usize, usize)> = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            let dist = points[i].distance_squared(&points[j]);
            edges.push((dist, i, j));
        }
    }

    // Sort edges by distance
    edges.sort_by_key(|e| e.0);

    // Add the specified number of shortest edges
    let mut uf = UnionFind::new(n);
    for i in 0..num_connections.min(edges.len()) {
        uf.union(edges[i].1, edges[i].2);
    }

    // Get all component sizes
    let mut sizes = uf.get_component_sizes();
    sizes.sort_by(|a, b| b.cmp(a)); // Sort descending

    // Multiply the 3 largest component sizes
    sizes.iter().take(3).product()
}

pub fn part1(input: &str) -> usize {
    solve(input, 1000)
}

pub fn part2(input: &str) -> usize {
    let points = parse_input(input);
    let n = points.len();

    // Find all edges with their distances
    let mut edges: Vec<(i64, usize, usize)> = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            let dist = points[i].distance_squared(&points[j]);
            edges.push((dist, i, j));
        }
    }

    // Sort edges by distance
    edges.sort_by_key(|e| e.0);

    // Add edges until we have only 1 component
    let mut uf = UnionFind::new(n);
    let mut num_components = n;
    let mut last_edge = (0, 0);

    for (_, i, j) in edges.iter() {
        // Check if these nodes are in different components
        if uf.find(*i) != uf.find(*j) {
            uf.union(*i, *j);
            num_components -= 1;
            last_edge = (*i, *j);

            if num_components == 1 {
                break;
            }
        }
    }

    // Multiply the X coordinates of the last two connected junction boxes
    (points[last_edge.0].x * points[last_edge.1].x) as usize
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
        assert_eq!(solve(EXAMPLE, 10), 40);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 25272);
    }
}
