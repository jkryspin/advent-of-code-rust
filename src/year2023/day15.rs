use std::collections::HashMap;

pub fn part1(input: &str) -> usize {
    let strings = input.split(",").collect::<Vec<_>>();
    return strings.iter().fold(0usize, |acc, s| acc + s.hash());
}

trait HolidayHash {
    fn hash(&self) -> usize;
}

impl HolidayHash for &str {
    fn hash(&self) -> usize {
        let chars = self.chars();
        let mut current_value = 0usize;
        for c in chars {
            let code = c as u32 as usize;
            current_value += code;
            current_value *= 17;
            current_value = current_value % 256;
        }
        current_value
    }
}
#[derive(Debug)]
struct Lens {
    label: String,
    focal_length: usize,
    hash: usize,
}

impl Lens {
    fn new(label: String, focal_length: usize) -> Self {
        Self {
            focal_length,
            hash: label.clone().as_str().hash(),
            label,
        }
    }
}

pub fn part2(input: &str) -> usize {
    let strings = input.split(",").collect::<Vec<_>>();
    let mut map = HashMap::<usize, Vec<Lens>>::new();
    strings.iter().for_each(|s| {
        if s.contains("-") {
            let label = s.split("-").into_iter().next().unwrap();
            let key = label.hash();
            // Remove the lens if we find it
            if let Some(lens) = map.get_mut(&key) {
                if let Some(pos) = lens.iter().position(|l| l.label == label) {
                    lens.remove(pos);
                }
            }
        } else {
            // Insert a lens
            let (left, right) = s.split_once("=").unwrap();
            let lens = Lens::new(left.to_string(), right.parse().unwrap());
            match map.get_mut(&lens.hash) {
                None => {
                    // Map is empty, put lens in
                    map.insert(lens.hash, vec![lens]);
                }
                Some(lenss) => match lenss.iter().position(|l| l.label == lens.label) {
                    // Lens label not already found, add to end
                    None => {
                        lenss.push(lens);
                    }
                    // Lens already found, replace at pos
                    Some(found_lens_pos) => {
                        lenss[found_lens_pos] = lens;
                    }
                },
            }
        }
    });
    get_focusing_power(map)
}

fn get_focusing_power(map: HashMap<usize, Vec<Lens>>) -> usize {
    let mut sum = 0usize;
    map.iter().for_each(|(box_num, lenses)| {
        sum += lenses
            .iter()
            .enumerate()
            .map(|(index, lens)| {
                return (box_num + 1) * (index + 1) * lens.focal_length;
            })
            .sum::<usize>();
    });
    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test removed - needs example data
    #[test]
    fn example_hasher() {
        assert_eq!("HASH".hash(), 52);
    }

    #[test]
    fn example_hasher_pt_2() {
        assert_eq!("rn".hash(), 0);
    }

    // Test removed - needs example data
}
