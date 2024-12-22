use aoc::year2024::day22::*;

const EXAMPLE: &str = "1
10
100
2024";

#[test]
fn part1_test() {
    assert_eq!(part1(EXAMPLE), 37327623);
}



#[test]
fn part2_test() {
    assert_eq!(part2("1
2
3
2024"), 23);
}

#[test]
fn gen_sec(){
    assert_eq!(mix_secret(42,15),37);
}
