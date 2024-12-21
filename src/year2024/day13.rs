
pub fn part1(input: &str) -> u64 {
    let mut sum = 0;
    for block in input.split("\n\n") {
        let mut claw = Claw::from(block);
        sum += claw.cheapest_cost(0, true);
    }
    sum
}

pub fn part2(input: &str) -> u64 {
    let mut sum: u64 = 0;
    for block in input.split("\n\n") {
        let mut claw = Claw::from(block);
        sum += claw.cheapest_cost(10000000000000, false);
    }
    sum
}

#[derive(Debug)]
struct Claw {
    button_a: (f64, f64),
    button_b: (f64, f64),
    prize: (f64, f64),
}

impl Claw {
    fn cheapest_cost(&mut self, offset: u64, limit: bool) -> u64 {
        self.prize.0 += offset as f64;
        self.prize.1 += offset as f64;
        let determinant = self.button_a.0 * self.button_b.1 - self.button_a.1 * self.button_b.0;

        let n1 = (self.prize.0 * self.button_b.1 - self.prize.1 * self.button_b.0) / determinant;

        let n2 = (self.prize.1 * self.button_a.0 - self.prize.0 * self.button_a.1) / determinant;

        if n1.fract() == 0.0 && n2.fract() == 0.0 {
            if limit {
                if (0..100).contains(&(n1 as u64)) && (0..100).contains(&(n2 as u64)) {
                    return 3 * n1 as u64 + n2 as u64;
                }
                unreachable!("Invalid input");
            } else {
                return 3 * n1 as u64 + n2 as u64;
            }
        }
        0
    }
}
impl From<&str> for Claw {
    fn from(value: &str) -> Self {
        let re = regex::Regex::new(r"X\+(\d+), Y\+(\d+)").unwrap();
        let lines = value.lines().collect::<Vec<_>>();
        let button_a = re.captures(lines[0]).unwrap();
        let button_b = re.captures(lines[1]).unwrap();
        let re2 = regex::Regex::new(r"X=(\d+), Y=(\d+)").unwrap();
        let prize = re2.captures(lines[2]).unwrap();

        Self {
            button_a: (button_a[1].parse().unwrap(), button_a[2].parse().unwrap()),
            button_b: (button_b[1].parse().unwrap(), button_b[2].parse().unwrap()),
            prize: (prize[1].parse().unwrap(), prize[2].parse().unwrap()),
        }
    }
}
