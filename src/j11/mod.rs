fn increase_energy(grid: &mut [[u32; 10]; 10]) {
    for row in 0..10 {
        for col in 0..10 {
            grid[row][col] += 1;
        }
    }
}

fn parse_grid(s: &str) -> [[u32; 10]; 10] {
    let mut grid = [[0u32; 10]; 10];

    s.lines().enumerate().flat_map(|(row, line)|
        line.chars().enumerate().map(move |(col, c)| (row, col, c))
    ).for_each(|(row, col, c)| {
        grid[row][col] = c as u32 - 48;
    });
    grid
}

fn get_flash_count_and_update_cells(grid: &mut [[u32; 10]; 10]) -> usize {
    let mut flashed = [[false; 10]; 10];
    let mut total_flashes = 0usize;
    loop {
        let mut flash_count = 0usize;
        for row in 0..10 {
            for col in 0..10 {
                if grid[row][col] > 9 && !flashed[row][col] {
                    flash_count += 1;
                    flashed[row][col] = true;
                    if row > 0 {
                        grid[row - 1][col] += 1;
                    }
                    if col > 0 {
                        grid[row][col - 1] += 1;
                    }
                    if row < 9 {
                        grid[row + 1][col] += 1;
                    }
                    if col < 9 {
                        grid[row][col + 1] += 1;
                    }
                    if row > 0 && col > 0 {
                        grid[row - 1][col - 1] += 1;
                    }
                    if row > 0 && col < 9 {
                        grid[row - 1][col + 1] += 1;
                    }
                    if row < 9 && col > 0 {
                        grid[row + 1][col - 1] += 1;
                    }
                    if row < 9 && col < 9 {
                        grid[row + 1][col + 1] += 1;
                    }
                }
            }
        }
        total_flashes += flash_count;
        if flash_count == 0 {
            break;
        }
    }
    total_flashes
}

fn reset_flashed_cells(grid: &mut [[u32; 10]; 10]) {
    for row in 0..10 {
        for col in 0..10 {
            if grid[row][col] > 9 {
                grid[row][col] = 0;
            }
        }
    }
}

pub fn _p1(s: &str) -> usize {
    let mut grid = parse_grid(s);

    let mut total_flashes = 0usize;
    for _step in 0..100 {
        increase_energy(&mut grid);

        total_flashes += get_flash_count_and_update_cells(&mut grid);
        reset_flashed_cells(&mut grid);
    }

    total_flashes
}

pub fn p1() -> usize {
    _p1(include_str!("j11.txt"))
}

pub fn _p2(s: &str) -> usize {
    let mut grid = parse_grid(s);

    for _step in 0.. {
        increase_energy(&mut grid);

        let total_flashes = get_flash_count_and_update_cells(&mut grid);

        reset_flashed_cells(&mut grid);
        if total_flashes == 100 {
            return _step + 1;
        }
    }
    panic!()
}

pub fn p2() -> usize {
    _p2(include_str!("j11.txt"))
}

#[cfg(test)]
mod j11_tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(1656, _p1(include_str!("j11_test.txt")));
        assert_eq!(1634, _p1(include_str!("j11.txt")));
    }

    #[test]
    fn test_p2() {
        assert_eq!(195, _p2(include_str!("j11_test.txt")));
        assert_eq!(210, _p2(include_str!("j11.txt")));
    }
}