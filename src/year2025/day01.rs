//! # Day 1: [Title]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    direction: Direction,
    steps: usize,
}

impl Instruction {
    fn parse(line: &str) -> Self {
        let direction = match line.chars().next().unwrap() {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid direction"),
        };
        let steps = line[1..].parse().unwrap();
        Self { direction, steps }
    }
}

struct Dial {
    position: usize,
}

impl Dial {
    fn new(position:usize) -> Self {
        Self { position }
    }

    fn turn(&mut self, direction: Direction, steps: usize) {
        // dial goes from 0 to 99 and loops around
        let delta = steps as i32 * (if direction == Direction::Left { -1 } else { 1 });
        self.position = (self.position as i32 + delta).rem_euclid(100) as usize;
    }

    fn count_zero_crossings(&self, direction: Direction, steps: usize) -> usize {
        // Count how many times we pass through position 0 during this turn
        if direction == Direction::Right {
            // Moving right: we hit 0 at positions 100-p, 200-p, etc.
            (self.position + steps) / 100
        } else {
            // Moving left: we hit 0 at step p, p+100, p+200, etc.
            // Special case: if already at 0, we don't hit 0 until we go around (step 100)
            if self.position == 0 {
                steps / 100
            } else if steps >= self.position {
                (steps - self.position) / 100 + 1
            } else {
                0
            }
        }
    }
}

pub fn part1(input: &str) -> usize {
    let instructions: Vec<Instruction> = input
        .lines()
        .map(|line| Instruction::parse(line))
        .collect();
    let mut times_at_zero = 0;
    let mut dial = Dial::new(50);
    for instruction in instructions {
        dial.turn(instruction.direction, instruction.steps);
        println!("{:?} {:?}", dial.position, instruction.direction);
        if dial.position == 0 {
            times_at_zero += 1;
        }
    }

    times_at_zero
}

pub fn part2(input: &str) -> usize {
    let instructions: Vec<Instruction> = input
        .lines()
        .map(|line| Instruction::parse(line))
        .collect();
    let mut total_zero_crossings = 0;
    let mut dial = Dial::new(50);
    for instruction in instructions {
        total_zero_crossings += dial.count_zero_crossings(instruction.direction, instruction.steps);
        dial.turn(instruction.direction, instruction.steps);
    }

    total_zero_crossings
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 6);
    }
}
