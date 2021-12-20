fn enhance_and_compute_pixel_count(s: &str, times: usize) -> usize {
    let mut matrix = Vec::new();

    let mut cypher = Vec::new();
    for c in s.lines().next().unwrap().chars() {
        cypher.push(match c {
            '#' => 1usize,
            '.' => 0usize,
            _ => panic!()
        });
    }

    for line in s.lines().skip(2) {
        let mut l = Vec::new();

        for _ in 0..(2 * times) {
            l.push(0usize);
        }

        for c in line.chars() {
            l.push(match c {
                '#' => 1usize,
                '.' => 0usize,
                _ => panic!()
            });
        }
        for _ in 0..(2 * times) {
            l.push(0usize);
        }

        matrix.push(l);
    }

    let cols = matrix[0].len();

    for _ in 0..(2 * times) {
        matrix.insert(0, (0..cols).map(|_| 0usize).collect());
        matrix.push((0..cols).map(|_| 0usize).collect());
    }

    let lines = matrix[0].len();

    let mut new_matrix = matrix.clone();

    for _ in 0..times {
        matrix = new_matrix.clone();
        for l in 1..(lines - 1) {
            for c in 1..(cols - 1) {
                let index = matrix[l - 1][c - 1] * 2usize.pow(8) +
                    matrix[l - 1][c] * 2usize.pow(7) +
                    matrix[l - 1][c + 1] * 2usize.pow(6) +
                    matrix[l][c - 1] * 2usize.pow(5) +
                    matrix[l][c] * 2usize.pow(4) +
                    matrix[l][c + 1] * 2usize.pow(3) +
                    matrix[l + 1][c - 1] * 2usize.pow(2) +
                    matrix[l + 1][c] * 2usize.pow(1) +
                    matrix[l + 1][c + 1] * 2usize.pow(0);
                new_matrix[l][c] = cypher[index]
            }
        }
    }

    let mut total = 0usize;

    for l in times..(lines - times) {
        for c in times..(cols - times) {
            total += new_matrix[l][c];
        }
    }
    total
}

pub fn _p1(s: &'static str) -> usize {
    let times = 2usize;

    let total = enhance_and_compute_pixel_count(s, times);

    total
}

pub fn p1() -> usize {
    _p1(include_str!("j20.txt"))
}

pub fn _p2(s: &'static str) -> usize {
    let times = 50usize;

    let total = enhance_and_compute_pixel_count(s, times);

    total
}

pub fn p2() -> usize {
    _p2(include_str!("j20.txt"))
}

#[cfg(test)]
mod j20_tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(35, _p1(include_str!("j20_test.txt")));
        assert_eq!(5619, _p1(include_str!("j20.txt")));
    }

    #[test]
    fn test_p2() {
        assert_eq!(3351, _p2(include_str!("j20_test.txt")));
        assert_eq!(20122, _p2(include_str!("j20.txt")));
    }
}