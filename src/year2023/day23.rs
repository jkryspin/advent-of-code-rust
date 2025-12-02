use std::collections::{HashMap, HashSet, VecDeque};

type Point = (i32, i32);

struct Grid {
    cells: Vec<Vec<char>>,
    height: i32,
    width: i32,
    start: Point,
    end: Point,
}

impl Grid {
    fn parse(input: &str) -> Self {
        let cells: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let height = cells.len() as i32;
        let width = cells[0].len() as i32;

        // Find start (top row with '.')
        let start = (0, cells[0].iter().position(|&c| c == '.').unwrap() as i32);

        // Find end (bottom row with '.')
        let end = (
            height - 1,
            cells[(height - 1) as usize].iter().position(|&c| c == '.').unwrap() as i32,
        );

        Self { cells, height, width, start, end }
    }

    fn get(&self, pos: Point) -> char {
        if pos.0 < 0 || pos.0 >= self.height || pos.1 < 0 || pos.1 >= self.width {
            return '#';
        }
        self.cells[pos.0 as usize][pos.1 as usize]
    }

    fn neighbors(&self, pos: Point, respect_slopes: bool) -> Vec<Point> {
        let (r, c) = pos;
        let cell = self.get(pos);

        let mut dirs = vec![];

        if respect_slopes {
            match cell {
                '>' => dirs.push((0, 1)),
                '<' => dirs.push((0, -1)),
                'v' => dirs.push((1, 0)),
                '^' => dirs.push((-1, 0)),
                '.' => {
                    dirs.push((0, 1));
                    dirs.push((0, -1));
                    dirs.push((1, 0));
                    dirs.push((-1, 0));
                }
                _ => {}
            }
        } else {
            dirs.push((0, 1));
            dirs.push((0, -1));
            dirs.push((1, 0));
            dirs.push((-1, 0));
        }

        dirs.into_iter()
            .map(|(dr, dc)| (r + dr, c + dc))
            .filter(|&next| self.get(next) != '#')
            .collect()
    }

    // Find all junction points (nodes with > 2 neighbors) plus start and end
    fn find_junctions(&self, respect_slopes: bool) -> Vec<Point> {
        let mut junctions = vec![self.start, self.end];

        for r in 0..self.height {
            for c in 0..self.width {
                let pos = (r, c);
                if self.get(pos) != '#' && pos != self.start && pos != self.end {
                    let neighbors = self.neighbors(pos, respect_slopes);
                    if neighbors.len() > 2 {
                        junctions.push(pos);
                    }
                }
            }
        }

        junctions
    }

    // Build compressed graph: map junction -> (neighbor_junction, distance)
    fn build_junction_graph(&self, respect_slopes: bool) -> HashMap<Point, Vec<(Point, usize)>> {
        let junctions = self.find_junctions(respect_slopes);
        let junction_set: HashSet<Point> = junctions.iter().copied().collect();
        let mut graph: HashMap<Point, Vec<(Point, usize)>> = HashMap::new();

        for &start_junction in &junctions {
            let mut edges = vec![];

            // BFS from this junction to find neighboring junctions
            let mut queue = VecDeque::new();
            let mut visited = HashSet::new();
            queue.push_back((start_junction, 0));
            visited.insert(start_junction);

            while let Some((pos, dist)) = queue.pop_front() {
                if pos != start_junction && junction_set.contains(&pos) {
                    edges.push((pos, dist));
                    continue; // Don't explore past this junction
                }

                for neighbor in self.neighbors(pos, respect_slopes) {
                    if !visited.contains(&neighbor) {
                        visited.insert(neighbor);
                        queue.push_back((neighbor, dist + 1));
                    }
                }
            }

            graph.insert(start_junction, edges);
        }

        graph
    }
}

fn longest_path_compressed(
    graph: &HashMap<Point, Vec<(Point, usize)>>,
    current: Point,
    end: Point,
    visited: &mut HashSet<Point>,
) -> Option<usize> {
    if current == end {
        return Some(0);
    }

    visited.insert(current);
    let mut max_length = None;

    if let Some(neighbors) = graph.get(&current) {
        for &(next, dist) in neighbors {
            if !visited.contains(&next) {
                if let Some(length) = longest_path_compressed(graph, next, end, visited) {
                    let total = dist + length;
                    max_length = Some(max_length.map_or(total, |m: usize| m.max(total)));
                }
            }
        }
    }

    visited.remove(&current); // Backtrack
    max_length
}

pub fn part1(input: &str) -> u32 {
    let grid = Grid::parse(input);
    let graph = grid.build_junction_graph(true);
    let mut visited = HashSet::new();
    longest_path_compressed(&graph, grid.start, grid.end, &mut visited).unwrap() as u32
}

pub fn part2(input: &str) -> u32 {
    let grid = Grid::parse(input);
    let graph = grid.build_junction_graph(false);
    let mut visited = HashSet::new();
    longest_path_compressed(&graph, grid.start, grid.end, &mut visited).unwrap() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 94);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 154);
    }
}
