pub fn part_1(input: &str) -> u32 {
    let mut grid: Vec<Vec<bool>> = Vec::new();
    for line in input.lines() {
        grid.push(line.chars().map(|x| x == '@').collect::<Vec<bool>>());
    }

    let mut total = 0;

    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if !grid[row][col] {
                continue;
            }

            let mut adjacent_rolls = 0;

            if col > 0 && row > 0 && grid[row - 1][col - 1] {
                adjacent_rolls += 1;
            }

            if row > 0 && grid[row - 1][col + 0] {
                adjacent_rolls += 1;
            }

            if col < grid[row].len() - 1 && row > 0 && grid[row - 1][col + 1] {
                adjacent_rolls += 1;
            }

            if col > 0 && grid[row + 0][col - 1] {
                adjacent_rolls += 1;
            }

            if col < grid[row].len() - 1 && grid[row + 0][col + 1] {
                adjacent_rolls += 1;
            }

            if col > 0 && row < grid.len() - 1 && grid[row + 1][col - 1] {
                adjacent_rolls += 1;
            }

            if row < grid.len() - 1 && grid[row + 1][col + 0] {
                adjacent_rolls += 1;
            }

            if col < grid[row].len() - 1 && row < grid.len() - 1 && grid[row + 1][col + 1] {
                adjacent_rolls += 1;
            }

            if adjacent_rolls < 4 {
                total += 1;
            }
        }
    }

    total
}

pub fn part_2(input: &str) -> u32 {
    let mut grid: Vec<Vec<bool>> = Vec::new();
    for line in input.lines() {
        grid.push(line.chars().map(|x| x == '@').collect::<Vec<bool>>());
    }

    let mut total = 0;

    loop {
        let mut to_be_removed: Vec<(usize, usize)> = Vec::new();

        for row in 0..grid.len() {
            for col in 0..grid[row].len() {
                if !grid[row][col] {
                    continue;
                }

                let mut adjacent_rolls = 0;
                
                if col > 0 && row > 0 && grid[row - 1][col - 1] {
                    adjacent_rolls += 1;
                }

                if row > 0 && grid[row - 1][col + 0] {
                    adjacent_rolls += 1;
                }

                if col < grid[row].len() - 1 && row > 0 && grid[row - 1][col + 1] {
                    adjacent_rolls += 1;
                }

                if col > 0 && grid[row + 0][col - 1] {
                    adjacent_rolls += 1;
                }

                if col < grid[row].len() - 1 && grid[row + 0][col + 1] {
                    adjacent_rolls += 1;
                }

                if col > 0 && row < grid.len() - 1 && grid[row + 1][col - 1] {
                    adjacent_rolls += 1;
                }

                if row < grid.len() - 1 && grid[row + 1][col + 0] {
                    adjacent_rolls += 1;
                }

                if col < grid[row].len() - 1 && row < grid.len() - 1 && grid[row + 1][col + 1] {
                    adjacent_rolls += 1;
                }

                if adjacent_rolls < 4 {
                    to_be_removed.push((row, col));
                }
            }
        }   

        if to_be_removed.is_empty() {
            break;
        } else {
            total += to_be_removed.len() as u32;

            for (row, col) in to_be_removed {
                grid[row][col] = false;
            }
        }
    }

    total
}