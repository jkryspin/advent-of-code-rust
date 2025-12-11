//! # Day 10: [Title]

use itertools::Itertools;
use rustc_hash::FxHashMap;

#[derive(Debug)]
struct Puzzle {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    target_joltage: Vec<usize>,
}

fn parse_line(line: &str) -> Puzzle {
    let parse_between = |start_char, end_char| {
        line.find(start_char)
            .and_then(|s| line[s..].find(end_char).map(|e| &line[s + 1..s + e]))
    };

    let lights = parse_between('[', ']')
        .map(|s| s.chars().map(|c| c == '#').collect())
        .unwrap_or_default();

    let buttons = line.match_indices('(')
        .filter_map(|(i, _)| {
            line[i..].find(')').map(|j| {
                line[i + 1..i + j]
                    .split(',')
                    .filter_map(|s| s.trim().parse().ok())
                    .collect()
            })
        })
        .collect();

    let target_joltage = parse_between('{', '}')
        .map(|s| s.split(',').filter_map(|s| s.trim().parse().ok()).collect())
        .unwrap_or_default();

    Puzzle { lights, buttons, target_joltage }
}

fn solve_puzzle(puzzle: &Puzzle) -> usize {
    (0..(1 << puzzle.buttons.len()))
        .filter_map(|mask| {
            let mut state = vec![false; puzzle.lights.len()];
            let mut presses = 0;

            for (i, button) in puzzle.buttons.iter().enumerate() {
                if (mask >> i) & 1 == 1 {
                    presses += 1;
                    for &idx in button {
                        if idx < state.len() {
                            state[idx] = !state[idx];
                        }
                    }
                }
            }

            (state == puzzle.lights).then_some(presses)
        })
        .min()
        .unwrap_or(usize::MAX)
}

pub fn part1(input: &str) -> usize {
    input.lines()
        .map(|line| solve_puzzle(&parse_line(line)))
        .sum()
}

fn generate_patterns(buttons: &[Vec<usize>], num_indices: usize) -> Vec<(Vec<usize>, usize)> {
    let mut patterns = Vec::new();

    for len in 0..=buttons.len() {
        for combo in (0..buttons.len()).combinations(len) {
            let mut pattern = vec![0; num_indices];
            for i in combo {
                for &idx in &buttons[i] {
                    if idx < num_indices {
                        pattern[idx] += 1;
                    }
                }
            }
            patterns.push((pattern, len));
        }
    }
    patterns
}

fn solve_recursive(
    patterns: &[(Vec<usize>, usize)],
    goal: Vec<usize>,
    cache: &mut FxHashMap<Vec<usize>, usize>,
) -> usize {
    if goal.iter().all(|&x| x == 0) {
        return 0;
    }

    if let Some(&result) = cache.get(&goal) {
        return result;
    }

    let result = patterns.iter()
        .filter(|(p, _)| p.iter().zip(&goal).all(|(p, g)| p <= g && p % 2 == g % 2))
        .filter_map(|(p, cost)| {
            let new_goal: Vec<_> = p.iter().zip(&goal).map(|(p, g)| (g - p) / 2).collect();
            let sub_result = solve_recursive(patterns, new_goal, cache);
            (sub_result != usize::MAX).then(|| cost + 2 * sub_result)
        })
        .min()
        .unwrap_or(usize::MAX);

    cache.insert(goal, result);
    result
}

fn solve_joltage(puzzle: &Puzzle) -> usize {
    let patterns = generate_patterns(&puzzle.buttons, puzzle.target_joltage.len());
    let mut cache = FxHashMap::default();
    solve_recursive(&patterns, puzzle.target_joltage.clone(), &mut cache)
}

pub fn part2(input: &str) -> usize {
    input.lines()
        .map(|line| solve_joltage(&parse_line(line)))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 7);
    }

    #[test]
    fn test_parse() {
        let puzzle = parse_line("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}");
        assert_eq!(puzzle.lights, vec![false, true, true, false]);
        assert_eq!(puzzle.buttons, vec![vec![3], vec![1, 3], vec![2], vec![2, 3], vec![0, 2], vec![0, 1]]);
        assert_eq!(puzzle.target_joltage, vec![3, 5, 4, 7]);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 33);
    }

    const REAL_INPUT_SAMPLE: &str = "[..##.##.#] (2,3,5,6,8) (0,3,7) (1,4,8) (0,1,6) (3,4,5,8) (2,3,7,8) (0,2,3,8) {21,15,27,149,126,127,16,12,153}
[#####..#] (0,1,4,5,6) (0,1,2,4,5,6) (1,2) (2,3) (2) (5) (0,4,5,7) {33,29,57,14,33,47,20,13}
[#.###] (0,1,3) (0,1,4) (0,2,3,4) (1,2) {20,29,13,6,16}
[#.###] (0,2) (1,2,4) (1,2,3) (0,2,4) {9,180,189,163,21}";

    #[test]
    fn test_real_input_line1() {
        let line = "[..##.##.#] (2,3,5,6,8) (0,3,7) (1,4,8) (0,1,6) (3,4,5,8) (2,3,7,8) (0,2,3,8) {21,15,27,149,126,127,16,12,153}";
        let puzzle = parse_line(line);
        assert_eq!(puzzle.lights.len(), 9);
        assert_eq!(puzzle.buttons.len(), 7);
        assert_eq!(puzzle.target_joltage, vec![21,15,27,149,126,127,16,12,153]);

        // Should complete without panic
        let result = solve_joltage(&puzzle);
        eprintln!("Real input line 1 result: {}", result);
        assert!(result < usize::MAX);
    }

    #[test]
    fn test_real_input_line2() {
        let line = "[#####..#] (0,1,4,5,6) (0,1,2,4,5,6) (1,2) (2,3) (2) (5) (0,4,5,7) {33,29,57,14,33,47,20,13}";
        let puzzle = parse_line(line);
        assert_eq!(puzzle.lights.len(), 8);
        assert_eq!(puzzle.buttons.len(), 7);

        let result = solve_joltage(&puzzle);
        eprintln!("Real input line 2 result: {}", result);
        assert!(result < usize::MAX);
    }

    #[test]
    fn test_real_input_sample() {
        let result = part2(REAL_INPUT_SAMPLE);
        eprintln!("Real input sample (4 lines) total: {}", result);
        assert!(result > 0);
    }
}
