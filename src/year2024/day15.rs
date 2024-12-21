use std::collections::VecDeque;


pub fn part1(input: &str) -> u32 {
    let (top, bottom) = input.split_once("\n\n").unwrap();

    let mut sol = Solution::new(top);
    let moves = Solution::moves(bottom);
    for (dx, dy, _) in moves {
        sol.move_robot(dx, dy);
        // sol.print();
    }
    sol.score()
}

pub fn part2(input: &str) -> usize {
    let (top, bottom) = input.split_once("\n\n").unwrap();

    let mut sol = Solution2::new(top);
    let moves = Solution::moves(bottom);
    for (dx, dy, c) in moves {
        sol.move_robot(dx, dy);
    }
    sol.print();

        sol.grid
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(x, c)| **c == '[')
                    .map(|(x, _)| (y * 100 + x))
                    .sum::<usize>()
            })
            .sum::<usize>()
}

struct Solution2 {
    grid: Vec<Vec<char>>,
}

impl Solution2 {
    fn new(input: &str) -> Self {
        let grid: Vec<Vec<char>> = input
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| {
                        let cs = match c {
                            '#' => "##",
                            '.' => "..",
                            'O' => "[]",
                            '@' => "@.",
                            _ => panic!("Invalid input"),
                        };
                        cs.chars()
                    })
                    .flat_map(|c| c)
                    .collect()
            })
            .collect();
        Self { grid }
    }
    fn robot_pos(&self) -> (i32, i32) {
        for (y, row) in self.grid.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if *c == '@' {
                    return (x as i32, y as i32);
                }
            }
        }
        panic!("Robot not found");
    }

    fn print(&self) {
        for row in &self.grid {
            println!("{}", row.iter().collect::<String>());
        }
    }
    fn move_robot(&mut self, dx: i32, dy: i32) {
        let (x, y) = self.robot_pos();
        // space available
        if self.grid[(y + dy) as usize][(x + dx) as usize] == '.' {
            self.grid[y as usize][x as usize] = '.';
            self.grid[(y + dy) as usize][(x + dx) as usize] = '@';
            return;
        }
        if self.grid[y as usize][(x + dx) as usize] == '#' {
            return;
        }

        if dy == 0 {
            if self.grid[y as usize][(x + dx) as usize] == '.' {
                self.grid[y as usize][x as usize] = '.';
                self.grid[y as usize][(x + dx) as usize] = '@';
                return;
            }

            if self.grid[y as usize][(x + dx) as usize] == '['
                || self.grid[y as usize][(x + dx) as usize] == ']'
            {
                let mut i = 1;
                let found_empty_space_distance = loop {
                    if self.grid[y as usize][(x + (dx * i)) as usize] == '.' {
                        break i;
                    }
                    if self.grid[y as usize][(x + (dx * i)) as usize] == '#' {
                        return;
                    }
                    i += 1;
                };
                if found_empty_space_distance == 1 {
                    return;
                }
                for i in 0..found_empty_space_distance {
                    // push left
                    if dx < 0 {
                        let new_x: usize = (x - found_empty_space_distance + i) as usize;
                        self.grid[y as usize][new_x] = self.grid[y as usize][new_x + 1];
                    } else {
                        let new_x: usize = (x + found_empty_space_distance - i) as usize;
                        self.grid[y as usize][new_x] = self.grid[y as usize][new_x - 1];
                    }
                }
            }
            self.grid[y as usize][x as usize] = '.';
        } else {
            if dy < 0 || dy > 0 {
                let mut fullL_positions: VecDeque<(i32, i32)> = VecDeque::new();
                let mut all_positions: VecDeque<(i32, i32)> = VecDeque::new();
                if self.grid[(y + dy) as usize][x as usize] == '[' {
                    fullL_positions.push_back((x, y + dy));
                } else if self.grid[(y + dy) as usize][x as usize] == ']' {
                    fullL_positions.push_back((x - 1, y + dy));
                } else {
                    // hit the wall
                    return;
                }

                let mut can_move = true;
                while can_move {
                    while let Some((x, y)) = fullL_positions.pop_front() {
                        all_positions.push_back((x, y));
                        let new_y = (y + dy) as usize;
                        if self.grid[new_y][x as usize] == '#'
                            || self.grid[new_y][x as usize + 1] == '#'
                        {
                            // do nothing
                            can_move = false;
                        } else if self.grid[new_y][x as usize] == '.'
                            && self.grid[new_y][x as usize + 1] == '.'
                        {
                        } else {
                            if self.grid[new_y][x as usize] == '[' {
                                fullL_positions.push_back((x, new_y as i32));
                            } else if self.grid[new_y][x as usize] == ']' {
                                fullL_positions.push_back((x - 1, new_y as i32));
                            }
                            if self.grid[new_y][x as usize + 1] == '[' {
                                fullL_positions.push_back((x + 1, new_y as i32));
                            }
                        }
                    }
                    if can_move {
                        // sort all_positions by y desc
                        if dy > 0 {
                            all_positions
                                .make_contiguous()
                                .sort_by(|a, b| b.1.cmp(&a.1));
                        } else {
                            all_positions
                                .make_contiguous()
                                .sort_by(|a, b| a.1.cmp(&b.1));
                        }
                        let robotpos = self.robot_pos();
                        for (x, y) in all_positions.clone() {
                            self.grid[y as usize][x as usize] = '.';
                            self.grid[y as usize][x as usize + 1] = '.';
                            self.grid[(y + dy) as usize][x as usize] = '[';
                            self.grid[(y + dy) as usize][x as usize + 1] = ']';
                        }
                        self.grid[robotpos.1 as usize][robotpos.0 as usize] = '.';
                        self.grid[(robotpos.1 + dy) as usize][robotpos.0 as usize] = '@';
                        can_move = false;
                    }
                    fullL_positions = all_positions.clone();
                }
            }
        }
    }
}

struct Solution {
    grid: Vec<Vec<char>>,
}

impl Solution {
    fn print(&self) {
        for row in &self.grid {
            println!("{}", row.iter().collect::<String>());
        }
    }
    fn score(&self) -> u32 {
        let mut score = 0;
        for (y, row) in self.grid.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if *c == 'O' {
                    score += (x + (100 * y));
                }
            }
        }
        score as u32
    }
    fn move_robot(&mut self, dx: i32, dy: i32) {
        let (x, y) = self.robot_pos();
        if self.grid[(y + dy) as usize][(x + dx) as usize] == '.' {
            self.grid[y as usize][x as usize] = '.';
            self.grid[(y + dy) as usize][(x + dx) as usize] = '@';
            return;
        }
        let len = self.length_to_push(dx, dy);
        if len == 0 {
            return;
        }
        for i in 2..=(len + 1) {
            self.grid[(y + (i * dy)) as usize][(x + (dx * i)) as usize] = 'O';
        }
        self.grid[y as usize][x as usize] = '.';
        self.grid[(y + dy) as usize][(x + dx) as usize] = '@';
    }
    fn length_to_push(&self, dx: i32, dy: i32) -> i32 {
        let (x, y) = self.robot_pos();
        let mut length = 0;
        let (mut nx, mut ny) = (x + dx, y + dy);
        while nx >= 0 && ny >= 0 && nx < self.grid[0].len() as i32 && ny < self.grid.len() as i32 {
            if self.grid[ny as usize][nx as usize] == '#' {
                return 0;
            }
            if self.grid[ny as usize][nx as usize] == '.' {
                return length;
            }
            length += 1;
            nx += dx;
            ny += dy;
        }
        unreachable!("Should not reach here");
    }

    fn robot_pos(&self) -> (i32, i32) {
        for (y, row) in self.grid.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if *c == '@' {
                    return (x as i32, y as i32);
                }
            }
        }
        panic!("Robot not found");
    }
    fn new(input: &str) -> Self {
        let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        Self { grid }
    }

    fn moves(input: &str) -> Vec<(i32, i32, char)> {
        let valid = "^v<>".chars().collect::<Vec<char>>();
        input
            .chars()
            .filter(|c| valid.contains(c))
            .map(|c| match c {
                '^' => (0, -1, c),
                'v' => (0, 1, c),
                '<' => (-1, 0, c),
                '>' => (1, 0, c),
                _ => {
                    panic!("Invalid input")
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_large() {
        let result = part1(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }
    #[test]
    fn test_part1_small() {
        let result = part1(
            r#"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"#,
        );
        assert_eq!(result, Some(2028));
    }

    #[test]
    fn test_part2() {
        let result = part2(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part2_a() {
        let result = part2(
            r#"#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^"#,
        );
        assert_eq!(result, Some(3606));
    }

    #[test]
    fn test_part2_large() {
        let result = part2(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
