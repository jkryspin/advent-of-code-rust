use crate::year2019::intcode::IntCode;
pub fn part1(input: &str) -> u32 {
    let mut intcode = IntCode::new(input.lines().next().unwrap());
    intcode.memory[1] = 12;
    intcode.memory[2] = 2;
    intcode.start();
    intcode.memory[0] as u32
}
pub fn part2(input: &str) -> u32 {
    for i in 0..100 {
        for j in 0..100 {
            let mut intcode = IntCode::new(input.lines().next().unwrap());
            intcode.memory[1] = i;
            intcode.memory[2] = j;
            intcode.start();
            if intcode.memory[0] == 19690720 {
                return (100 * i + j) as u32;
            }
        }
    }
    return 0;
}
