use std::ops::Range;
use std::str::FromStr;
use regex::Regex;

fn simulate_shoot(x: i64, y: i64, target_x_range: &Range<i64>, target_y_range: &Range<i64>) -> (bool, i64) {
    let mut current_x = 0;
    let mut current_y = 0;

    let mut speed_x = x;
    let mut speed_y = y;

    let mut max_y = i64::MIN;

    for _ in 0.. {
        current_x += speed_x;
        current_y += speed_y;

        max_y = max_y.max(current_y);

        speed_x -= 1;
        speed_x = speed_x.max(0);

        speed_y -= 1;

        if target_x_range.contains(&current_x) &&
            target_y_range.contains(&current_y) {
            return (true, max_y);
        }

        if target_x_range.end < current_x {
            return (false, max_y);
        }

        if target_y_range.start > current_y {
            return (false, max_y);
        }
    }
    panic!();
}


fn find_optimal_shoot(s: &str) -> usize {
    let (x_range, y_range) = get_target_zone_range(s);

    for y in ((y_range.start - 1)..1000).rev() {
        for x in 0..x_range.end + 1 {
            let (hit, max_y) = simulate_shoot(x, y, &x_range, &y_range);
            if hit {
                return max_y as usize
            }
        }
    }
    panic!()
}

fn get_target_zone_range(s: &str) -> (Range<i64>, Range<i64>) {
    let re =
        Regex::new(r"x=(-?[0-9]+)\.\.(-?[0-9]+), y=(-?[0-9]+)\.\.(-?[0-9]+)")
            .unwrap();

    let captures = re.captures(s).unwrap();
    let x_range = (i64::from_str(&captures[1])).unwrap()..(i64::from_str(&captures[2]).unwrap() + 1);
    let y_range = (i64::from_str(&captures[3])).unwrap()..(i64::from_str(&captures[4]).unwrap() + 1);
    (x_range, y_range)
}


fn find_all_valid_shoots(s: &str) -> usize {
    let (x_range, y_range) = get_target_zone_range(s);

    let mut valid_shoots_count = 0usize;
    for x in 0..x_range.end + 1 {
        for y in (y_range.start - 1)..1000 {
            let (hit, _max_y) = simulate_shoot(x, y, &x_range, &y_range);
            if hit {
                valid_shoots_count += 1;
            }
        }
    }
    valid_shoots_count
}


pub fn _p1(s: &'static str) -> usize {
    find_optimal_shoot(s)
}

pub fn p1() -> usize {
    _p1(include_str!("j17.txt"))
}

pub fn _p2(s: &'static str) -> usize {
    find_all_valid_shoots(s)
}

pub fn p2() -> usize {
    _p2(include_str!("j17.txt"))
}

#[cfg(test)]
mod j17_tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(45, _p1(include_str!("j17_test.txt")));
        assert_eq!(11781, _p1(include_str!("j17.txt")));
    }

    #[test]
    fn test_p2() {
        assert_eq!(112, _p2(include_str!("j17_test.txt")));
        assert_eq!(4531, _p2(include_str!("j17.txt")));
    }
}