use smallvec::SmallVec;

fn hex_to_bits(s: &'static str) -> impl Iterator<Item=u8> {
    s.chars().flat_map(|c| match c {
        '0' => [0, 0, 0, 0],
        '1' => [0, 0, 0, 1],
        '2' => [0, 0, 1, 0],
        '3' => [0, 0, 1, 1],
        '4' => [0, 1, 0, 0],
        '5' => [0, 1, 0, 1],
        '6' => [0, 1, 1, 0],
        '7' => [0, 1, 1, 1],
        '8' => [1, 0, 0, 0],
        '9' => [1, 0, 0, 1],
        'A' => [1, 0, 1, 0],
        'B' => [1, 0, 1, 1],
        'C' => [1, 1, 0, 0],
        'D' => [1, 1, 0, 1],
        'E' => [1, 1, 1, 0],
        'F' => [1, 1, 1, 1],
        _ => panic!()
    })
}

fn read_n_bits_and_compute_sum(bits: &mut impl Iterator<Item=u8>, count: u64) -> i64 {
    let mut sum = 0;
    for i in (0..count).rev() {
        let b = bits.next().unwrap() as i64;
        sum += 2i64.pow(i as u32) * b;
    }
    sum
}

fn parse_packet(bits: &mut impl Iterator<Item=u8>) -> (usize, usize, i64) {
    let mut bit_read = 0usize;
    let mut total_versions = 0usize;
    let version = read_n_bits_and_compute_sum(bits, 3);
    bit_read += 3;
    total_versions += version as usize;
    let t = read_n_bits_and_compute_sum(bits, 3);
    bit_read += 3;

    match t {
        // Literal
        4 => {
            let mut num_bits = SmallVec::<[u8; 4]>::new();
            loop {
                let b = bits.next().unwrap();
                bit_read += 1;
                for _ in 0..4 {
                    num_bits.push(bits.next().unwrap());
                }
                bit_read += 4;
                match b {
                    0 => { break; }
                    1 => {}
                    _ => panic!()
                }
            }
            let num_bits_len = num_bits.len();
            let num = read_n_bits_and_compute_sum(&mut num_bits.into_iter(), num_bits_len as u64);
            return (total_versions, bit_read, num);
        }
        // Operator
        _ => {
            let length_type = bits.next().unwrap();
            bit_read += 1;
            let mut result = None;
            match length_type {
                0 => {
                    let total_length_in_bits = read_n_bits_and_compute_sum(bits, 15);
                    bit_read += 15;
                    let mut packet_bits = 0usize;
                    loop {
                        let (tv, br, num) = parse_packet(bits);

                        result = update_result_based_on_operator_type(t, result, num);

                        total_versions += tv;
                        packet_bits += br;
                        if packet_bits == total_length_in_bits as usize {
                            break;
                        }
                        if packet_bits > total_length_in_bits as usize {
                            panic!();
                        }
                    }
                    bit_read += packet_bits;
                }
                1 => {
                    let number_of_sub_packets = read_n_bits_and_compute_sum(bits, 11);
                    bit_read += 11;

                    for _ in 0..number_of_sub_packets {
                        let (tv, br, num) = parse_packet(bits);

                        result = update_result_based_on_operator_type(t, result, num);

                        total_versions += tv;
                        bit_read += br;
                    }
                }
                _ => panic!(),
            }
            (total_versions, bit_read, result.unwrap())
        }
    }
}

fn update_result_based_on_operator_type(t: i64, result: Option<i64>, num: i64) -> Option<i64> {
    Some(match t {
        0 => result.unwrap_or(0) + num,
        1 => result.unwrap_or(1) * num,
        2 => result.unwrap_or(i64::MAX).min(num),
        3 => result.unwrap_or(i64::MIN).max(num),
        5 => if let Some(prev) = result {
            if prev > num { 1 } else { 0 }
        } else {
            num
        }
        6 => if let Some(prev) = result {
            if prev < num { 1 } else { 0 }
        } else {
            num
        }
        7 => if let Some(prev) = result {
            if prev == num { 1 } else { 0 }
        } else {
            num
        }
        _ => panic!()
    })
}

fn parse_str(s: &'static str) -> (usize, usize, i64) {
    let mut bits = hex_to_bits(s).into_iter();

    parse_packet(&mut bits)
}


pub fn _p1(s: &'static str) -> usize {
    parse_str(s).0
}

pub fn p1() -> usize {
    _p1(include_str!("j16.txt"))
}

pub fn _p2(s: &'static str) -> i64 {
    parse_str(s).2
}

pub fn p2() -> i64 {
    _p2(include_str!("j16.txt"))
}

#[cfg(test)]
mod j16_tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(16, _p1(include_str!("j16_p1_test_0.txt")));
        assert_eq!(12, _p1(include_str!("j16_p1_test_1.txt")));
        assert_eq!(23, _p1(include_str!("j16_p1_test_2.txt")));
        assert_eq!(31, _p1(include_str!("j16_p1_test_3.txt")));
        assert_eq!(906, _p1(include_str!("j16.txt")));
    }

    #[test]
    fn test_p2() {
        assert_eq!(3, _p2(include_str!("j16_p2_test_0.txt")));
        assert_eq!(54, _p2(include_str!("j16_p2_test_1.txt")));
        assert_eq!(7, _p2(include_str!("j16_p2_test_2.txt")));
        assert_eq!(9, _p2(include_str!("j16_p2_test_3.txt")));
        assert_eq!(1, _p2(include_str!("j16_p2_test_4.txt")));
        assert_eq!(0, _p2(include_str!("j16_p2_test_5.txt")));
        assert_eq!(0, _p2(include_str!("j16_p2_test_6.txt")));
        assert_eq!(1, _p2(include_str!("j16_p2_test_7.txt")));
        assert_eq!(819324480368, _p2(include_str!("j16.txt")));
    }
}