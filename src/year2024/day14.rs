use regex::Regex;


pub fn part1(input: &str) -> u32 {
    solve(input, true)
}

#[derive(Debug)]
struct Robot {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

pub fn part2(input: &str) -> u32 {
    solve(input, false)
}

fn solve(input: &str, part1: bool) -> u32 {
    let re = Regex::new(r"(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
    let mut robots = vec![];
    let width = 101;
    let height = 103;

    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        let x = caps[1].parse().unwrap();
        let y = caps[2].parse().unwrap();
        let vx = caps[3].parse().unwrap();
        let vy = caps[4].parse().unwrap();
        robots.push(Robot { x, y, vx, vy });
    }
    let end = if part1 { 100 } else { usize::MAX };
    for i in 0..end {
        for robot in &mut robots {
            robot.x += robot.vx;
            robot.y += robot.vy;
        }

        if part1 {
            continue;
        }

        // print robots of grid of width x height
        let mut grid = vec![vec![0; width as usize]; height as usize];
        for robot in robots.iter_mut() {
            robot.x = robot.x % width;
            robot.y = robot.y % height;
            if robot.x < 0 {
                robot.x += width;
            }
            if robot.y < 0 {
                robot.y += height;
            }
            assert!(robot.x >= 0);
            assert!(robot.y >= 0);
            grid[robot.y as usize][robot.x as usize] += 1;
        }
        // check if triangle exists with top 2 sides with length at least 12
        for y in 0..height {
            for x in 0..width {
                if grid[y as usize][x as usize] > 0 {
                    let mut count = 0;
                    for i in 0..6 {
                        let next_left = (x - i) % width;
                        let next_right = (x + i) % width;
                        let next_y = y + i;
                        // if not in bounds, break
                        if next_left < 0 || next_right < 0 {
                            break;
                        }
                        if next_left >= width || next_right >= width {
                            break;
                        }
                        if next_y >= height {
                            break;
                        }
                        if grid[y as usize + i as usize][next_left as usize] > 0
                            && grid[y as usize + i as usize][next_right as usize] > 0
                        {
                            count += 1;
                        } else {
                            break;
                        }
                    }
                    if count >= 6 {
                        for row in &grid {
                            println!("{}", row.iter().map(|s| s.to_string()).collect::<String>());
                        }
                        return i as u32 + 1;
                    }
                }
            }
        }
    }

    for robot in robots.iter_mut() {
        robot.x = robot.x % width;
        robot.y = robot.y % height;
        if robot.x < 0 {
            robot.x += width;
        }
        if robot.y < 0 {
            robot.y += height;
        }
        assert!(robot.x >= 0);
        assert!(robot.y >= 0);
    }

    let mut quadrant_counts = [0; 4];
    for robot in &robots {
        let middle_x = width / 2;
        let middle_y = height / 2;
        let quadrant = if robot.x < middle_x {
            if robot.y < middle_y {
                Some(0)
            } else if robot.y > middle_y {
                Some(2)
            } else {
                None
            }
        } else if robot.x > middle_x {
            if robot.y < middle_y {
                Some(1)
            } else if robot.y > middle_y {
                Some(3)
            } else {
                None
            }
        } else {
            None
        };
        match quadrant {
            Some(q) => {
                quadrant_counts[q] += 1;
            }
            None => {}
        }
    }
    quadrant_counts.iter().product()
}