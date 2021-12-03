use std::str::FromStr;

use itertools::enumerate;

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
                '1' => {
                    zeros_counts[i] += 1;
                }
                '0' => {
                    ones_counts[i] += 1;
                }
                _ => {
                    dbg!(c);
                    panic!()
                }
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
    let mut gamma = 0;
    let mut power = 0;

    let mut values = Vec::new();

    for (i, line) in s.lines().enumerate() {
        values.push(Vec::new());
        for c in line.chars() {
            values[i].push(c)
        }
    }

    let char_count = values[0].len();


    let mut zeros_counts = Vec::new();
    let mut ones_counts = Vec::new();

    let mut gamma_idx = None;

    'outer: for i in 0..char_count {
        if zeros_counts.len() == i {
            zeros_counts.push(0);
        }
        if ones_counts.len() == i {
            ones_counts.push(0);
        }
        for (line_idx, line) in values.iter().enumerate() {
            let c = line[i];

            let mut ignore = false;

            for j in 0..i {
                match zeros_counts[j] - ones_counts[j] {
                    _ if zeros_counts[j] + ones_counts[j] <= 1 => {
                        break 'outer;
                    }
                    0 if line[j] == '1' => {
                        continue;
                    }
                    k if k > 0 && line[j] == '0' => {
                        continue;
                    }
                    k if k < 0 && line[j] == '1' => {
                        continue;
                    }
                    _ => {
                        ignore = true;
                        break;
                    }
                }
            }

            if ignore {
                continue;
            }

            gamma_idx = Some(line_idx);

            match c {
                '1' => {
                    ones_counts[i] += 1;
                }
                '0' => {
                    zeros_counts[i] += 1;
                }
                _ => {}
            }
        }
    }
    // dbg!(&zeros_counts);
    // dbg!(&ones_counts);

    if let Some(i) = gamma_idx {
        // dbg!(&values[i]);
        for (i, c) in values[i].iter().rev().enumerate() {
            match c {
                '0' => {},
                '1' => gamma += 2usize.pow(i as u32),
                _ => panic!()
            }
        }
    }
    else {
        for (i, (zeros, ones)) in zeros_counts.iter().zip(ones_counts)
            .rev().enumerate() {
            match *zeros > ones {
                true => {
                    gamma += 2usize.pow(i as u32) * 0;
                }
                false => {
                    gamma += 2usize.pow(i as u32) * 1;
                }
            }
        }
    }

    let mut zeros_counts = Vec::new();
    let mut ones_counts = Vec::new();

    let mut power_idx = None;

    'outer: for i in 0..char_count {
        if zeros_counts.len() == i {
            zeros_counts.push(0);
        }
        if ones_counts.len() == i {
            ones_counts.push(0);
        }
        for (line_idx, line) in values.iter().enumerate() {
            let c = line[i];

            let mut ignore = false;

            for j in 0..i {
                match zeros_counts[j] - ones_counts[j] {
                    _ if zeros_counts[j] + ones_counts[j] <= 1 => {
                        break 'outer;
                    }
                    0 if line[j] == '0' => {
                        continue;
                    }
                    k if k < 0 && line[j] == '0' => {
                        continue;
                    }
                    k if k > 0 && line[j] == '1' => {
                        continue;
                    }
                    _ => {
                        ignore = true;
                        break;
                    }
                }
            }

            if ignore {
                continue;
            }

            power_idx = Some(line_idx);

            match c {
                '1' => {
                    ones_counts[i] += 1;
                }
                '0' => {
                    zeros_counts[i] += 1;
                }
                _ => {}
            }
        }
    }
    // dbg!(&zeros_counts);
    // dbg!(&ones_counts);

    if let Some(i) = power_idx {
        // dbg!(&values[i]);
        for (i, c) in values[i].iter().rev().enumerate() {
            match c {
                '0' => {},
                '1' => power += 2usize.pow(i as u32),
                _ => panic!()
            }
        }
    }
    else {
        for (i, (zeros, ones)) in zeros_counts.iter().zip(&ones_counts)
            .rev().enumerate() {
            match *zeros >= *ones {
                true => {
                    power += 2usize.pow(i as u32) * 1;
                }
                false => {
                    power += 2usize.pow(i as u32) * 0;
                }
            }
        }
    }

    // let mut power = 0;
    // let mut zeros_counts = Vec::new();
    // let mut ones_counts = Vec::new();
    // //
    // // let mut compatible_lines_idx = Vec::new();
    // //
    // let mut values = Vec::new();
    //
    // for (i, line) in s.lines().enumerate() {
    //     values.push(Vec::new());
    //     for c in line.chars() {
    //         values[i].push(c)
    //     }
    // }
    //
    // let char_count = values[0].len();
    //
    // let mut power_idx = 0;
    //
    // for i in 0..char_count {
    //     if zeros_counts.len() == i {
    //         zeros_counts.push(0);
    //     }
    //     if ones_counts.len() == i {
    //         ones_counts.push(0);
    //     }
    //     for line in values.iter() {
    //         let c = line[i];
    //
    //         let mut ignore = false;
    //
    //         for j in 0..i {
    //             match zeros_counts[j] - ones_counts[j] {
    //                 _ if zeros_counts[j] + ones_counts[j] <= 1 => {
    //                     power_idx = i;
    //                     continue
    //                 }
    //                 0 if line[j] == '0' => {
    //                     continue;
    //                 }
    //                 k if k < 0 && line[j] == '0' => {
    //                     continue;
    //                 }
    //                 k if k > 0 && line[j] == '1' => {
    //                     continue;
    //                 }
    //                 _ => {
    //                     ignore = true;
    //                     break;
    //                 }
    //             }
    //         }
    //
    //         if ignore {
    //             continue;
    //         }
    //
    //         match c {
    //             '1' => {
    //                 ones_counts[i] += 1;
    //             }
    //             '0' => {
    //                 zeros_counts[i] += 1;
    //             }
    //             _ => {}
    //         }
    //     }
    // }

    // dbg!(&zeros_counts);
    // dbg!(&ones_counts);
    //
    // dbg!(&values[power_idx.unwrap()]);

    // for (i, (zeros, ones)) in zeros_counts.iter().zip(ones_counts)
    //     .rev().enumerate() {
    //     match *zeros > ones {
    //         true => {
    //             power += 2usize.pow(i as u32) * 1;
    //         }
    //         false => {
    //             power += 2usize.pow(i as u32) * 0;
    //         }
    //     }
    // }
    dbg!(gamma);
    dbg!(power);
    gamma * power
}

pub fn p2() -> usize {
    _p2(include_str!("j3.txt"))
}

#[cfg(test)]
mod j1_tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(198, _p1(include_str!("j3_test.txt")))
    }

    #[test]
    fn test_p2() {
        assert_eq!(230, _p2(include_str!("j3_test.txt")))
    }
}