use std::collections::HashSet;
use std::str::FromStr;
use itertools::Itertools;

#[derive(Default)]
struct BingoCell {
    number: usize,
    validated: bool,
}

#[derive(Default)]
struct BingoGrid {
    grid: [[BingoCell; 5]; 5],
}

impl BingoGrid {
    pub fn line_completed(&self) -> bool {
        for row in &self.grid {
            if row.iter().all(|c| c.validated) {
                return true;
            }
        }
        return false;
    }

    pub fn column_completed(&self) -> bool {
        for col in 0..5 {
            if (0..5).all(|row| self.grid[row][col].validated) {
                return true;
            }
        }
        return false;
    }

    pub fn mark_number(&mut self, num: usize) {
        for row in 0..5 {
            for col in 0..5 {
                if self.grid[row][col].number == num {
                    self.grid[row][col].validated = true;
                }
            }
        }
    }
}


pub fn _p1(s: &str) -> usize {
    let mut lines = s.lines();

    let number_order = lines.next().unwrap().split(',').map(|num_str| usize::from_str(num_str).unwrap()).collect_vec();

    let mut grids = Vec::new();

    while let Some(empty_line) = lines.next() {
        assert!(empty_line.is_empty());

        let mut grid = BingoGrid::default();
        for row in 0..5 {
            let mut row_it = lines.next().unwrap().split_whitespace().map(|num_str| usize::from_str(num_str).unwrap());
            for col in 0..5 {
                grid.grid[row][col] = BingoCell {
                    number: row_it.next().unwrap(),
                    validated: false,
                }
            }
        }
        grids.push(grid);
    }

    for num in number_order {
        for grid in grids.iter_mut() {
            grid.mark_number(num);
            if grid.line_completed() || grid.column_completed() {
                let sum_unmarked: usize = grid.grid.iter().flat_map(|line| line.iter()).filter(|c|!c.validated).map(|c|c.number).sum();
                return num * sum_unmarked
            }
        }
    }
    panic!()
}

pub fn p1() -> usize {
    _p1(include_str!("j4.txt"))
}

pub fn _p2(s: &str) -> usize {
    let mut lines = s.lines();

    let number_order = lines.next().unwrap().split(',').map(|num_str| usize::from_str(num_str).unwrap()).collect_vec();

    let mut grids = Vec::new();

    while let Some(empty_line) = lines.next() {
        assert!(empty_line.is_empty());

        let mut grid = BingoGrid::default();
        for row in 0..5 {
            let mut row_it = lines.next().unwrap().split_whitespace().map(|num_str| usize::from_str(num_str).unwrap());
            for col in 0..5 {
                grid.grid[row][col] = BingoCell {
                    number: row_it.next().unwrap(),
                    validated: false,
                }
            }
        }
        grids.push(grid);
    }

    let mut validated_grids = HashSet::new();
    let mut last_validated_grid_and_number = (None, None);

    let grids_count = grids.len();

    'outer: for num in number_order {
        for (grid_id, grid) in grids.iter_mut().enumerate() {
            grid.mark_number(num);
            if !validated_grids.contains(&grid_id)  && (grid.line_completed() || grid.column_completed()) {
                validated_grids.insert(grid_id);
                last_validated_grid_and_number = (Some(grid_id), Some(num));
                if validated_grids.len() == grids_count {
                    break 'outer;
                }
            }
        }
    }

    let last_grid = &grids[last_validated_grid_and_number.0.unwrap()];
    let sum_unmarked: usize = last_grid.grid.iter().flat_map(|line| line.iter()).filter(|c|!c.validated).map(|c|c.number).sum();
    sum_unmarked * last_validated_grid_and_number.1.unwrap()
}

pub fn p2() -> usize {
    _p2(include_str!("j4.txt"))
}

#[cfg(test)]
mod j4_tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(4512, _p1(include_str!("j4_test.txt")));
        assert_eq!(6592, _p1(include_str!("j4.txt")));
    }

    #[test]
    fn test_p2() {
        assert_eq!(1924, _p2(include_str!("j4_test.txt")));
        assert_eq!(31755, _p2(include_str!("j4.txt")));
    }
}