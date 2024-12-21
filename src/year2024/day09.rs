
pub fn part1(input: &str) -> i128 {
    let mut output = parse_input(input);
    process_output_part1(&mut output);
    calculate_sum(&output)
}

pub fn part2(input: &str) -> i128 {
    let mut output = parse_input(input);
    process_output_part2(&mut output);
    calculate_sum(&output)
}

fn parse_input(input: &str) -> Vec<i128> {
    let mut output = Vec::new();
    let mut i = 0;
    for c in input.lines().next().unwrap().chars().map(|c| c.to_digit(10).unwrap()) {
        if i % 2 == 0 {
            for _ in 0..c {
                output.push(i / 2);
            }
        } else {
            for _ in 0..c {
                output.push(-1);
            }
        }
        i += 1;
    }
    output
}

fn process_output_part1(output: &mut Vec<i128>) {
    let mut i = 0;
    while i < output.len() {
        if output[i] == -1 {
            if let Some(pos) = output.iter_mut().rposition(|x| *x != -1) {
                if pos < i {
                    output.retain(|&x| x != -1);
                    break;
                }
                output[i] = output.remove(pos);
            } else {
                panic!("no more values");
            }
        }
        i += 1;
    }
}

fn process_output_part2(output: &mut Vec<i128>) {
    let mut i = output.len() - 1;
    while i > 0 {
        if output[i] == -1 {
            i -= 1;
            continue;
        }

        let block_len = output
            .iter()
            .rev()
            .skip(output.len() - i)
            .take_while(|&&x| x == output[i])
            .count()
            + 1;

        let mut j = 0;
        while j < i {
            if output[j] == -1 {
                let free_space_len = output.iter().skip(j).take_while(|&&x| x == -1).count();
                if free_space_len >= block_len {
                    for k in 0..block_len {
                        output[j + k] = output[i];
                    }
                    for k in 0..block_len {
                        output[i - k] = -1;
                    }
                    break;
                }
            }
            j += 1;
        }
        i = i.saturating_sub(block_len)
    }
}

fn calculate_sum(output: &Vec<i128>) -> i128 {
    output
        .iter()
        .enumerate()
        .filter(|(_, &x)| x != -1)
        .map(|(i, &c)| i as i128 * c)
        .sum()
}