#[derive(Debug, Clone, Copy)]
struct Hailstone {
    px: i64,
    py: i64,
    pz: i64,
    vx: i64,
    vy: i64,
    vz: i64,
}

fn parse_input(input: &str) -> Vec<Hailstone> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let (pos, vel) = line.split_once('@').unwrap();
            let pos_parts: Vec<i64> = pos
                .split(',')
                .map(|s| s.trim().parse().unwrap())
                .collect();
            let vel_parts: Vec<i64> = vel
                .split(',')
                .map(|s| s.trim().parse().unwrap())
                .collect();

            Hailstone {
                px: pos_parts[0],
                py: pos_parts[1],
                pz: pos_parts[2],
                vx: vel_parts[0],
                vy: vel_parts[1],
                vz: vel_parts[2],
            }
        })
        .collect()
}

fn check_intersection(h1: &Hailstone, h2: &Hailstone, min: f64, max: f64) -> bool {
    // Convert to f64 for intersection calculation
    let px1 = h1.px as f64;
    let py1 = h1.py as f64;
    let vx1 = h1.vx as f64;
    let vy1 = h1.vy as f64;

    let px2 = h2.px as f64;
    let py2 = h2.py as f64;
    let vx2 = h2.vx as f64;
    let vy2 = h2.vy as f64;

    // Calculate denominator (cross product of velocity vectors)
    let denom = vx1 * vy2 - vy1 * vx2;

    // Lines are parallel if denominator is 0
    if denom.abs() < 1e-10 {
        return false;
    }

    // Calculate difference in starting positions
    let dx = px2 - px1;
    let dy = py2 - py1;

    // Calculate time parameters for both hailstones
    let t1 = (dx * vy2 - dy * vx2) / denom;
    let t2 = (dx * vy1 - dy * vx1) / denom;

    // Check if intersection is in the future for both hailstones
    if t1 < 0.0 || t2 < 0.0 {
        return false;
    }

    // Calculate intersection point
    let x = px1 + vx1 * t1;
    let y = py1 + vy1 * t1;

    // Check if within bounds
    x >= min && x <= max && y >= min && y <= max
}

pub fn part1(input: &str) -> u32 {
    part1_with_bounds(input, 200000000000000.0, 400000000000000.0)
}

fn part1_with_bounds(input: &str, min: f64, max: f64) -> u32 {
    let hailstones = parse_input(input);
    let mut count = 0;

    // Check all pairs of hailstones
    for i in 0..hailstones.len() {
        for j in i + 1..hailstones.len() {
            if check_intersection(&hailstones[i], &hailstones[j], min, max) {
                count += 1;
            }
        }
    }

    count
}

pub fn part2(input: &str) -> u32 {
    let hailstones = parse_input(input);
    // TODO: implement part 2 logic
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
";

    #[test]
    fn test_part1() {
        assert_eq!(part1_with_bounds(EXAMPLE, 7.0, 27.0), 2);
    }

    #[test]
    #[ignore]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 47);
    }
}
