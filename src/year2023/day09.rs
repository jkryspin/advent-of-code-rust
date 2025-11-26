
pub fn part1(input: &str) -> i32 {
    let mut sum = 0i32;
    input.lines().for_each(|l| {
        let mut curr = l
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let mut ending: Vec<i32> = vec![];
        ending.push(curr.last().unwrap().clone());
        while curr.iter().any(|&x| x != 0) {
            let mut new: Vec<i32> = vec![];
            let mut i = 0;
            while i < curr.len() - 1 {
                new.push(curr[i + 1] - curr[i]);
                i += 1;
            }
            curr = new.clone();
            ending.push(new.last().unwrap().clone());
        }
        sum += ending.iter().sum::<i32>();
    });
    sum
}

pub fn part2(input: &str) -> i32 {
    let mut sum = 0i32;
    input.lines().for_each(|l| {
        let mut curr = l
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let mut ending: Vec<i32> = vec![];
        ending.push(curr.first().unwrap().clone());
        while curr.iter().any(|&x| x != 0) {
            let mut new: Vec<i32> = vec![];
            curr.windows(2).for_each(|w| {
                new.push(w[1] - w[0]);
            });
            curr = new.clone();
            ending.push(new.first().unwrap().clone());
        }
        let mut prev = 0;
        let new_ending = ending
            .iter()
            .rev()
            .map(|x| {
                let n = x - prev;
                prev = n;
                n
            })
            .last()
            .unwrap();
        sum += new_ending
    });
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test removed - needs example data

    // Test removed - needs example data
}
