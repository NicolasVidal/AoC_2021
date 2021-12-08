use std::collections::{HashMap};

use itertools::Itertools;

pub fn _p1(s: &str) -> usize {
    s.lines().flat_map(|line|
        line.split('|').skip(1).next().unwrap().split_whitespace()
            .filter(|s| match s.len() {
                2 | 3 | 4 | 7 => true,
                _ => false
            })
    ).count()
}

pub fn p1() -> usize {
    _p1(include_str!("j8.txt"))
}

pub fn _p2(s: &str) -> usize {
    fn position_from_count(count: usize) -> Box<dyn Iterator<Item=usize>> {
        match count {
            8 => Box::new([0, 2].into_iter()),
            6 => Box::new([1].into_iter()),
            7 => Box::new([3, 6].into_iter()),
            4 => Box::new([4].into_iter()),
            9 => Box::new([5].into_iter()),
            _ => panic!(),
        }
    }

    let flags = [
        [1, 1, 1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1, 1, 1],
        [1, 1, 0, 1, 1, 0, 1],
        [0, 1, 0, 1, 1, 0, 1],
        [1, 0, 0, 0, 1, 0, 1],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
    ];

    let flag_to_number = [
        [1, 1, 1, 0, 1, 1, 1],
        [0, 0, 1, 0, 0, 1, 0],
        [1, 0, 1, 1, 1, 0, 1],
        [1, 0, 1, 1, 0, 1, 1],
        [0, 1, 1, 1, 0, 1, 0],
        [1, 1, 0, 1, 0, 1, 1],
        [1, 1, 0, 1, 1, 1, 1],
        [1, 0, 1, 0, 0, 1, 0],
        [1, 1, 1, 1, 1, 1, 1],
        [1, 1, 1, 1, 0, 1, 1],
    ];

    s.lines().map(|line| {
        let mut possibilities = ('a'..='g').map(|c|
            (c, [0; 7])
        ).collect::<HashMap<_, _>>();
        let mut splitted = line.split('|');

        let chunks = splitted.next().unwrap().split_whitespace().collect_vec();

        chunks.iter()
            .for_each(|conf| {
                conf.chars().for_each(|c| {
                    let tab = possibilities.get_mut(&c).unwrap();

                    for i in 0..7 {
                        tab[i] += flags[conf.len()][i];
                    }
                });
            });

        for c in 'a'..='g' {
            let count = chunks.iter()
                .filter(|s| s.contains(|c2| c == c2))
                .count();

            for i in 0..7 {
                if position_from_count(count).contains(&i) {
                    continue;
                }
                possibilities.get_mut(&c).unwrap()[i] += 1;
            }
        }

        while possibilities.iter().any(|(_k, v)| v.iter().filter(|elt| **elt == 0).count() >= 2) {
            for c in 'a'..='g' {
                match possibilities.get(&c).unwrap().iter().filter(|pos| **pos == 0).count() {
                    1 => {
                        for c2 in 'a'..='g' {
                            let source = possibilities.get(&c).unwrap().clone();
                            let target = possibilities.get_mut(&c2).unwrap();
                            // dbg!(&source);
                            // dbg!(&target);
                            for i in 0..7 {
                                if source[i] == 0 && c != c2 {
                                    target[i] += 1;
                                }
                            }
                        }
                    }
                    count if count >= 2 => continue,
                    _ => panic!(),
                }
            }
        }

        line.split('|').skip(1).next().unwrap().split_whitespace()
            .map(|s| {
                let mask = s.chars().fold([0, 0, 0, 0, 0, 0, 0], |mut mask, c| {
                    let idx = possibilities.get(&c).unwrap().iter().enumerate().filter(|(_idx, n)|
                        **n == 0
                    ).next().unwrap().0;
                    mask[idx] = 1;
                    mask
                });
                flag_to_number
                    .iter()
                    .enumerate()
                    .filter(|(_num, m)| m.iter().zip(mask.iter())
                        .all(|(b1, b2)| *b1 == *b2)).next().unwrap().0
            }).rev().enumerate().fold(0, |total, (i, num)|
                total + num as i32 * 10_i32.pow(i as u32)
        )
    }).sum::<i32>() as usize
}

pub fn p2() -> usize {
    _p2(include_str!("j8.txt"))
}

#[cfg(test)]
mod j8_tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(26, _p1(include_str!("j8_test.txt")));
        assert_eq!(344138, _p1(include_str!("j8.txt")));
    }

    #[test]
    fn test_p2() {
        assert_eq!(5353, _p2(include_str!("j8_small_test.txt")));
        assert_eq!(61229, _p2(include_str!("j8_test.txt")));
        assert_eq!(915941, _p2(include_str!("j8.txt")));
    }
}