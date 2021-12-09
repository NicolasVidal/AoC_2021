use std::collections::HashMap;

use itertools::Itertools;

fn compute_matrix_height_width(s: &str) -> (Vec<Vec<u8>>, usize, usize) {
    let matrix = s.lines().into_iter().map(|line|
        line.chars().into_iter().map(|c| c as u8 - 48u8).collect_vec()
    ).collect_vec();
    let height = matrix.len();
    let width = matrix[0].len();

    (matrix, height, width)
}

pub fn _p1(s: &str) -> usize {
    let (matrix, height, width) = compute_matrix_height_width(s);

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
                risks += (matrix[i][j] as usize) + 1;
            }
        }
    }
    risks
}

pub fn p1() -> usize {
    _p1(include_str!("j9.txt"))
}

pub fn _p2(s: &str) -> usize {
    let (matrix, height, width) = compute_matrix_height_width(s);

    let mut basin_sizes = HashMap::new();
    let mut low_point_basins = HashMap::new();

    for i in 0..height {
        for j in 0..width {
            if low_point_basins.contains_key(&(i, j)) {
                continue;
            }

            let mut path = Vec::new();

            let (mut row, mut col) = (i, j);

            loop {
                if let Some(&(row, col)) = low_point_basins.get(&(row, col)) {
                    for p in path {
                        low_point_basins.insert(p, (row, col));
                    }
                    break;
                }

                if matrix[row][col] < 9 {
                    path.push((row, col))
                }

                let mut lowest = (row, col);
                let mut lowest_score = matrix[row][col];
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

                if (row, col) == lowest {
                    for p in path {
                        low_point_basins.insert(p, (row, col));
                    }
                    break;
                }

                row = lowest.0;
                col = lowest.1;
            }
        }
    }

    for (_, v) in low_point_basins.iter() {
        *basin_sizes.entry(*v).or_insert(0usize) += 1;
    }

    basin_sizes.into_iter()
        .map(|(_, v)| v)
        .sorted()
        .rev()
        .take(3)
        .product()
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