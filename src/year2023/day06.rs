pub fn part1(input: &str) -> u64 {
    let mut lines = input.lines();
    let times = parse_part_one(lines.next().unwrap());
    let records = parse_part_one(lines.next().unwrap());
    let total = (0..times.len())
        .into_iter()
        .map(|i| Race {
            time: times[i],
            record: records[i],
        })
        .map(|r| r.ways_to_win())
        .reduce(|acc, e| acc * e)
        .unwrap();

    total
}

pub fn part2(input: &str) -> u64 {
    let mut lines = input.lines();
    let time = parse_part_two(lines.next().unwrap());
    let distance = parse_part_two(lines.next().unwrap());
    let r = Race {
        time,
        record: distance,
    };
    r.ways_to_win()
}

struct Race {
    time: u64,
    record: u64,
}

impl Race {
    fn ways_to_win(&self) -> u64 {
        (0..self.time)
            .into_iter()
            .reduce(|acc, held_time| {
                let traveled = held_time * (self.time - held_time);
                if traveled > self.record {
                    acc + 1
                } else {
                    acc
                }
            })
            .unwrap()
    }
}

fn parse_part_one(line: &str) -> Vec<u64> {
    let times = line
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect::<Vec<u64>>();
    return times;
}

fn parse_part_two(line: &str) -> u64 {
    let times = line
        .split_whitespace()
        .skip(1)
        .collect::<Vec<&str>>()
        .join("")
        .parse::<u64>()
        .unwrap();
    return times;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let result = part1(input);
        assert_eq!(result, 288);
    }

    #[test]
    fn test_part2() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let result = part2(input);
        assert_eq!(result, 71503);
    }
}
