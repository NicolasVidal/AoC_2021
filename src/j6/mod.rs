mod naive_solution;

use std::str::FromStr;

fn count_lantern_fish_smart(s: &'static str, days: usize) -> usize {
    let mut lantern_fish = [0usize; 9];

    for f in s.lines().next().unwrap().split(',')
        .map(|num| usize::from_str(num).unwrap()) {
        lantern_fish[f] += 1;
    }

    for _d in 0..days {
        let mut new_lantern_fish_count = [0usize; 9];
        for n in 1..=8 {
            new_lantern_fish_count[n - 1] += lantern_fish[n];
        }
        new_lantern_fish_count[6] += lantern_fish[0];
        new_lantern_fish_count[8] += lantern_fish[0];
        lantern_fish = new_lantern_fish_count;
    }

    lantern_fish.iter().sum()
}

pub fn _p1(s: &'static str) -> usize {
    let days = 80;
    count_lantern_fish_smart(s, days)
}

pub fn p1() -> usize {
    _p1(include_str!("j6.txt"))
}

pub fn _p2(s: &'static str) -> usize {
    let days = 256;
    count_lantern_fish_smart(s, days)
}

pub fn p2() -> usize {
    _p2(include_str!("j6.txt"))
}

#[cfg(test)]
mod j6_tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(5934, _p1(include_str!("j6_test.txt")));
        assert_eq!(355386, _p1(include_str!("j6.txt")));
    }

    #[test]
    fn test_p2() {
        assert_eq!(26984457539, _p2(include_str!("j6_test.txt")));
        assert_eq!(1613415325809, _p2(include_str!("j6.txt")));
    }
}