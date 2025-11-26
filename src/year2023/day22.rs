use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;

type Point3D = (i32, i32, i32);

#[derive(Debug, Clone)]
struct Brick {
    id: usize,
    start: Point3D,
    end: Point3D,
}

impl Brick {
    fn parse(id: usize, line: &str) -> Self {
        let (start, end) = line.split_once('~').unwrap();
        let parse = |s: &str| {
            let nums: Vec<i32> = s.split(',').map(|n| n.parse().unwrap()).collect();
            (nums[0], nums[1], nums[2])
        };
        Self { id, start: parse(start), end: parse(end) }
    }

    // Get all points this brick occupies
    fn points(&self) -> Vec<Point3D> {
        let mut pts = Vec::new();
        for x in self.start.0..=self.end.0 {
            for y in self.start.1..=self.end.1 {
                for z in self.start.2..=self.end.2 {
                    pts.push((x, y, z));
                }
            }
        }
        pts
    }

    // Move brick down by dz units
    fn drop(&mut self, dz: i32) {
        self.start.2 -= dz;
        self.end.2 -= dz;
    }
}

struct Space {
    grid: FxHashMap<Point3D, usize>, // Maps point -> brick ID
    bricks: Vec<Brick>,
}

impl Space {
    fn new(input: &str) -> Self {
        let bricks: Vec<Brick> = input.lines()
            .enumerate()
            .map(|(i, line)| Brick::parse(i, line))
            .collect();

        let mut space = Self {
            grid: FxHashMap::default(),
            bricks,
        };

        space.settle_all_bricks();
        space
    }

    // Fill grid with brick positions
    fn fill_brick(&mut self, brick: &Brick) {
        for pt in brick.points() {
            self.grid.insert(pt, brick.id);
        }
    }

    // Remove brick from grid
    fn remove_brick(&mut self, brick_id: usize) {
        let brick = &self.bricks[brick_id];
        for pt in brick.points() {
            self.grid.remove(&pt);
        }
    }

    // Settle all bricks by dropping them
    fn settle_all_bricks(&mut self) {
        // Sort by z coordinate (lowest first)
        let mut indices: Vec<usize> = (0..self.bricks.len()).collect();
        indices.sort_by_key(|&i| self.bricks[i].start.2.min(self.bricks[i].end.2));

        for &idx in &indices {
            // Find how far this brick can drop
            let mut drop_distance = 0;
            loop {
                let brick = &self.bricks[idx];
                if brick.start.2 - drop_distance <= 1 {
                    break; // Hit ground
                }

                // Check if any point below is occupied
                let can_drop = brick.points().iter().all(|&(x, y, z)| {
                    let below = (x, y, z - drop_distance - 1);
                    !self.grid.contains_key(&below) || self.grid[&below] == idx
                });

                if !can_drop {
                    break;
                }
                drop_distance += 1;
            }

            // Drop the brick
            if drop_distance > 0 {
                self.bricks[idx].drop(drop_distance);
            }
            let brick = self.bricks[idx].clone();
            self.fill_brick(&brick);
        }
    }

    // Get brick IDs that are directly supported by brick_id
    fn supported_by(&self, brick_id: usize) -> Vec<usize> {
        let brick = &self.bricks[brick_id];
        let mut supported = FxHashMap::default();

        for (x, y, z) in brick.points() {
            let above = (x, y, z + 1);
            if let Some(&other_id) = self.grid.get(&above) {
                if other_id != brick_id {
                    supported.insert(other_id, ());
                }
            }
        }

        supported.into_keys().collect()
    }

    // Check if removing brick_id would cause others to fall
    fn can_disintegrate(&self, brick_id: usize) -> bool {
        let supported = self.supported_by(brick_id);

        // For each brick we support, check if it has other supports
        for &other_id in &supported {
            let other_brick = &self.bricks[other_id];
            let mut has_other_support = false;

            // Check points below this brick
            for (x, y, z) in other_brick.points() {
                let below = (x, y, z - 1);
                if let Some(&support_id) = self.grid.get(&below) {
                    if support_id != brick_id && support_id != other_id {
                        has_other_support = true;
                        break;
                    }
                }
            }

            if !has_other_support {
                return false; // This brick relies solely on brick_id
            }
        }

        true
    }

    // Count how many bricks would fall in a chain reaction if brick_id is removed
    fn count_chain_reaction(&self, brick_id: usize) -> usize {
        let mut fallen = FxHashSet::default();
        fallen.insert(brick_id);

        let mut queue = VecDeque::new();
        queue.push_back(brick_id);

        while let Some(current_id) = queue.pop_front() {
            // Find all bricks supported by current brick
            let supported = self.supported_by(current_id);

            for &other_id in &supported {
                if fallen.contains(&other_id) {
                    continue;
                }

                // Check if this brick has any support left
                let other_brick = &self.bricks[other_id];
                let mut has_support = false;

                for (x, y, z) in other_brick.points() {
                    if z == 1 {
                        // This point is on the ground
                        has_support = true;
                        break;
                    }

                    let below = (x, y, z - 1);
                    if let Some(&support_id) = self.grid.get(&below) {
                        // There's a brick below this point
                        if support_id != other_id && !fallen.contains(&support_id) {
                            // And it hasn't fallen
                            has_support = true;
                            break;
                        }
                    }
                }

                // If no support remaining, it falls
                if !has_support {
                    fallen.insert(other_id);
                    queue.push_back(other_id);
                }
            }
        }

        fallen.len() - 1 // Subtract 1 to not count the original brick
    }
}

pub fn part1(input: &str) -> usize {
    let space = Space::new(input);

    (0..space.bricks.len())
        .filter(|&i| space.can_disintegrate(i))
        .count()
}

pub fn part2(input: &str) -> usize {
    let space = Space::new(input);

    (0..space.bricks.len())
        .map(|i| space.count_chain_reaction(i))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 5);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 7);
    }
}
