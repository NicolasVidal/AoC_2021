pub fn _p1(s: &str) -> usize {
    let mut stack = Vec::new();

    let mut total = 0;
    for line in s.lines() {
        stack.clear();
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                _ => match (stack.pop(), c) {
                    (Some('('), ')') => {}
                    (Some('['), ']') => {}
                    (Some('{'), '}') => {}
                    (Some('<'), '>') => {}
                    (_, ')') => {
                        total += 3;
                        break;
                    }
                    (_, ']') => {
                        total += 57;
                        break;
                    }
                    (_, '}') => {
                        total += 1197;
                        break;
                    }
                    (_, '>') => {
                        total += 25137;
                        break;
                    }
                    (_, _) => panic!()
                }
            }
        }
    }
    total
}

pub fn p1() -> usize {
    _p1(include_str!("j10.txt"))
}

pub fn _p2(s: &str) -> usize {
    let mut stack = Vec::new();

    let mut totals = Vec::new();
    'next_line: for line in s.lines() {
        let mut subtotal = 0usize;
        stack.clear();
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                _ => match (stack.pop(), c) {
                    (Some('('), ')') => {}
                    (Some('['), ']') => {}
                    (Some('{'), '}') => {}
                    (Some('<'), '>') => {}
                    (_, ')') => { continue 'next_line; }
                    (_, ']') => { continue 'next_line; }
                    (_, '}') => { continue 'next_line; }
                    (_, '>') => { continue 'next_line; }
                    (_, _) => panic!()
                }
            }
        }
        if stack.is_empty() {
            continue;
        }

        while let Some(c) = stack.pop() {
            match c {
                '(' => {
                    subtotal *= 5;
                    subtotal += 1
                }
                '[' => {
                    subtotal *= 5;
                    subtotal += 2
                }
                '{' => {
                    subtotal *= 5;
                    subtotal += 3
                }
                '<' => {
                    subtotal *= 5;
                    subtotal += 4
                }
                _ => panic!(),
            }
        }
        totals.push(subtotal)
    }
    totals.sort();
    totals[totals.len() / 2]
}

pub fn p2() -> usize {
    _p2(include_str!("j10.txt"))
}

#[cfg(test)]
mod j10_tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(26397, _p1(include_str!("j10_test.txt")));
        assert_eq!(268845, _p1(include_str!("j10.txt")));
    }

    #[test]
    fn test_p2() {
        assert_eq!(288957, _p2(include_str!("j10_test.txt")));
        assert_eq!(4038824534, _p2(include_str!("j10.txt")));
    }
}