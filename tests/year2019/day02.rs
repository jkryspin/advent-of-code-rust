use aoc::year2019::day02::*;

const EXAMPLE: &str = "1,1,1,4,99,5,6,0,99";

#[test]
fn part1_test() {
    assert_eq!(part1(EXAMPLE), 30);
}

#[test]
fn part2_test() {
    assert_eq!(part2(EXAMPLE), 1202);
}
