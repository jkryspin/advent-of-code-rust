pub fn part1(input: &str) -> u32 {
    input.lines().map(|line| {
        let mass: u32 = line.parse().unwrap();
        mass / 3 - 2
    }).sum()
}
pub fn part2(input: &str) -> u32 {
    input.lines().map(|line| {
        let mass: u32 = line.parse().unwrap();
        let mut fuel = 0;
        let mut remaining_mass = mass;
        loop {
            let fuel_needed = (remaining_mass / 3).saturating_sub(2);
            if fuel_needed == 0 {
                break;
            }
            fuel += fuel_needed;
            remaining_mass = fuel_needed;
        }
        fuel
    }).sum()
}
