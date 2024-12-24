use crate::year2019::intcode::IntCode;

pub fn part1(input: &str) -> i64 {
    let l = input.lines().next().unwrap();
    let mut intcode = IntCode::new(l);
    intcode.input = 1;
    intcode.start();
    println!("{:?}", intcode.output);
    intcode.output.unwrap()
}

pub fn part2(input: &str) -> u32 {
    let l = input.lines().next().unwrap();
    let mut intcode = IntCode::new(l);
    intcode.input = 5;
    intcode.start();
    println!("{:?}", intcode.output);
    intcode.output.unwrap() as u32
}