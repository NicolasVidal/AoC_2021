use std::collections::HashMap;
use std::str::FromStr;

fn parse_group(s: &str) -> (usize, usize) {
    let mut splitted = s.split(',');
    (usize::from_str(splitted.next().unwrap()).unwrap(),
     usize::from_str(splitted.next().unwrap()).unwrap())
}

pub fn _p1(s: &str) -> usize {
    let mut covered_positions = HashMap::new();
    let mut at_least_two_overlaps = 0;
    for line in s.lines() {
        let mut groups = line.split_whitespace();
        let (x1, y1) = parse_group(groups.next().unwrap());
        let _ = groups.next().unwrap();
        let (x2, y2) = parse_group(groups.next().unwrap());

        if x1 == x2 {
            for i in if y1 <= y2 { y1..=y2 } else { y2..=y1 } {
                let v = covered_positions.entry((x1, i)).or_insert(0);
                *v += 1;
                if *v == 2 {
                    at_least_two_overlaps += 1;
                }
            }
        }

        if y1 == y2 {
            for i in if x1 <= x2 { x1..=x2 } else { x2..=x1 } {
                let v = covered_positions.entry((i, y1)).or_insert(0);
                *v += 1;
                if *v == 2 {
                    at_least_two_overlaps += 1;
                }
            }
        }
    }

    at_least_two_overlaps
}

pub fn p1() -> usize {
    _p1(include_str!("j5.txt"))
}

pub fn _p2(s: &str) -> usize {
    let mut covered_positions = HashMap::new();
    let mut at_least_two_overlaps = 0;
    for line in s.lines() {
        let mut groups = line.split_whitespace();
        let (x1, y1) = parse_group(groups.next().unwrap());
        let _ = groups.next().unwrap();
        let (x2, y2) = parse_group(groups.next().unwrap());

        if x1 == x2 {
            for i in if y1 <= y2 { y1..=y2 } else { y2..=y1 } {
                let v = covered_positions.entry((x1, i)).or_insert(0);
                *v += 1;
                if *v == 2 {
                    at_least_two_overlaps += 1;
                }
            }
        }

        if y1 == y2 {
            for i in if x1 <= x2 { x1..=x2 } else { x2..=x1 } {
                let v = covered_positions.entry((i, y1)).or_insert(0);
                *v += 1;
                if *v == 2 {
                    at_least_two_overlaps += 1;
                }
            }
        }

        let diff_x = x2 as i32 - x1 as i32;
        let step_diff_x = diff_x.signum();
        let abs_diff_x = diff_x.abs();
        let diff_y = y2 as i32 - y1 as i32;
        let step_diff_y = diff_y.signum();
        let abs_diff_y = diff_y.abs();
        if abs_diff_x == abs_diff_y {
            let (mut x, mut y) = (x1 as i32, y1 as i32);
            for _ in 0..=abs_diff_x {
                let v = covered_positions.entry((x as usize, y as usize)).or_insert(0);
                *v += 1;
                if *v == 2 {
                    at_least_two_overlaps += 1;
                }
                x += step_diff_x;
                y += step_diff_y;
            }
        }
    }

    // for y in 0..10 {
    //     for x in 0..10 {
    //         let ent = covered_positions.entry((x, y)).or_insert(0);
    //         print!("{}", match *ent {
    //             0 => format!("."),
    //             i => format!("{}", i)
    //         })
    //     }
    //     println!();
    // }

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