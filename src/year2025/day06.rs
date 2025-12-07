//! # Day 6: [Title]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation {
    Add,
    Multiply,
}

#[derive(Debug)]
struct Column {
    numbers: Vec<i64>,
    operation: Operation,
}

fn parse_input(input: &str) -> Vec<Column> {
    let lines: Vec<&str> = input.lines().collect();

    if lines.is_empty() {
        return vec![];
    }

    // Last line contains operations
    let ops_line = lines[lines.len() - 1];
    let number_lines = &lines[..lines.len() - 1];

    // Parse operations
    let operations: Vec<Operation> = ops_line
        .chars()
        .filter_map(|ch| match ch {
            '+' => Some(Operation::Add),
            '*' => Some(Operation::Multiply),
            _ => None,
        })
        .collect();

    // Parse numbers from each line (whitespace-separated)
    let rows: Vec<Vec<i64>> = number_lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|s| s.parse::<i64>().ok())
                .collect()
        })
        .collect();

    // Transpose: convert rows to columns
    if rows.is_empty() || operations.is_empty() {
        return vec![];
    }

    let num_cols = operations.len();
    let mut columns = Vec::new();

    for col_idx in 0..num_cols {
        let mut numbers = Vec::new();
        for row in &rows {
            if col_idx < row.len() {
                numbers.push(row[col_idx]);
            }
        }
        if !numbers.is_empty() {
            columns.push(Column {
                numbers,
                operation: operations[col_idx],
            });
        }
    }

    columns
}

pub fn part1(input: &str) -> usize {
    let columns = parse_input(input);
    let mut sum: i64 = 0;
    for col in columns {
        let result: i64 = match col.operation {
            Operation::Add => col.numbers.into_iter().reduce(|a, b| a + b).unwrap(),
            Operation::Multiply => col.numbers.into_iter().reduce(|a, b| a * b).unwrap(),
        };
        sum += result;
    }
    sum as usize
}

pub fn part2(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();

    if lines.is_empty() {
        return 0;
    }

    // Find the maximum line length to know how far right we need to go
    let max_len = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    let mut sum: i64 = 0;
    let mut current_numbers: Vec<i64> = Vec::new();

    // Process columns from RIGHT to LEFT
    for col_idx in (0..max_len).rev() {
        let mut digits = String::new();
        let mut operation: Option<Operation> = None;

        // Read this column from TOP to BOTTOM
        for line in &lines {
            if col_idx < line.len() {
                let ch = line.chars().nth(col_idx).unwrap();

                if ch.is_ascii_digit() {
                    digits.push(ch);
                } else if ch == '+' {
                    operation = Some(Operation::Add);
                } else if ch == '*' {
                    operation = Some(Operation::Multiply);
                }
            }
        }

        // If we collected digits, parse them into a number
        if !digits.is_empty() {
            if let Ok(num) = digits.parse::<i64>() {
                current_numbers.push(num);
            }
        }

        // If we found an operation, apply it to all collected numbers
        if let Some(op) = operation {
            if !current_numbers.is_empty() {
                let result = match op {
                    Operation::Add => current_numbers.iter().sum::<i64>(),
                    Operation::Multiply => current_numbers.iter().product::<i64>(),
                };
                sum += result;
                current_numbers.clear();
            }
        }
    }

    sum as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_parse() {
        let columns = parse_input(EXAMPLE);
        assert_eq!(columns.len(), 4);
        for (i, col) in columns.iter().enumerate() {
            println!("Column {}: {:?} with op {:?}", i, col.numbers, col.operation);
        }
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 4277556);
    }

    #[test]
    fn test_part2_debug() {
        let columns = parse_input(EXAMPLE);
        for (i, col) in columns.iter().enumerate() {
            println!("Column {}: {:?} with op {:?}", i, col.numbers, col.operation);
        }

        let result = part2(EXAMPLE);
        println!("Part 2 result: {}", result);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 3263827);
    }
}
