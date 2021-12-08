use std::str::FromStr;

fn compute_optimal_crab_move(s: &str, distance: fn(i32, i32) ->i32) -> usize {
    let mut crabs_positions = Vec::new();

    for f in s.lines().next().unwrap().split(',')
        .map(|num| i32::from_str(num).unwrap()) {
        crabs_positions.push(f);
    }

    let mut min = i32::MAX;
    for p in 0.. {
        let sum_moves = crabs_positions.iter().map(|c|distance(*c,p)).sum();
        if sum_moves < min {
            min = sum_moves;
        }
        else {
            break
        }
    }

    min as usize
}

pub fn _p1(s: &str) -> usize {
    compute_optimal_crab_move(s, |c, p|(c-p).abs())
}

pub fn p1() -> usize {
    _p1(include_str!("j7.txt"))
}

pub fn _p2(s: &str) -> usize {
    compute_optimal_crab_move(s, |c, p|(0..=(c-p).abs()).sum())
}

pub fn p2() -> usize {
    _p2(include_str!("j7.txt"))
}

#[cfg(test)]
mod j7_tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(37, _p1(include_str!("j7_test.txt")));
        assert_eq!(344138, _p1(include_str!("j7.txt")));
    }

    #[test]
    fn test_p2() {
        assert_eq!(168, _p2(include_str!("j7_test.txt")));
        assert_eq!(94862124, _p2(include_str!("j7.txt")));
    }
}