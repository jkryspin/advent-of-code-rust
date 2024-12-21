
pub fn part1(input: &str) -> u32 {
    let plot = Plot::create(input);
    let mut count = 0;
    for y in 0..plot.items.len() {
        for x in 0..plot.items[y].len() {
            let ways = plot.count_ways(x, y);
            if ways > 0 {
                // println!("Found {} ways at {},{}", ways, x, y);
                count += ways;
            }
        }
    }
    count
}

struct Plot {
    items: Vec<Vec<char>>,
}

impl Plot {
    fn create(input: &str) -> Plot {
        let items = input.lines().map(|l| l.chars().collect()).collect();
        Plot { items }
    }

    fn get_9_cell_grid(&self, x: usize, y: usize) -> Option<Vec<Vec<char>>> {
        if x == 0 || y == 0 || x >= self.items[0].len() - 1 || y >= self.items.len() - 1 {
            return None;
        }

        let mut grid = Vec::new();
        for i in y - 1..=y + 1 {
            let mut row = Vec::new();
            for j in x - 1..=x + 1 {
                row.push(self.items[i][j]);
            }
            grid.push(row);
        }
        Some(grid)
    }

    fn is_valid_grid(grid: &Vec<Vec<char>>) -> bool {
        // M.S
        // .A.
        // M.S
        if grid[0][0] == 'M' && grid[0][2] == 'S' && grid[2][0] == 'M' && grid[2][2] == 'S' {
            return true;
        }
        false
    }

    fn count_x_mas(&self, x: usize, y: usize) -> u32 {
        if self.items[y][x] != 'A' {
            return 0;
        }

        // Get the 9 cells around the current cell
        if let Some(mut grid) = self.get_9_cell_grid(x, y) {
            // check if valid grid in all r rotations
            // rotate grid 4 times
            for _ in 0..4 {
                if Plot::is_valid_grid(&grid) {
                    return 1;
                }
                // rotate grid
                let mut new_grid = vec![vec!['.'; 3]; 3];
                for i in 0..3 {
                    for j in 0..3 {
                        new_grid[j][2 - i] = grid[i][j];
                    }
                }
                grid = new_grid;
            }
        }
        0
    }

    fn count_ways(&self, x: usize, y: usize) -> u32 {
        if self.items[y][x] != 'X' {
            return 0;
        }

        let mut count = 0;

        // Check right
        if x + 3 < self.items[y].len()
            && self.items[y][x..=x + 3].iter().collect::<String>() == "XMAS"
        {
            count += 1;
        }

        // Check down
        if y + 3 < self.items.len()
            && (y..=y + 3).all(|i| self.items[i][x] == "XMAS".chars().nth(i - y).unwrap())
        {
            count += 1;
        }

        // Check left
        if x >= 3 && self.items[y][x - 3..=x].iter().rev().collect::<String>() == "XMAS" {
            count += 1;
        }

        // Check up
        if y >= 3
            && (y - 3..=y)
                .rev()
                .all(|i| self.items[i][x] == "XMAS".chars().nth(y - i).unwrap())
        {
            count += 1;
        }

        // Check diagonal up right
        if x + 3 < self.items[y].len()
            && y >= 3
            && (0..=3).all(|i| self.items[y - i][x + i] == "XMAS".chars().nth(i).unwrap())
        {
            count += 1;
        }

        // Check diagonal up left
        if x >= 3
            && y >= 3
            && (0..=3).all(|i| self.items[y - i][x - i] == "XMAS".chars().nth(i).unwrap())
        {
            count += 1;
        }

        // Check diagonal down right
        if x + 3 < self.items[y].len()
            && y + 3 < self.items.len()
            && (0..=3).all(|i| self.items[y + i][x + i] == "XMAS".chars().nth(i).unwrap())
        {
            count += 1;
        }

        // Check diagonal down left
        if x >= 3
            && y + 3 < self.items.len()
            && (0..=3).all(|i| self.items[y + i][x - i] == "XMAS".chars().nth(i).unwrap())
        {
            count += 1;
        }

        assert!(count < 9);
        count
    }
}

pub fn part2(input: &str) -> u32 {
    let plot = Plot::create(input);
    let mut count = 0;
    for y in 0..plot.items.len() {
        for x in 0..plot.items[y].len() {
            let ways = plot.count_x_mas(x, y);
            if ways > 0 {
                // println!("Found {} ways at {},{}", ways, x, y);
                count += ways;
            }
        }
    }
    count
}
