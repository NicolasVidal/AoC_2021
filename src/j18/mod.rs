use std::collections::VecDeque;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
enum Symbol {
    Open,
    Close,
    Comma,
    Num(u64),
}

impl From<char> for Symbol {
    fn from(c: char) -> Self {
        match c {
            '[' => Symbol::Open,
            ']' => Symbol::Close,
            ',' => Symbol::Comma,
            c if ('0'..='9').contains(&c) => Symbol::Num((c as u8 - 48) as u64),
            _ => panic!()
        }
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Symbol::Open => write!(f, "["),
            Symbol::Close => write!(f, "]"),
            Symbol::Comma => write!(f, ","),
            Symbol::Num(n) => write!(f, "{}", n)
        }
    }
}

impl Symbol {
    fn num(&self) -> u64 {
        match self {
            Symbol::Open | Symbol::Close | Symbol::Comma => panic!(),
            Symbol::Num(n) => *n
        }
    }
}

fn reduce(v: &mut VecDeque<Symbol>) {
    let mut copy: VecDeque<Symbol> = VecDeque::with_capacity(v.len());
    'top: loop {
        copy.clear();
        let mut depth = 0usize;
        for (i, c) in v.iter().enumerate() {
            match (depth, c) {
                (4, Symbol::Open) => {
                    let left_val = v[i + 1].num();
                    let right_val = v[i + 3].num();
                    let copy_initial_len = copy.len();
                    for j in (0..copy_initial_len).rev() {
                        match copy[j] {
                            Symbol::Num(n) => {
                                copy[j] = Symbol::Num(n + left_val);
                                break;
                            }
                            Symbol::Open | Symbol::Close | Symbol::Comma => {}
                        }
                    }
                    copy.push_back(Symbol::Num(0));
                    let mut added = false;
                    for c in v.iter().skip(i + 5) {
                        match c {
                            Symbol::Num(n) if !added => {
                                copy.push_back(Symbol::Num(n + right_val));
                                added = true;
                            }
                            c => {
                                copy.push_back(c.clone());
                            }
                        }
                    }
                    v.clear();
                    v.clone_from(&copy);
                    continue 'top;
                }
                (_, Symbol::Open) => depth += 1,
                (_, Symbol::Close) => depth -= 1,
                (_, Symbol::Comma) => {}
                (_, _) => {}
            }
            copy.push_back(c.clone());
        }
        v.clear();
        v.clone_from(&copy);
        copy.clear();
        for (i, c) in v.iter().enumerate() {
            match (depth, c) {
                (_, Symbol::Open) => depth += 1,
                (_, Symbol::Close) => depth -= 1,
                (_, Symbol::Comma) => {}
                (_, Symbol::Num(n)) if (0u64..=9u64).contains(&n) => {}
                (_, Symbol::Num(n)) if *n > 9 => {
                    copy.push_back(Symbol::Open);
                    copy.push_back(Symbol::Num((*n as f64 / 2.0).floor() as u64));
                    copy.push_back(Symbol::Comma);
                    copy.push_back(Symbol::Num((*n as f64 / 2.0).ceil() as u64));
                    copy.push_back(Symbol::Close);
                    for c in v.iter().skip(i + 1) {
                        copy.push_back(c.clone());
                    }
                    v.clear();
                    v.clone_from(&copy);
                    continue 'top;
                }
                (_, c) => {
                    dbg!(&c);
                    panic!()
                }
            }
            copy.push_back(c.clone());
        }
        v.clear();
        v.clone_from(&copy);
        break;
    }
}

fn compute_magnitude(v: &mut VecDeque<Symbol>) -> u64 {
    match v.pop_front().unwrap() {
        Symbol::Open => {
            let left_magnitude = compute_magnitude(v);
            v.pop_front();
            let right_magnitude = compute_magnitude(v);
            v.pop_front();
            3 * left_magnitude + 2 * right_magnitude
        }
        Symbol::Close => panic!(),
        Symbol::Comma => panic!(),
        Symbol::Num(n) => {
            n
        }
    }
}

pub fn _p1(s: &'static str) -> u64 {
    let mut v: VecDeque<Symbol> = VecDeque::new();
    for line in s.lines() {
        if !v.is_empty() {
            add_snail_fish(&mut v, line);
        } else {
            line_to_snail_fish(&mut v, line)
        }
        reduce(&mut v);
    }
    compute_magnitude(&mut v)
}

fn add_snail_fish(mut v: &mut VecDeque<Symbol>, line: &str) {
    v.push_front('['.into());
    v.push_back(','.into());
    line_to_snail_fish(&mut v, line);
    v.push_back(']'.into());
}

fn line_to_snail_fish(v: &mut VecDeque<Symbol>, line: &str) {
    for c in line.chars() {
        v.push_back(c.into());
    }
}

pub fn p1() -> u64 {
    _p1(include_str!("j18.txt"))
}

pub fn _p2(s: &'static str) -> u64 {
    let mut v: VecDeque<Symbol> = VecDeque::new();

    let mut largest_magnitude = u64::MIN;
    for (i, line1) in s.lines().enumerate() {
        for line2 in s.lines().skip(i + 1) {
            v.clear();
            line_to_snail_fish(&mut v, line1);
            add_snail_fish(&mut v, line2);
            reduce(&mut v);
            largest_magnitude = largest_magnitude.max(compute_magnitude(&mut v));

            v.clear();
            line_to_snail_fish(&mut v, line2);
            add_snail_fish(&mut v, line1);
            reduce(&mut v);
            largest_magnitude = largest_magnitude.max(compute_magnitude(&mut v));
        }
    }

    largest_magnitude
}

pub fn p2() -> u64 {
    _p2(include_str!("j18.txt"))
}

#[cfg(test)]
mod j18_tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(4140, _p1(include_str!("j18_test.txt")));
        assert_eq!(3524, _p1(include_str!("j18.txt")));
    }

    #[test]
    fn test_p2() {
        assert_eq!(3993, _p2(include_str!("j18_test.txt")));
        assert_eq!(4656, _p2(include_str!("j18.txt")));
    }
}