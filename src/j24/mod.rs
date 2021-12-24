use std::str::FromStr;
use itertools::Itertools;
use rand::{Rng, thread_rng};
use smallvec::SmallVec;
use crate::j24::SecondArgument::{Mem, Val};
use crate::j24::Instruction::*;

fn letter_to_storage(c: char) -> usize {
    match c {
        'x' => 0,
        'y' => 1,
        'z' => 2,
        'w' => 3,
        _ => panic!("{}", c)
    }
}

#[derive(Debug)]
enum SecondArgument {
    Val(i64),
    Mem(usize),
}

#[derive(Debug)]
enum Instruction {
    Inp(usize),
    Add(usize, SecondArgument),
    Mul(usize, SecondArgument),
    Div(usize, SecondArgument),
    Mod(usize, SecondArgument),
    Eql(usize, SecondArgument),
}

impl Instruction {
    pub fn exec(&self, input: [i64; 14], idx: &mut usize, mem: &mut [i64; 4]) {
        match *self {
            Instruction::Inp(a) => {
                mem[a] = input[*idx];
                *idx += 1;
            }
            Instruction::Add(a, Mem(b)) => { mem[a] += mem[b]; }
            Instruction::Mul(a, Mem(b)) => { mem[a] *= mem[b]; }
            Instruction::Div(a, Mem(b)) => { mem[a] /= mem[b]; }
            Instruction::Mod(a, Mem(b)) => { mem[a] = mem[a] % mem[b]; }
            Instruction::Eql(a, Mem(b)) => { mem[a] = if mem[a] == mem[b] { 1 } else { 0 } }
            Instruction::Add(a, Val(b)) => { mem[a] += b; }
            Instruction::Mul(a, Val(b)) => { mem[a] *= b; }
            Instruction::Div(a, Val(b)) => { mem[a] /= b; }
            Instruction::Mod(a, Val(b)) => { mem[a] = mem[a] % b; }
            Instruction::Eql(a, Val(b)) => { mem[a] = if mem[a] == b { 1 } else { 0 } }
        }
    }
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let a = letter_to_storage(s[4..5].chars().next().unwrap());
        Ok(match &s[0..3] {
            "inp" => Inp(a),
            _ => {
                let b = match &s[6..] {
                    l if l == "x" || l == "y" || l == "z" || l == "w" => Mem(letter_to_storage(l.chars().next().unwrap())),
                    l => Val(i64::from_str(l).unwrap())
                };
                match &s[0..3] {
                    "add" => Add(a, b),
                    "mul" => Mul(a, b),
                    "div" => Div(a, b),
                    "mod" => Mod(a, b),
                    "eql" => Eql(a, b),
                    _ => panic!()
                }
            }
        })
    }
}

#[test]
fn test_program() {
    let s = include_str!("j24.txt");

    let mut program = SmallVec::<[Instruction; 512]>::new();

    for line in s.lines() {
        program.push(Instruction::from_str(line).unwrap())
    }

    for l in 0..=9 {
        for k in 0..=9 {
            for j in 0..=9 {
                for i in 0..=9 {
                    let serial = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, l, k, j, i];

                    let mut idx = 0usize;

                    let mut mem = [0; 4];
                    for ins in program.iter() {
                        ins.exec(serial.clone(), &mut idx, &mut mem);
                    }

                    assert_eq!(program_rust(&serial.clone(), 14), mem[2]);
                }
            }
        }
    }
}

fn serial_to_number(serial: &[i64; 14]) -> i64 {
    serial.iter().rev().enumerate().map(|(i,e)|*e*10i64.pow(i as u32)).sum()
}

#[test]
fn genetic_search() {

    // dbg!(program_rust(&[4,1,2,9,9,9,9,4,8,4,6,9,5,9],14));
    // return;
    let mut population = [[1; 14]; 1024];
    let mut new_population = [[1i64; 14]; 1024];
    let first_digit = 1i64..=1i64;
    let mut stag = 0usize;

    for i in 0..1024 {
        for j in 0..14 {
            population[i][j] = thread_rng().gen_range(1..=9);
        }
        population[i][0] = thread_rng().gen_range(first_digit.clone());;
    }

    let mut best = [164; 14];
    let mut best_score = i64::MAX;

    loop {
        if stag > 1_000_0 {
            dbg!("reset");
            stag = 0;


            for i in 0..1024 {
                for j in 0..14 {
                    population[i][j] = thread_rng().gen_range(1..=9);
                }
                population[i][0] = thread_rng().gen_range(first_digit.clone());;
            }
        }

        let breeders_scored: SmallVec<[(usize, i64); 512]> = population.iter()
            .map(|p| program_rust(p, 14)).enumerate().sorted_by_key(|(i,p)|*p).take(512)
            .collect();


        if breeders_scored[0].1 == best_score && serial_to_number(&population[breeders_scored[0].0]) <
            serial_to_number(&best){
            best_score = breeders_scored[0].1;
            best = population[breeders_scored[0].0].clone();
            println!("{}", best.iter().join(""));
            dbg!(best_score);
        }

        if breeders_scored[0].1 < best_score {
            best_score = breeders_scored[0].1;
            best = population[breeders_scored[0].0].clone();
            println!("{}", best.iter().join(""));
            dbg!(best_score);
        }
        else {
            stag += 1;
        }

        // if breeders_scored[0].1 == 0 {
        //     break;
        // }

        for i in 0..1024 {
            if i < 32 {
                new_population[i] = population[breeders_scored[i].0].clone();
                new_population[i][0] = thread_rng().gen_range(first_digit.clone());
            }
            if i > 768 {
                for j in 0..14 {
                    new_population[i][j] = thread_rng().gen_range(1i64..9i64);
                }
                new_population[i][0] = thread_rng().gen_range(first_digit.clone());;
                continue
            }
            let b1 = &population[breeders_scored[thread_rng().gen_range(0..512)].0];
            let b2 = &population[breeders_scored[thread_rng().gen_range(0..512)].0];

            for j in 0..14 {
                new_population[i][j] = if j < 7 {b1[j]} else {b2[j]};
            }
            new_population[i][0] = thread_rng().gen_range(first_digit.clone());;

            if thread_rng().gen_range(0.0..1.0) < 0.5 {
                let idx = thread_rng().gen_range(0..14);
                new_population[i][idx] = match new_population[i][idx] {
                    9 => 8,
                    1 => 2,
                    n => thread_rng().gen_range(0..2) * 2 - 1 + n
                };
            }
            new_population[i][0] = thread_rng().gen_range(first_digit.clone());;

            if thread_rng().gen_range(0.0..1.0) < 0.5 {
                let idx1 = thread_rng().gen_range(0..14);
                let idx2 = thread_rng().gen_range(0..14);
                let tmp = new_population[i][idx1];
                new_population[i][idx1] = new_population[i][idx2];
                new_population[i][idx2] = tmp;
            }
            new_population[i][0] = thread_rng().gen_range(first_digit.clone());;
        }

        std::mem::swap(&mut population, &mut new_population);
    }
}

#[test]
fn final_test() {
    for v1 in (1..=9).rev() {
        for v2 in (1..=9).rev() {
            for v3 in (1..=9).rev() {
                for v4 in (1..=9).rev() {
                    for v5 in (1..=9).rev() {
                        for v6 in (1..=9).rev() {
                            for v7 in (1..=9).rev() {
                                let seal = [4, 1, 2, 9, 9, 9, 9, v1, v2, v3, v4, v5, v6, v7];

                                if program_rust(&seal, 14) == 0 {
                                    println!("{}",seal.iter().join(""));
                                    return;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// Best min : 11189561113216
// Best : 41299994879959
// 41299994879959
// 41299994846959
// 41299994824959
// 41299994813959
// 41299694324429
// 41299694224329
// 41189783146239
// 41189583224319
//best = 31299572113218
// 21189661124227
// 21189783335437
// 21189661113227

#[test]
fn program_rust_test() {
    let mut serial = [9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9];
    let mut rslt = i64::MAX;

    let mut best_serial = serial;
    let mut best_rslt = i64::MAX;
    loop {

        let idx = thread_rng().gen_range(0..14);
        let old_idx = serial[idx];
        serial[idx] = match serial[idx] {
            9 => 8,
            1 => 2,
            n => thread_rng().gen_range(0..2) * 2 - 1 + n
        };

        let tmp = program_rust(&serial.clone(), 14);
        if tmp == 0 {
            dbg!(serial);
            break;
        }

        if tmp > rslt && thread_rng().gen_range(0.0..1.0) < 0.90 {
            serial[idx] = old_idx;
        } else {
            if tmp < best_rslt {
                best_serial = serial.clone();
                best_rslt = tmp;
                println!("{}", best_serial.iter().join(""));
                dbg!(best_rslt);
            }
            rslt = tmp;
        }
    }
}


fn program_rust(variables: &[i64; 14], max: usize) -> i64 {
    let vals = [11, 12, 13, -5, -3, 14, 15, -16, 14, 15, -7, -11, -6, -11];
    let divs = [1, 1, 1, 26, 26, 1, 1, 26, 1, 1, 26, 26, 26, 26];
    let vals2 = [16, 11, 12, 12, 12, 2, 11, 4, 12, 9, 10, 11, 6, 15];

    let mut x;
    let mut z = 0;
    let mut w;

    for i in 0..max {
        w = variables[i];
        x = z % 26;
        z = z / divs[i];
        x = x + vals[i];
        x = if x != w { 1 } else { 0 };
        z = (25 * x + 1) * z + (w + vals2[i]) * x;

        // println!("{}, {}, {}", x, z, w);
    }

    z
}

#[test]
fn simple_test() {
    let s = include_str!("j24.txt");

    let mut program = SmallVec::<[Instruction; 512]>::new();

    for line in s.lines() {
        program.push(Instruction::from_str(line).unwrap())
    }

    for l in 0..=9 {
        for k in 0..=9 {
            for j in 0..=9 {
                for i in 0..=9 {
                    let serial = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, l, k, j, i];

                    let mut idx = 0usize;

                    let mut mem = [0; 4];
                    for ins in program.iter() {
                        ins.exec(serial, &mut idx, &mut mem);
                    }
                    if mem[0] == 0 {
                        println!("{}", serial.iter().join(""));
                    }
                    if mem[2] == 0 {
                        println!("{}", mem.iter().join(","))
                        // println!("{}", mem.iter().join(","));
                    }
                }
                // println!()
            }
        }
    }
}

pub fn _p1(s: &'static str) -> usize {
    let mut program = SmallVec::<[Instruction; 512]>::new();

    for line in s.lines() {
        program.push(Instruction::from_str(line).unwrap())
    }

    for v1 in (1i64..=9).rev() {
        for v2 in (1i64..=9).rev() {
            for v3 in (1i64..=9).rev() {
                for v4 in (1i64..=9).rev() {
                    for v5 in (1i64..=9).rev() {
                        for v6 in (1i64..=9).rev() {
                            for v7 in (1i64..=9).rev() {
                                for v8 in (1i64..=9).rev() {
                                    for v9 in (1i64..=9).rev() {
                                        for v10 in (1i64..=9).rev() {
                                            for v11 in (1i64..=9).rev() {
                                                for v12 in (1i64..=9).rev() {
                                                    for v13 in (1i64..=9).rev() {
                                                        for v14 in (1i64..=9).rev() {
                                                            let serial =
                                                                [v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14];

                                                            if program_rust(&serial, 14) == 0
                                                            {
                                                                dbg!("{SERIAL FOUND : }", serial);
                                                                return 42;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    dbg!(program);

    42
}

pub fn p1() -> usize {
    _p1(include_str!("j24.txt"))
}

pub fn _p2(s: &'static str) -> usize {
    42
}

pub fn p2() -> usize {
    _p2(include_str!("j24.txt"))
}

#[cfg(test)]
mod j24_tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(12521, _p1(include_str!("j24_test.txt")));
        assert_eq!(11320, _p1(include_str!("j24.txt")));
    }

    #[test]
    fn test_p2() {
        assert_eq!(44169, _p2(include_str!("j24_test.txt")));
        assert_eq!(49532, _p2(include_str!("j24.txt")));
    }
}