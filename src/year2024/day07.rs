
pub fn part1(input: &str) -> u64 {
        input
            .lines()
            .map(Calibration::from)
            .filter(|c| c.is_valid(false))
            .map(|c| c.test_value)
            .sum()
}

pub fn part2(input: &str) -> u64 {
        input
            .lines()
            .map(Calibration::from)
            .filter(|c| c.is_valid(true))
            .map(|c| c.test_value)
            .sum()
}

#[derive(Debug)]
struct Calibration {
    test_value: u64,
    operands: Vec<u64>,
}

impl From<&str> for Calibration {
    fn from(s: &str) -> Self {
        let (left, right) = s.split_once(": ").unwrap();
        let test_value = left.parse().unwrap();
        let operands = right
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        Self {
            test_value,
            operands,
        }
    }
}

impl Calibration {
    fn is_valid(&self, use_concat_operator: bool) -> bool {
        let mut iter = self.operands.iter();
        let mut calculated_results = vec![iter.next().unwrap().clone()];
        for o in iter {
            let results = calculated_results.drain(..).collect::<Vec<u64>>();
            for r in results {
                if r > self.test_value {
                    continue;
                }
                calculated_results.push(o + r);
                calculated_results.push(o * r);
                if use_concat_operator {
                    calculated_results.push(format!("{}{}", r, o).parse().unwrap());
                }
            }
        }
        calculated_results.contains(&&self.test_value)
    }
}
