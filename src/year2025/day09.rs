//! # Day 9: Largest Rectangle

use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone, Copy)]
struct Rectangle {
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
}

impl Rectangle {
    fn from_points(p1: Point, p2: Point) -> Self {
        Rectangle {
            min_x: p1.x.min(p2.x),
            max_x: p1.x.max(p2.x),
            min_y: p1.y.min(p2.y),
            max_y: p1.y.max(p2.y),
        }
    }

    fn area(&self) -> usize {
        let width = (self.max_x - self.min_x + 1) as usize;
        let height = (self.max_y - self.min_y + 1) as usize;
        width * height
    }

    fn corners(&self) -> [Point; 4] {
        [
            Point { x: self.min_x, y: self.min_y },
            Point { x: self.max_x, y: self.min_y },
            Point { x: self.max_x, y: self.max_y },
            Point { x: self.min_x, y: self.max_y },
        ]
    }

    fn edges(&self) -> [(Point, Point); 4] {
        [
            (Point { x: self.min_x, y: self.min_y }, Point { x: self.max_x, y: self.min_y }), // Top
            (Point { x: self.max_x, y: self.min_y }, Point { x: self.max_x, y: self.max_y }), // Right
            (Point { x: self.max_x, y: self.max_y }, Point { x: self.min_x, y: self.max_y }), // Bottom
            (Point { x: self.min_x, y: self.max_y }, Point { x: self.min_x, y: self.min_y }), // Left
        ]
    }
}

fn parse_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<i64> = line.split(',').map(|s| s.trim().parse().unwrap()).collect();
            Point { x: parts[0], y: parts[1] }
        })
        .collect()
}

fn generate_all_rectangles(points: &[Point]) -> Vec<Rectangle> {
    let mut rectangles = Vec::new();
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            rectangles.push(Rectangle::from_points(points[i], points[j]));
        }
    }
    // Sort by area descending
    rectangles.sort_by(|a, b| b.area().cmp(&a.area()));
    rectangles
}

pub fn part1(input: &str) -> usize {
    let points = parse_input(input);
    let rectangles = generate_all_rectangles(&points);
    rectangles[0].area()
}

/// Build a polygon boundary by connecting consecutive points
fn build_polygon_boundary(points: &[Point]) -> HashSet<Point> {
    let mut boundary = HashSet::new();
    let point_set: HashSet<Point> = points.iter().copied().collect();

    for i in 0..points.len() {
        let p1 = points[i];
        let p2 = points[(i + 1) % points.len()];

        let dx = (p2.x - p1.x).signum();
        let dy = (p2.y - p1.y).signum();

        let mut x = p1.x;
        let mut y = p1.y;

        while x != p2.x || y != p2.y {
            let point = Point { x, y };
            if !point_set.contains(&point) {
                boundary.insert(point);
            }
            if x != p2.x { x += dx; }
            if y != p2.y { y += dy; }
        }
    }

    boundary
}

/// Check if two line segments properly cross (not just touch)
fn segments_cross(p1: Point, p2: Point, p3: Point, p4: Point) -> bool {
    fn orientation(a: Point, b: Point, c: Point) -> i64 {
        (c.y - a.y) * (b.x - a.x) - (b.y - a.y) * (c.x - a.x)
    }

    let o1 = orientation(p3, p4, p1);
    let o2 = orientation(p3, p4, p2);
    let o3 = orientation(p1, p2, p3);
    let o4 = orientation(p1, p2, p4);

    // Segments properly cross if orientations are different on both sides
    ((o1 > 0 && o2 < 0) || (o1 < 0 && o2 > 0)) && ((o3 > 0 && o4 < 0) || (o3 < 0 && o4 > 0))
}

/// Check if a point is inside the polygon using ray casting
fn point_inside_polygon(point: Point, polygon: &[Point], boundary: &HashSet<Point>) -> bool {
    // Points on the boundary are considered inside
    if boundary.contains(&point) || polygon.contains(&point) {
        return true;
    }

    // Count crossings with horizontal ray to the right
    let mut crossings = 0;
    for i in 0..polygon.len() {
        let p1 = polygon[i];
        let p2 = polygon[(i + 1) % polygon.len()];

        if (p1.y <= point.y && p2.y > point.y) || (p2.y <= point.y && p1.y > point.y) {
            let x_intersect = p1.x + (point.y - p1.y) * (p2.x - p1.x) / (p2.y - p1.y);
            if point.x < x_intersect {
                crossings += 1;
            }
        }
    }

    crossings % 2 == 1
}

/// Check if a rectangle fits entirely inside the polygon
fn rectangle_fits_in_polygon(rect: &Rectangle, polygon: &[Point], boundary: &HashSet<Point>) -> bool {
    // Check if any rectangle edge crosses any polygon edge
    for &(r1, r2) in &rect.edges() {
        for i in 0..polygon.len() {
            let p1 = polygon[i];
            let p2 = polygon[(i + 1) % polygon.len()];
            if segments_cross(r1, r2, p1, p2) {
                return false;
            }
        }
    }

    // All corners must be inside the polygon
    rect.corners().iter().all(|&corner| point_inside_polygon(corner, polygon, boundary))
}

pub fn part2(input: &str) -> usize {
    let points = parse_input(input);
    let boundary = build_polygon_boundary(&points);
    let rectangles = generate_all_rectangles(&points);

    rectangles
        .iter()
        .find(|rect| rectangle_fits_in_polygon(rect, &points, &boundary))
        .map(|rect| rect.area())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 50);
    }

    #[test]
    fn test_part1_simple_square() {
        let input = "0,0\n10,0\n10,10\n0,10";
        assert_eq!(part1(input), 121); // 11x11 grid
    }

    #[test]
    fn test_part1_rectangle() {
        let input = "0,0\n20,0\n20,5\n0,5";
        assert_eq!(part1(input), 126); // 21x6 grid
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 24);
    }

    #[test]
    fn test_part2_simple_square() {
        let input = "0,0\n10,0\n10,10\n0,10";
        assert_eq!(part2(input), 121); // Entire square fits
    }

    #[test]
    fn test_part2_with_indent() {
        // L-shaped polygon - largest interior rectangle is 11x6 in the bottom section
        let input = "0,0\n10,0\n10,5\n5,5\n5,10\n0,10";
        assert_eq!(part2(input), 66); // 11 x 6 from (0,0) to (10,5)
    }
}
