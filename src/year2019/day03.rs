use std::collections::{HashMap, HashSet};
use crate::util::point::Point;
type Move = (char, i64);

pub fn part1(input: &str) -> i64 {
    let moves = get_moves(input);
    let origin = Point::new(0, 0);

    let lines = get_lines(moves.0);
    let lines2 = get_lines(moves.1);
    let (pos1, _) = get_positions(lines);
    let (pos2, _) = get_positions(lines2);
    pos1
        .intersection(&pos2)
        .map(|p| origin.manhattan_distance(p))
        .filter(|&d| d > 0)
        .min()
        .unwrap() as i64
}

pub fn part2(input: &str) -> u32 {
    let moves = get_moves(input);

    let lines = get_lines(moves.0);
    let lines2 = get_lines(moves.1);
    let (pos1, costs1) = get_positions(lines);
    let (pos2, costs2) = get_positions(lines2);
    pos1
        .intersection(&pos2)
        .map(|p| {
            costs1.get(p).unwrap() + costs2.get(p).unwrap()
        })
        .filter(|&d| d > 0)
        .min()
        .unwrap()

}
fn get_positions(lines:Vec<Line>)-> (HashSet<Point>, HashMap<Point,u32>){
    let mut positions = HashSet::new();
    let mut cost = 0;
    let mut costs = HashMap::new();
    for line in lines {
        let Line(start, end) = line;
        let x = start.x;
        let y = start.y;
        let dx = end.x - start.x;
        let dy = end.y - start.y;
        let steps = dx.abs() + dy.abs();
        for i in 0..steps {
            let x = x + i * dx / steps;
            let y = y + i * dy / steps;
            positions.insert(Point::new(x, y));
            costs.insert(Point::new(x, y), cost);
            cost += 1;
        }
    }
    (positions, costs)
}

struct Line(Point, Point);

fn get_lines(moves:Vec<Move>) -> Vec<Line> {
    let mut x = 0;
    let mut y = 0;
    let mut lines:Vec<Line> = Vec::new();
    for (dir, dist) in moves {
        let start = Point::new(x, y);
        match dir {
            'U' => y -= dist,
            'D' => y += dist,
            'L' => x -= dist,
            'R' => x += dist,
            _ => panic!("Invalid direction"),
        }
        let end = Point::new(x, y);
        lines.push(Line(start, end));
    }
    lines
}

fn get_moves(input: &str) -> (Vec<Move>, Vec<Move>) {
    let mut lines = input.lines();
    (lines
        .next().unwrap()
        .split(',')
        .map(|m| {
            let dir = m.chars().next().unwrap();
            let dist = m[1..].parse().unwrap();
            (dir, dist)
        })
        .collect()
        ,
        lines
         .next().unwrap()
         .split(',')
         .map(|m| {
             let dir = m.chars().next().unwrap();
             let dist = m[1..].parse().unwrap();
             (dir, dist)
         })
         .collect())
}
