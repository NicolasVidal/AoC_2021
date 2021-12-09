use std::collections::HashMap;

use itertools::Itertools;

fn compute_matrix_height_width_lower_points_risks(s: &str) -> (Vec<Vec<u8>>, usize, usize, Vec<(usize, usize)>, usize) {
    let matrix = s.lines().into_iter().map(|line|
        line.chars().into_iter().map(|c| c as u8 - 48u8).collect_vec()
    ).collect_vec();
    let height = matrix.len();
    let width = matrix[0].len();

    let mut lower_points = Vec::new();
    let mut risks = 0usize;
    for i in 0usize..height {
        for j in 0usize..width {
            let mut min = u8::MAX;

            if i > 0 {
                min = min.min(matrix[i - 1][j])
            }
            if j > 0 {
                min = min.min(matrix[i][j - 1])
            }
            if i < height - 1 {
                min = min.min(matrix[i + 1][j])
            }
            if j < width - 1 {
                min = min.min(matrix[i][j + 1])
            }

            if matrix[i][j] < min {
                lower_points.push((i, j));
                risks += (matrix[i][j] as usize) + 1;
            }
        }
    }

    (matrix, height, width, lower_points, risks)
}

pub fn _p1(s: &str) -> usize {
    compute_matrix_height_width_lower_points_risks(s).4
}

pub fn p1() -> usize {
    _p1(include_str!("j9.txt"))
}

pub fn _p2(s: &str) -> usize {
    let (matrix, height, width, lower_points, _) =
        compute_matrix_height_width_lower_points_risks(s);

    let mut basin_sizes = lower_points.iter().map(|_| 1usize).collect_vec();
    let mut low_point_basins = HashMap::new();

    for i in 0..height {
        for j in 0..width {
            if low_point_basins.contains_key(&(i, j)) {
                continue;
            }

            let mut path = Vec::new();

            let (mut row, mut col) = (i, j);

            while !lower_points.contains(&(row, col)) {
                if matrix[row][col] < 9 {
                    path.push((row, col))
                }

                let mut lowest = (row, col);
                let mut lowest_score = u8::MAX;
                if row > 0 && matrix[row - 1][col] < lowest_score {
                    lowest = (row - 1, col);
                    lowest_score = matrix[row - 1][col];
                }
                if col > 0 && matrix[row][col - 1] < lowest_score {
                    lowest = (row, col - 1);
                    lowest_score = matrix[row][col - 1];
                }
                if row < height - 1 && matrix[row + 1][col] < lowest_score {
                    lowest = (row + 1, col);
                    lowest_score = matrix[row + 1][col];
                }
                if col < width - 1 && matrix[row][col + 1] < lowest_score {
                    lowest = (row, col + 1);
                }
                row = lowest.0;
                col = lowest.1;
            }

            for p in path {
                low_point_basins.insert(p, (row, col));
            }
        }
    }

    for (idx, &(row, col)) in lower_points.iter().enumerate() {
        for (_, &(a, b)) in low_point_basins.iter() {
            if row == a && col == b {
                basin_sizes[idx] += 1
            }
        }
    }

    basin_sizes.sort();

    basin_sizes[basin_sizes.len() - 3..].iter().product()
}

pub fn p2() -> usize {
    _p2(include_str!("j9.txt"))
}

#[cfg(test)]
mod j9_tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(15, _p1(include_str!("j9_test.txt")));
        assert_eq!(504, _p1(include_str!("j9.txt")));
    }

    #[test]
    fn test_p2() {
        assert_eq!(1134, _p2(include_str!("j9_test.txt")));
        assert_eq!(1558722, _p2(include_str!("j9.txt")));
    }
}