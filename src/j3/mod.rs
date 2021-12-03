pub fn _p1(s: &str) -> usize {
    let mut gamma = 0;
    let mut power = 0;
    let mut zeros_counts = Vec::new();
    let mut ones_counts = Vec::new();
    for line in s.lines() {
        for (i, c) in line.chars().enumerate() {
            if zeros_counts.len() == i {
                zeros_counts.push(0);
            }
            if ones_counts.len() == i {
                ones_counts.push(0);
            }
            match c {
                '1' => zeros_counts[i] += 1,
                '0' => ones_counts[i] += 1,
                _ => panic!()
            }
        }
    }

    for (i, (zeros, ones)) in zeros_counts.iter().zip(ones_counts)
        .rev().enumerate() {
        match *zeros >= ones {
            true => {
                gamma += 2usize.pow(i as u32) * 0;
                power += 2usize.pow(i as u32) * 1;
            }
            false => {
                gamma += 2usize.pow(i as u32) * 1;
                power += 2usize.pow(i as u32) * 0;
            }
        }
    }
    gamma * power
}

pub fn p1() -> usize {
    _p1(include_str!("j3.txt"))
}

pub fn _p2(s: &str) -> usize {
    let mut values = Vec::new();

    for (i, line) in s.lines().enumerate() {
        values.push(Vec::new());
        for c in line.chars() {
            values[i].push(c)
        }
    }

    let char_count = values[0].len();

    fn string_to_usize(s: String) -> usize {
        s.chars().rev().enumerate().fold(0usize, |ox, (i, c)| if c == '1' {
            ox + 2usize.pow(i as u32)
        } else { ox })
    }

    fn compute_zeros_count_ones_count_and_last_char(s: &String, values: &Vec<Vec<char>>, i: usize) -> (usize, usize, Option<char>) {
        values.iter().fold((0usize, 0usize, None), |(zeros, ones, last_char), line|
            match (s.chars().zip(line.iter()).all(|(a, b)| a == *b), line[i]) {
                (false, _) => (zeros, ones, last_char),
                (_, '0') => (zeros + 1, ones, Some('0')),
                (_, '1') => (zeros, ones + 1, Some('1')),
                (_, _) => panic!()
            })
    }

    let oxygen =
        string_to_usize((0..char_count).fold(String::new(), |mut s, i| {
            match compute_zeros_count_ones_count_and_last_char(&s, &values, i) {
                (_, _, None) => panic!(),
                (zeros, ones, Some(c)) if zeros + ones == 1 => s.push(c),
                (zeros, ones, _) if zeros == ones => s.push('1'),
                (zeros, ones, _) if zeros > ones => s.push('0'),
                (zeros, ones, _) if zeros < ones => s.push('1'),
                _ => panic!()
            };
            s
        }));

    let co2 =
        string_to_usize((0..char_count).fold(String::new(), |mut s, i| {
            match compute_zeros_count_ones_count_and_last_char(&s, &values, i) {
                (_, _, None) => panic!(),
                (zeros, ones, Some(c)) if zeros + ones == 1 => s.push(c),
                (zeros, ones, _) if zeros == ones => s.push('0'),
                (zeros, ones, _) if zeros < ones => s.push('0'),
                (zeros, ones, _) if zeros > ones => s.push('1'),
                _ => panic!()
            };
            s
        }));

    oxygen * co2
}

pub fn p2() -> usize {
    _p2(include_str!("j3.txt"))
}

#[cfg(test)]
mod j3_tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(198, _p1(include_str!("j3_test.txt")));
        assert_eq!(2640986, _p1(include_str!("j3.txt")));
    }

    #[test]
    fn test_p2() {
        assert_eq!(230, _p2(include_str!("j3_test.txt")));
        assert_eq!(6822109, _p2(include_str!("j3.txt")));
    }
}