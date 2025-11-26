struct Map {
    source: u64,
    destination: u64,
    length: u64,
}

impl Map {
    fn get_source(&self, dest: &u64) -> Option<u64> {
        return if (self.destination..(self.destination + self.length)).contains(dest) {
            Some(dest - self.destination + self.source)
        } else {
            None
        };
    }
    fn get_dest(&self, source: u64) -> Option<u64> {
        if (self.source..(self.source + self.length)).contains(&source) {
            return Some(source - self.source + self.destination);
        }
        return None;
    }
}

fn parse_map(s: &str) -> Vec<Map> {
    let lines = s.lines();

    let v = lines
        .skip(1)
        .into_iter()
        .map(|l| {
            let mut split = l.split_whitespace();
            let destination = split.next().unwrap().parse::<u64>().unwrap();
            let source = split.next().unwrap().parse::<u64>().unwrap();
            let length = split.next().unwrap().parse::<u64>().unwrap();
            Map {
                source,
                destination,
                length,
            }
        })
        .collect::<Vec<Map>>();
    return v;
}
pub fn part1(input: &str) -> u64 {
    let mut maps_s = input.split("\n\n");
    let mut init_seeds = maps_s.next().unwrap().split_whitespace();
    init_seeds.next();

    let seeds: Vec<u64> = init_seeds.map(|s| s.parse::<u64>().unwrap()).collect();
    let maps: Vec<Vec<Map>> = maps_s
        .map(|m| {
            return parse_map(m);
        })
        .collect();

    let locations = seeds
        .iter()
        .map(|seed| {
            let mut curr_seed = seed.to_owned();
            maps.iter().for_each(|map| {
                for m in map.iter() {
                    match m.get_dest(curr_seed) {
                        None => {}
                        Some(seed) => {
                            curr_seed = seed;
                            break;
                        }
                    }
                }
            });
            curr_seed
        })
        .collect::<Vec<u64>>();

    locations.iter().min().unwrap().to_owned().to_owned()
}

pub fn part2(input: &str) -> u64 {
    let mut maps_s = input.split("\n\n");
    let mut init_seeds = maps_s.next().unwrap().split_whitespace();
    init_seeds.next();
    let mut x = 0;
    let v_seeds = init_seeds.collect::<Vec<&str>>();
    let mut valid_ranges = vec![];
    while x + 1 < v_seeds.len() {
        let left = v_seeds[x].parse::<u64>().unwrap();
        let right = v_seeds[x + 1].parse::<u64>().unwrap();
        valid_ranges.push(left..=(right + left));
        x += 2;
    }
    let maps: Vec<Vec<Map>> = maps_s
        .map(|m| {
            return parse_map(m);
        })
        .collect();

    let mut seed = 0;
    loop {
        let source = source(seed, &maps);
        for r in valid_ranges.iter() {
            if r.contains(&source) {
                return seed;
            }
        }
        seed += 1;
    }
}

fn source(seed: u64, maps: &Vec<Vec<Map>>) -> u64 {
    let mut s = seed;
    maps.iter().rev().for_each(|v_maps| {
        for m in v_maps.iter() {
            match m.get_source(&s) {
                None => {}
                Some(src) => {
                    s = src;
                    break;
                }
            }
        }
    });
    return s;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let result = part1(input);
        assert_eq!(result, 35);
    }

    #[test]
    #[ignore] // This test takes too long
    fn test_part2() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let result = part2(input);
        assert_eq!(result, 46);
    }
}
