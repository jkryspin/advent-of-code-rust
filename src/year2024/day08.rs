use std::collections::{HashMap, HashSet};


pub fn part1(input: &str) -> u32 {
    // hashmap with key char and value x,y
    let mut positions_by_char: HashMap<char, HashSet<(i32, i32)>> = HashMap::new();
    let y_max = input.lines().count() as i32;
    let x_max = input.lines().next().unwrap().chars().count() as i32;
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if c != '.' {
                positions_by_char
                    .entry(c)
                    .or_insert(HashSet::new())
                    .insert((x as i32, y as i32));
            }
        });
    });
    // for each char in hashmap, calculate the position double the distance between each point
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    for positions in positions_by_char.values() {
        let positions_vec: Vec<&(i32, i32)> = positions.iter().collect();

        // iterate in chunks of 2
        for i in 0..positions_vec.len() {
            for j in i + 1..positions_vec.len() {
                let (x1, y1) = positions_vec[i];
                let (x2, y2) = positions_vec[j];
                let dx = x2 - x1;
                let dy = y2 - y1;
                let candidates = vec![(x1 - dx, y1 - dy), (x2 + dx, y2 + dy)];
                for (x, y) in candidates {
                    if x >= 0 && x < x_max && y >= 0 && y < y_max {
                        antinodes.insert((x, y));
                    }
                }
            }
        }
    }

    antinodes.len() as u32
}

pub fn part2(input: &str) -> u32 {
    let mut positions_by_char: HashMap<char, HashSet<(i32, i32)>> = HashMap::new();
    let y_max = input.lines().count() as i32;
    let x_max = input.lines().next().unwrap().chars().count() as i32;
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if c != '.' {
                positions_by_char
                    .entry(c)
                    .or_insert(HashSet::new())
                    .insert((x as i32, y as i32));
            }
        });
    });
    // for each char in hashmap, calculate the position double the distance between each point
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    for positions in positions_by_char.values() {
        // all a's
        let positions_vec: Vec<&(i32, i32)> = positions.iter().collect();
        // iterate in chunks of 2
        for i in 0..positions_vec.len() {
            for j in i + 1..positions_vec.len() {
                let (x1, y1) = positions_vec[i];
                let (x2, y2) = positions_vec[j];
                let dx = x2 - x1;
                let dy = y2 - y1;
                let mut x = x1 - dx;
                let mut y = y1 - dy;
                while x >= 0 && x < x_max && y >= 0 && y < y_max {
                    antinodes.insert((x, y));
                    x -= dx;
                    y -= dy;
                }
                x = x2 + dx;
                y = y2 + dy;
                while x >= 0 && x < x_max && y >= 0 && y < y_max {
                    antinodes.insert((x, y));
                    x += dx;
                    y += dy;
                }
            }
        }
    }

    positions_by_char.values().for_each(|v| {
        v.iter().for_each(|(x, y)| {
            antinodes.insert((*x, *y));
        })
    });

    antinodes.len() as u32
}
