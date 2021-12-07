use std::str::FromStr;
use itertools::Itertools;

enum OneOrTwo {
    One(u8),
    Two(u8, u8),
}

struct OneOrTwoIterator {
    v: OneOrTwo,
    current: usize,
}

#[allow(dead_code)]
impl IntoIterator for OneOrTwo {
    type Item = u8;
    type IntoIter = OneOrTwoIterator;

    fn into_iter(self) -> Self::IntoIter {
        OneOrTwoIterator { v: self, current: 0 }
    }
}

#[allow(dead_code)]
impl Iterator for OneOrTwoIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match (&self.v, &self.current) {
            (OneOrTwo::One(c), 0) => Some(*c),
            (OneOrTwo::Two(c, _), 0) => Some(*c),
            (OneOrTwo::Two(_, c), 1) => Some(*c),
            (_, _) => None,
        };
        if result.is_some() {
            self.current += 1;
        }
        result
    }
}

#[allow(dead_code)]
pub fn count_lantern_fish(s: &'static str, days: usize) -> usize {
    let mut old_lantern_fish = s.lines().next().unwrap().split(',')
        .map(|num| u8::from_str(num).unwrap())
        .collect_vec();
    let mut lantern_fish = Vec::new();

    for _day in 0..days {
        for f in old_lantern_fish.drain(..).flat_map(|n| match n {
            0 => OneOrTwo::Two(6, 8),
            c if c >= 1 && c <= 8 => OneOrTwo::One(c - 1),
            _ => panic!()
        }) {
            lantern_fish.push(f);
        }
        let tmp = old_lantern_fish;
        old_lantern_fish = lantern_fish;
        lantern_fish = tmp;
    }

    old_lantern_fish.len()
}
