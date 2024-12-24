use aoc::year2019::day06::*;

const EXAMPLE: &str = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
";

#[test]
fn part1_test() {
    assert_eq!(part1(EXAMPLE), 42);
}

#[test]
fn part2_test() {
    assert_eq!(part2("COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN"), 4);
}
