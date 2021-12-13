use std::collections::HashSet;
use std::str::FromStr;

fn compute_fold(s: &str, once: bool) -> (HashSet<(usize, usize)>, usize, usize) {
    let mut cols = 0usize;
    let mut rows = 0usize;

    let mut coordinates = HashSet::new();
    let mut lines = s.lines();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let mut chars = line.split(',');
        let col = usize::from_str(chars.next().unwrap()).unwrap();
        let row = usize::from_str(chars.next().unwrap()).unwrap();
        cols = cols.max(col + 1);
        rows = rows.max(row + 1);
        coordinates.insert((row, col));
    }

    while let Some(line) = lines.next() {
        let mut chars = line.split('=').next().unwrap().chars().skip("fold along ".len());
        let letter = chars.next().unwrap();
        let num = usize::from_str(line.split('=').skip(1).next().unwrap()).unwrap();
        match (letter, num) {
            ('y', _) => {
                rows = num + 1;
                let mut to_insert = HashSet::new();
                let mut to_remove = HashSet::new();
                for &(row, col) in coordinates.iter() {
                    if row > num {
                        to_remove.insert((row, col));
                        to_insert.insert((num - (row - num), col));
                    }
                }
                to_remove.into_iter().for_each(|rc| { coordinates.remove(&rc); });
                to_insert.into_iter().for_each(|rc| { coordinates.insert(rc); });
            }
            ('x', _) => {
                cols = num + 1;
                let mut to_insert = HashSet::new();
                let mut to_remove = HashSet::new();
                for &(row, col) in coordinates.iter() {
                    if col > num {
                        to_remove.insert((row, col));
                        to_insert.insert((row, num - (col - num)));
                    }
                }
                to_remove.into_iter().for_each(|rc| { coordinates.remove(&rc); });
                to_insert.into_iter().for_each(|rc| { coordinates.insert(rc); });
            }
            _ => panic!()
        }
        if once {
            break;
        }
    }
    (coordinates, rows, cols)
}

pub fn _p1(s: &str) -> usize {
    let (coordinates, _, _) = compute_fold(s, true);
    coordinates.len()
}

pub fn p1() -> usize {
    _p1(include_str!("j13.txt"))
}

pub fn _p2(s: &str) -> String {
    let (coordinates, rows, cols) = compute_fold(s, false);

    let mut result = String::with_capacity((rows + 1) * (cols + 1));
    for row in 0..rows {
        for col in 0..cols {
            result.push(if coordinates.contains(&(row, col)) { '#' } else { '.' });
        }
        result.push_str("\r\n");
    }

    result
}

pub fn p2() -> String {
    _p2(include_str!("j13.txt"))
}

#[cfg(test)]
mod j13_tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(17, _p1(include_str!("j13_test.txt")));
        assert_eq!(785, _p1(include_str!("j13.txt")));
    }

    #[test]
    fn test_p2() {
        assert_eq!(include_str!("j13_test_result.txt"), _p2(include_str!("j13_test.txt")));
        assert_eq!(include_str!("j13_result.txt"), _p2(include_str!("j13.txt")));
    }
}