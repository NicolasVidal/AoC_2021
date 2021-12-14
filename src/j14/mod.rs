use std::collections::HashMap;

use itertools::{Itertools, MinMaxResult};

fn apply_patterns_and_get_result(s: &str, iter: usize) -> usize {
    let mut start_pattern = None;
    let mut rules = HashMap::new();
    for (i, line) in s.lines().enumerate() {
        if i == 0 {
            start_pattern = Some(line);
            continue;
        }
        if i == 1 {
            continue;
        }
        let mut rule_splitted = line.split("->");
        rules.insert(
            rule_splitted.next().unwrap().trim(),
            rule_splitted.next().unwrap().trim(),
        );
    }

    let mut counts = HashMap::new();

    let mut start_pattern = String::from(start_pattern.unwrap());
    let mut result = String::with_capacity(start_pattern.len() * 2);
    for _ in 0..iter {
        for i in 0..(start_pattern.len() - 1) {
            if let Some(c) = rules.get(&start_pattern[i..i + 2]) {
                result.push(start_pattern.chars().nth(i).unwrap());
                result.push(c.chars().next().unwrap());
            } else {
                result.push(start_pattern.chars().nth(i).unwrap());
            }
        }
        result.push(start_pattern.chars().nth(start_pattern.len() - 1).unwrap());
        start_pattern = String::from(result);
        result = String::with_capacity(start_pattern.len() * 2);
    }

    for c in start_pattern.chars() {
        let entry = counts.entry(c).or_insert(0usize);
        *entry += 1;
    }


    match counts.into_iter().map(|(_, count)| count).minmax() {
        MinMaxResult::NoElements => { panic!() }
        MinMaxResult::OneElement(_) => { panic!() }
        MinMaxResult::MinMax(min, max) => { max - min }
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
        assert_eq!(2188189693529, _p2(include_str!("j14.txt")));
    }
}