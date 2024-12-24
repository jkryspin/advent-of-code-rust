use aoc::year2019::day05::*;

const EXAMPLE: &str = "3,0,4,0,99";

#[test]
fn part1_test() {
    assert_eq!(part1(EXAMPLE), 1);
}

#[test]
fn part1_test_2() {
    assert_eq!(part1("1101,100,-1,4,0"), 1);
}

#[test]
fn part2_test() {
    assert_eq!(part2(EXAMPLE), 0);
}
