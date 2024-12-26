use aoc::year2024::day25::*;

const EXAMPLE: &str = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";

#[test]
fn part1_test() {
    assert_eq!(part1(EXAMPLE), 3);
}



// #[test]
// fn part2_test() {
//     assert_eq!(part2(EXAMPLE), "co,de,ka,ta");
// }

// #[test]
// fn part_3_test() {
//     part3();
// }

