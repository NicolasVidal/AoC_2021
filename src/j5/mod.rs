use std::str::FromStr;
use std::sync::Arc;
use std::sync::mpsc::{channel, Sender};
use dashmap::DashMap;
use threadpool::ThreadPool;

fn parse_group(s: &str) -> (usize, usize) {
    let mut splitted = s.split(',');
    (usize::from_str(splitted.next().unwrap()).unwrap(),
     usize::from_str(splitted.next().unwrap()).unwrap())
}

fn parse_and_extract_move_info_from_line(line: &'static str) -> (usize, usize, i32, i32, i32, i32) {
    let mut groups = line.split_whitespace();
    let (x1, y1) = parse_group(groups.next().unwrap());
    let _ = groups.next().unwrap();
    let (x2, y2) = parse_group(groups.next().unwrap());

    let diff_x = x2 as i32 - x1 as i32;
    let step_diff_x = diff_x.signum();
    let abs_diff_x = diff_x.abs();
    let diff_y = y2 as i32 - y1 as i32;
    let step_diff_y = diff_y.signum();
    let abs_diff_y = diff_y.abs();
    (x1, y1, step_diff_x, abs_diff_x, step_diff_y, abs_diff_y)
}

fn update_cells_and_transmit(covered_positions: Arc<DashMap<(usize, usize), i32>>,
                             tx: Sender<()>,
                             x1: usize,
                             y1: usize,
                             step_diff_x: i32,
                             abs_diff_x: i32,
                             step_diff_y: i32,
                             abs_diff_y: i32) {
    let (mut x, mut y) = (x1 as i32, y1 as i32);
    for _ in 0..=abs_diff_x.max(abs_diff_y) {
        let mut v = covered_positions.entry((x as usize, y as usize)).or_insert(0);
        let v = v.value_mut();
        *v += 1;
        if *v == 2 {
            tx.send(()).unwrap();
        }
        x += step_diff_x;
        y += step_diff_y;
    }
}

pub fn _p1(s: &'static str) -> usize {
    let covered_positions = Arc::new(DashMap::new());
    let mut at_least_two_overlaps = 0;

    let n_workers = 8;
    let pool = ThreadPool::new(n_workers);

    let (tx, rx) = channel();

    for line in s.lines() {
        let tx = tx.clone();
        let covered_positions = covered_positions.clone();

        pool.execute(move || {
            let (x1, y1, step_diff_x, abs_diff_x, step_diff_y, abs_diff_y) =
                parse_and_extract_move_info_from_line(line);
            if abs_diff_x == 0 || abs_diff_y == 0 {
                update_cells_and_transmit(covered_positions,
                                          tx,
                                          x1,
                                          y1,
                                          step_diff_x,
                                          abs_diff_x,
                                          step_diff_y,
                                          abs_diff_y)
            }
        });
    }
    drop(tx);

    while let Ok(_) = rx.recv() {
        at_least_two_overlaps += 1;
    }

    at_least_two_overlaps
}

pub fn p1() -> usize {
    _p1(include_str!("j5.txt"))
}

pub fn _p2(s: &'static str) -> usize {
    let covered_positions = Arc::new(DashMap::new());
    let mut at_least_two_overlaps = 0;

    let n_workers = 8;
    let pool = ThreadPool::new(n_workers);

    let (tx, rx) = channel();

    for line in s.lines() {
        let tx = tx.clone();
        let covered_positions = covered_positions.clone();

        pool.execute(move || {
            let (x1, y1, step_diff_x, abs_diff_x, step_diff_y, abs_diff_y) =
                parse_and_extract_move_info_from_line(line);

            if abs_diff_x == abs_diff_y || abs_diff_x == 0 || abs_diff_y == 0 {
                update_cells_and_transmit(covered_positions,
                                          tx,
                                          x1,
                                          y1,
                                          step_diff_x,
                                          abs_diff_x,
                                          step_diff_y,
                                          abs_diff_y)
            }
        });
    }
    drop(tx);

    while let Ok(_) = rx.recv() {
        at_least_two_overlaps += 1;
    }

    at_least_two_overlaps
}

pub fn p2() -> usize {
    _p2(include_str!("j5.txt"))
}

#[cfg(test)]
mod j5_tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(5, _p1(include_str!("j5_test.txt")));
        assert_eq!(5608, _p1(include_str!("j5.txt")));
    }

    #[test]
    fn test_p2() {
        assert_eq!(12, _p2(include_str!("j5_test.txt")));
        assert_eq!(20299, _p2(include_str!("j5.txt")));
    }
}