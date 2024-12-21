use cached::proc_macro::cached;

const NUM_PAD: [&str; 4] = ["789", "456", "123", " 0A"];
const DIR_PAD: [&str; 2] = [" ^A", "<v>"];

pub fn part1(input: &str) -> usize {
    input.lines()
        .map(|line| {
            solve(line.trim().to_string(), 0, 2) * line[..3].parse::<usize>().unwrap()
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    input.lines()
        .map(|line| {
            solve(line.trim().to_string(), 0, 25) * line[..3].parse::<usize>().unwrap()
        })
        .sum()
}

fn path(pad: &[&str], from: char, to: char) -> String {
    let (from_x, from_y) = pad.iter().enumerate()
        .flat_map(|(y, row)| row.chars().enumerate().map(move |(x, c)| (x, y, c)))
        .find(|&(_, _, c)| c == from)
        .map(|(x, y, _)| (x, y))
        .unwrap();

    let (to_x, to_y) = pad.iter().enumerate()
        .flat_map(|(y, row)| row.chars().enumerate().map(move |(x, c)| (x, y, c)))
        .find(|&(_, _, c)| c == to)
        .map(|(x, y, _)| (x, y))
        .unwrap();

    fn generate_paths(pad: &[&str], x: usize, y: usize, target_x: usize, target_y: usize, path: String) -> Vec<String> {
        let mut results = Vec::new();
        if (x, y) == (target_x, target_y) {
            results.push(path.clone() + "A");
        }
        let directions = [
            (target_x < x && x > 0, x.wrapping_sub(1), y, "<"),
            (target_y < y && y > 0, x, y.wrapping_sub(1), "^"),
            (target_y > y && y < pad.len() - 1, x, y + 1, "v"),
            (target_x > x && x < pad[0].len() - 1, x + 1, y, ">"),
        ];
        for &(condition, new_x, new_y, dir) in &directions {
            if condition && pad[new_y].chars().nth(new_x).unwrap() != ' ' {
                results.extend(generate_paths(pad, new_x, new_y, target_x, target_y, path.clone() + dir));
            }
        }
        results
    }

    // find the minimum direction changes so that we dont have to move
    generate_paths(pad, from_x, from_y, to_x, to_y, String::new())
        .into_iter()
        .min_by_key(|path| path.chars().zip(path.chars().skip(1)).filter(|(a, b)| a != b).count())
        .unwrap()
}

#[cached]
fn solve(sequence: String, level: usize, limit: u8) -> usize {
        if level > limit as usize {
            return sequence.len();
        }
    let result = ('A'.to_string() + &*sequence)
        .chars()
        .zip(sequence.chars())
        .map(|(from, to)| solve(path(if level == 0 { &NUM_PAD } else { &DIR_PAD }, from, to), level + 1, limit))
        .sum();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part2() {
        let result = part2(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}