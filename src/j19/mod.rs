use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use regex::Regex;

fn roll(xyz: (i64, i64, i64)) -> (i64, i64, i64) {
    (xyz.0, xyz.2, -xyz.1)
}

fn turn(xyz: (i64, i64, i64)) -> (i64, i64, i64) {
    (-xyz.1, xyz.0, xyz.2)
}

struct RotationIterator {
    cycle: usize,
    step: usize,
    i: usize,
    prev_i: bool,
    v: (i64, i64, i64),
}

impl RotationIterator {
    fn new(v: (i64, i64, i64)) -> Self {
        RotationIterator {
            cycle: 0,
            step: 0,
            i: 0,
            prev_i: true,
            v,
        }
    }
}

impl Iterator for RotationIterator {
    type Item = (i64, i64, i64);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match (self.cycle, self.step, self.i, self.prev_i) {
                (2, _, _, _) => break None,
                (_, 3, _, _) => {
                    self.v = roll(turn(roll(self.v)));
                    self.step = 0;
                    self.cycle += 1;
                    self.i = 0;
                    self.prev_i = true;
                }
                (_, _, 3, _) => {
                    self.step += 1;
                    self.i = 0;
                    self.prev_i = true;
                }
                (_, _, _, true) => {
                    self.v = roll(self.v);
                    self.prev_i = false;
                    break Some(self.v);
                }
                (_, _, _, false) => {
                    self.v = turn(self.v);
                    self.i += 1;
                    break Some(self.v);
                }
            }
        }
    }
}

fn parse_position(line: &str, reg: &Regex) -> Option<(i64, i64, i64)> {
    reg.captures(line)
        .map(|cap| (i64::from_str(&cap[1]).unwrap(),
                    i64::from_str(&cap[2]).unwrap(),
                    i64::from_str(&cap[3]).unwrap()))
}

fn parse_scanner(line: &str, reg: &Regex) -> Option<usize> {
    reg.captures(line)
        .map(|cap| usize::from_str(&cap[1]).unwrap())
}

fn get_probes_with_at_least_12_common_beacons(beacons_relative_positions: &Vec<HashSet<(i64, i64, i64)>>) -> HashMap<(usize, usize), ((i64, i64, i64), (i64, i64, i64), usize, HashSet<(i64, i64, i64)>, (i64, i64, i64))> {
    let mut transformed_probes = HashSet::new();
    let mut transformations = HashMap::new();
    for i in 0..beacons_relative_positions.len() {
        'j: for j in 0..beacons_relative_positions.len() {
            if i == j {
                continue;
            }
            for (_b1_idx, &b1) in beacons_relative_positions[i].iter().enumerate() {
                for (_b2_idx, &b2) in beacons_relative_positions[j].iter().enumerate() {
                    for r in 0..24 {
                        transformed_probes.clear();

                        for &v in beacons_relative_positions[j].iter() {
                            let rotated = RotationIterator::new(
                                (v.0 - b2.0, v.1 - b2.1, v.2 - b2.2)
                            ).skip(r).next().unwrap();
                            transformed_probes.insert((b1.0 + rotated.0, b1.1 + rotated.1, b1.2 + rotated.2));
                        }
                        if transformed_probes.intersection(&beacons_relative_positions[i]).count() >= 12 {
                            let center_rotated = RotationIterator::new(
                                (-b2.0, -b2.1, -b2.2)
                            ).skip(r).next().unwrap();
                            transformations.entry((i, j)).or_insert((b1, b2, r, transformed_probes.clone(),
                                                                     (b1.0 + center_rotated.0, b1.1 + center_rotated.1, b1.2 + center_rotated.2)));
                            continue 'j;
                        }
                    }
                }
            }
        }
    }
    transformations
}

fn parse_beacons_relative_positions(s: &str) -> Vec<HashSet<(i64, i64, i64)>> {
    let pos_reg: Regex = Regex::new("(-?[0-9]+),(-?[0-9]+),(-?[0-9]+)").unwrap();
    let scan_reg: Regex = Regex::new("--- scanner ([0-9]+) ---").unwrap();
    let mut scanners_with_probes = Vec::new();

    let mut current_scanner = 0usize;
    for line in s.lines() {
        if let Some(scanner_id) = parse_scanner(line, &scan_reg) {
            current_scanner = scanner_id;
            scanners_with_probes.push(HashSet::with_capacity(30));
            continue;
        }
        if line.is_empty() {
            continue;
        }
        if let Some((x, y, z)) = parse_position(line, &pos_reg) {
            scanners_with_probes[current_scanner]
                .insert((x, y, z));
        }
    }
    scanners_with_probes
}

pub fn reduce_transformations(
    num_scanners: usize,
    transformations: &mut HashMap<(usize, usize), ((i64, i64, i64), (i64, i64, i64), usize, HashSet<(i64, i64, i64)>, (i64, i64, i64))>) -> HashMap<(usize, usize), ((i64, i64, i64), (i64, i64, i64), usize, HashSet<(i64, i64, i64)>, (i64, i64, i64))> {
    let mut reduced_transformations = transformations.clone();
    for k in 1..num_scanners {
        while !reduced_transformations.contains_key(&(0usize, k)) {
            'c: for (&(k1, k2), (b1, b2, r21, _, _)) in transformations.iter() {
                for (&(k3, k4), (_, _, _, hs43, center)) in reduced_transformations.iter() {
                    if k2 != k3 || reduced_transformations.contains_key(&(k1, k4)) {
                        continue;
                    }
                    let mut transformed_probes = HashSet::with_capacity(hs43.len());

                    for &v in hs43.iter() {
                        let rotated = RotationIterator::new((
                            v.0 - b2.0,
                            v.1 - b2.1,
                            v.2 - b2.2,
                        )).skip(*r21).next().unwrap();
                        transformed_probes.insert((
                            b1.0 + rotated.0,
                            b1.1 + rotated.1,
                            b1.2 + rotated.2,
                        ));
                    }
                    let center_rotated = RotationIterator::new((
                        center.0 - b2.0,
                        center.1 - b2.1,
                        center.2 - b2.2,
                    )).skip(*r21).next().unwrap();
                    reduced_transformations.entry((k1, k4)).or_insert((*b1, *b2, *r21, transformed_probes,(
                        b1.0 + center_rotated.0,
                        b1.1 + center_rotated.1,
                        b1.2 + center_rotated.2,
                    )));
                    continue 'c;
                }
            }
        }
    }
    reduced_transformations
}

pub fn _p1(s: &'static str) -> usize {
    let beacons_relative_positions = parse_beacons_relative_positions(s);

    let mut transformations = get_probes_with_at_least_12_common_beacons(&beacons_relative_positions);

    let reduced_transformations = reduce_transformations(beacons_relative_positions.len(),
                                                         &mut transformations);

    let mut final_positions = beacons_relative_positions[0].clone();

    for i in 1..beacons_relative_positions.len() {
        for elt in reduced_transformations[&(0, i)].3.iter() {
            final_positions.insert(*elt);
        }
    }

    final_positions.len()
}


pub fn p1() -> usize {
    _p1(include_str!("j19.txt"))
}


pub fn _p2(s: &'static str) -> usize {
    let beacons_relative_positions = parse_beacons_relative_positions(s);

    let mut transformations = get_probes_with_at_least_12_common_beacons(&beacons_relative_positions);

    let reduced_transformations = reduce_transformations(beacons_relative_positions.len(),
                                                         &mut transformations);

    let mut centers = Vec::new();

    centers.push((0, 0, 0));

    for i in 1..beacons_relative_positions.len() {
        centers.push(reduced_transformations[&(0, i)].4);
    }

    let mut max_distance = usize::MIN;
    for (i, &c1) in centers.iter().enumerate() {
        for &c2 in centers.iter().skip(i) {
            max_distance = max_distance.max(
                (c1.0 - c2.0).abs() as usize +
                    (c1.1 - c2.1).abs() as usize +
                    (c1.2 - c2.2).abs() as usize
            );
        }
    }

    max_distance
}

pub fn p2() -> usize {
    _p2(include_str!("j19.txt"))
}

#[cfg(test)]
mod j19_tests {
    use super::*;

    #[test]
    fn test_vec_orientations() {
        let mut hash = HashSet::new();
        for v in RotationIterator::new((1, 2, 3)) {
            hash.insert(v);
        }
        assert_eq!(24, hash.len());
    }

    #[test]
    fn test_p1() {
        assert_eq!(79, _p1(include_str!("j19_test.txt")));
        assert_eq!(385, _p1(include_str!("j19.txt")));
    }

    #[test]
    fn test_p2() {
        assert_eq!(3621, _p2(include_str!("j19_test.txt")));
        assert_eq!(10707, _p2(include_str!("j19.txt")));
    }
}