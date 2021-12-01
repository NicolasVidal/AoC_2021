use std::str::FromStr;

use itertools::Itertools;

pub fn _p1(s: &str) -> usize {
    s.lines()
        .map(i32::from_str)
        .map(Result::unwrap)
        .tuple_windows()
        .filter(|(prev, next)| prev < next)
        .count()
}

pub fn p1() -> usize {
    _p1(include_str!("j1.txt"))
}

pub fn _p2(s: &str) -> usize {
    s.lines()
        .map(i32::from_str)
        .map(Result::unwrap)
        .tuple_windows::<(_, _, _, _)>()
        .filter(|(v0, v1, v2, v3)| v0 + v1 + v2 < v1 + v2 + v3)
        .count()
}

pub fn p2() -> usize {
    _p2(include_str!("j1.txt"))
}

#[cfg(test)]
mod j1_tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(7, _p1(include_str!("j1_test.txt")))
    }

    #[test]
    fn test_p2() {
        assert_eq!(5, _p2(include_str!("j1_test.txt")))
    }
}