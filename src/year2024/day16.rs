use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

fn shortest_path_with_cost(
    grid: &Vec<Vec<char>>,
    start: (usize, usize, Direction, Direction),
    end: (usize, usize),
    part1:bool
) -> Option<usize> {
    let mut dist: HashMap<(usize, usize, Direction, Direction), usize> = HashMap::new();
    let mut heap = BinaryHeap::new();

    // Initialize the distance to the start node to 0
    dist.insert(start, 0);
    heap.push(State {
        cost: 0,
        position: start,
        positions_visited: HashSet::new(),
    });


    // Directions for moving in the grid (up, down, left, right)
    let directions = [(0, 1), (-1, 0), (0, -1), (1, 0)];
    let mut expected_len = None;
    let mut total_positions = HashSet::new();
    while let Some(State { cost, position, positions_visited }) = heap.pop() {
        // If we reached the end, reconstruct all paths and return the cost and number of visited nodes
        if position.0 == end.0 && position.1 == end.1 {
            // bfs through positions_visited
            if part1 {
                return Some(cost);
            }
            if expected_len.is_none() {
                expected_len = Some(positions_visited.len());
            }
            if positions_visited.len() == expected_len.unwrap() {
                // add to total_positions
                total_positions.extend(positions_visited.iter().map(|(x,y)|(*x,*y)));
            }
        }
        // If the cost is greater than the recorded distance, skip this node
        if cost > *dist.get(&position).unwrap_or(&usize::MAX) {
            continue;
        }

        // Explore the neighbors
        for direction in &directions {
            // skip if direction is opposite of current direction
            if Direction::from(direction) == position.3.opposite() {
                continue;
            }
            let new_position = (
                (position.0 as isize + direction.0) as usize,
                (position.1 as isize + direction.1) as usize,
                Direction::from(direction),
                position.2,
            );

            // Check if the new position is within the grid bounds
            if new_position.1 < grid.len() && new_position.1 < grid[0].len()
            {
                if grid[new_position.0][new_position.1] == '#' {
                    continue;
                }
                let next_cost = if Direction::from(direction) != position.2 {
                    cost + 1001
                } else {
                    cost + 1
                };

                // If the new cost is less than the recorded distance, update the distance and push to the heap
                if next_cost <= *dist.get(&new_position).unwrap_or(&usize::MAX) {
                    dist.insert(new_position, next_cost);
                    if new_position.0 == start.0 && new_position.1 == start.1 {
                        continue;
                    }

                    let mut clone = positions_visited.clone();
                    clone.insert((new_position.0, new_position.1));

                    heap.push(State {
                        cost: next_cost,
                        position: new_position,
                        positions_visited: clone,
                    });
                }
            }
        }
    }
    total_positions.insert((start.0, start.1));
    total_positions.insert((end.0, end.1));
    Some(total_positions.len())
}

fn find_char(grid: &Vec<Vec<char>,>, c: char) -> (usize, usize) {
    grid.iter()
        .enumerate()
        .find_map(|(y, row)| row.iter().position(|&c2| c2 == c).map(|x| (x, y)))
        .unwrap()
}

pub fn part1(input: &str) -> u32 {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let start = find_char(&grid, 'S');
    let end = find_char(&grid, 'E');

    let start = (start.0, start.1, Direction::Right, Direction::Right);

    let asn =
        shortest_path_with_cost(&grid, start, end, true).map(|cost| (1000 + cost as u32));

    //add 9 to 7 unique values

    asn.unwrap()
}

pub fn part2(input: &str) -> u32 {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let start = grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| row.iter().position(|&c| c == 'S').map(|x| (x, y)))
        .unwrap();
    let end = grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| row.iter().position(|&c| c == 'E').map(|x| (x, y)))
        .unwrap();

    let start = (start.0, start.1, Direction::Right, Direction::Right);

    let len = shortest_path_with_cost(&grid, start, end, false).unwrap();
    len as u32
}

#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (usize, usize, Direction, Direction), // (x, y, current direction, previous direction)
    positions_visited: HashSet<(usize, usize)>,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction{
    fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

impl From<&(isize, isize)> for Direction {
    fn from((x, y): &(isize, isize)) -> Self {
        match (x, y) {
            (0, 1) => Direction::Down,
            (1, 0) => Direction::Right,
            (0, -1) => Direction::Up,
            (-1, 0) => Direction::Left,
            _ => panic!("Invalid direction"),
        }
    }
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part1_a() {
        let result = part1(
            r#"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
"#,
        );
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part2() {
        let result = part2(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45));
    }

    #[test]
    fn test_part2_a() {
        let result = part2(
            r#"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
"#,
        );
        assert_eq!(result, Some(64));
    }
}
