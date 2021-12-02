use std::str::FromStr;

pub fn _p1(s: &str) -> usize {
    let (final_x, final_y) = s.lines()
        .fold((0usize, 0usize), |(x, y), line| {
            let mut it = line.split_whitespace();
            let command = it.next().unwrap();
            let value = usize::from_str(it.next().unwrap()).unwrap();
            match command {
                "forward" => (x + value, y),
                "down" => (x, y + value),
                "up" => (x, y - value),
                &_ => panic!("Unknown Command")
            }
        });
    final_x * final_y
}

pub fn p1() -> usize {
    _p1(include_str!("j2.txt"))
}

pub fn _p2(s: &str) -> usize {
    let (_, final_x, final_y) = s.lines()
        .fold((0usize, 0usize, 0usize), |(aim, x, y), line| {
            let mut it = line.split_whitespace();
            let command = it.next().unwrap();
            let value = usize::from_str(it.next().unwrap()).unwrap();
            match command {
                "forward" => (aim, x + value, y + aim * value),
                "down" => (aim + value, x, y),
                "up" => (aim - value, x, y),
                &_ => panic!("Unknown Command")
            }
        });
    final_x * final_y
}

pub fn p2() -> usize {
    _p2(include_str!("j2.txt"))
}

#[cfg(test)]
mod j1_tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(150, _p1(include_str!("j2_test.txt")))
    }

    #[test]
    fn test_p2() {
        assert_eq!(900, _p2(include_str!("j2_test.txt")))
    }
}