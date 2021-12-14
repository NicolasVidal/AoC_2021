use std::collections::HashMap;

use itertools::{Itertools, MinMaxResult};

fn apply_patterns_and_get_result(s: &str, iter: usize) -> usize {
    let mut start_pattern = None;
    let mut expand_rules = HashMap::<(char, char), char>::new();
    let mut counts_rules = HashMap::<((char, char), usize), HashMap<char, usize>>::new();
    for (i, line) in s.lines().enumerate() {
        if i == 0 {
            start_pattern = Some(line);
            continue;
        }
        if i == 1 {
            continue;
        }
        let mut rule_split = line.split("->");
        let mut map = HashMap::new();
        let key = rule_split.next().unwrap().trim().chars().collect_tuple::<(char, char)>().unwrap();
        let v = rule_split.next().unwrap().trim().chars().next().unwrap();
        map.insert(v, 1usize);
        expand_rules.insert(
            key,
            v,
        );
        counts_rules.insert(
            (key, 0usize),
            map,
        );
    }

    for i in 0..(iter-1) {
        for ((c1, c2), c) in expand_rules.iter() {
            let mut map = HashMap::new();

            fill_map(&mut counts_rules, i, *c1, *c, &mut map);

            fill_map(&mut counts_rules, i, *c, *c2, &mut map);

            (*map.entry(*c).or_insert(0usize)) += 1;

            counts_rules.insert(((*c1, *c2), i + 1), map);
        }
    }


    let mut map = HashMap::new();

    let start_pattern = start_pattern.unwrap();
    for c in start_pattern.chars() {
        (*map.entry(c).or_insert(0usize)) += 1;
    }

    for (c1, c2) in start_pattern.chars().tuple_windows() {
        for (k, v) in counts_rules.get(&((c1, c2), iter-1)).unwrap() {
            (*map.entry(*k).or_insert(0usize)) += v;
        }
    }

    match map.into_iter().map(|(_, count)| count).minmax() {
        MinMaxResult::NoElements => { panic!() }
        MinMaxResult::OneElement(_) => { panic!() }
        MinMaxResult::MinMax(min, max) => { max - min }
    }
}

fn fill_map(counts_rules: &mut HashMap<((char, char), usize), HashMap<char, usize>>, i: usize, c1: char, c: char, map: &mut HashMap<char, usize>) {
    for (k, v) in counts_rules.get(&((c1, c), i)).unwrap() {
        (*map.entry(*k).or_insert(0usize)) += v;
    }
}


pub fn _p1(s: &str) -> usize {
    apply_patterns_and_get_result(s, 10)
}

pub fn p1() -> usize {
    _p1(include_str!("j14.txt"))
}

pub fn _p2(s: &str) -> usize {
    apply_patterns_and_get_result(s, 40)
}

pub fn p2() -> usize {
    _p2(include_str!("j14.txt"))
}

#[cfg(test)]
mod j14_tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(1588, _p1(include_str!("j14_test.txt")));
        assert_eq!(2233, _p1(include_str!("j14.txt")));
    }

    #[test]
    fn test_p2() {
        assert_eq!(2188189693529, _p2(include_str!("j14_test.txt")));
        assert_eq!(2884513602164, _p2(include_str!("j14.txt")));
    }
}