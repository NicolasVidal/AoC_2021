use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use regex::Regex;

fn get_nth_rotation(v: (i32, i32, i32), i: usize) -> (i32, i32, i32) {
    let (x, y, z) = v;
    match i {
        0 => (x, z, -y),
        1 => (-z, x, -y),
        2 => (-x, -z, -y),
        3 => (z, -x, -y),
        4 => (z, -y, x),
        5 => (y, z, x),
        6 => (-z, y, x),
        7 => (-y, -z, x),
        8 => (-y, x, z),
        9 => (-x, -y, z),
        10 => (y, -x, z),
        11 => (x, y, z),
        12 => (-z, -x, y),
        13 => (x, -z, y),
        14 => (z, x, y),
        15 => (-x, z, y),
        16 => (-x, y, -z),
        17 => (-y, -x, -z),
        18 => (x, -y, -z),
        19 => (y, x, -z),
        20 => (y, -z, -x),
        21 => (z, y, -x),
        22 => (-y, z, -x),
        23 => (-z, -y, -x),
        _ => panic!()
    }
}

fn parse_position(line: &str, reg: &Regex) -> Option<(i32, i32, i32)> {
    reg.captures(line)
        .map(|cap| (i32::from_str(&cap[1]).unwrap(),
                    i32::from_str(&cap[2]).unwrap(),
                    i32::from_str(&cap[3]).unwrap()))
}

fn parse_scanner(line: &str, reg: &Regex) -> Option<usize> {
    reg.captures(line)
        .map(|cap| usize::from_str(&cap[1]).unwrap())
}

fn get_probes_with_at_least_12_common_beacons(beacons_relative_positions: &Vec<HashSet<(i32, i32, i32)>>) -> HashMap<(usize, usize), ((i32, i32, i32), (i32, i32, i32), usize, HashSet<(i32, i32, i32)>, (i32, i32, i32))> {
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
                            let rotated = get_nth_rotation(
                                (v.0 - b2.0, v.1 - b2.1, v.2 - b2.2),
                                r);
                            transformed_probes.insert((b1.0 + rotated.0, b1.1 + rotated.1, b1.2 + rotated.2));
                        }
                        if transformed_probes.intersection(&beacons_relative_positions[i]).count() >= 12 {
                            let center_rotated = get_nth_rotation(
                                (-b2.0, -b2.1, -b2.2),
                                r);
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

fn parse_beacons_relative_positions(s: &str) -> Vec<HashSet<(i32, i32, i32)>> {
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
    transformations: &mut HashMap<(usize, usize), ((i32, i32, i32), (i32, i32, i32), usize, HashSet<(i32, i32, i32)>, (i32, i32, i32))>) -> HashMap<(usize, usize), ((i32, i32, i32), (i32, i32, i32), usize, HashSet<(i32, i32, i32)>, (i32, i32, i32))> {
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
                        let rotated = get_nth_rotation((
                                                           v.0 - b2.0,
                                                           v.1 - b2.1,
                                                           v.2 - b2.2,
                                                       ), *r21);
                        transformed_probes.insert((
                            b1.0 + rotated.0,
                            b1.1 + rotated.1,
                            b1.2 + rotated.2,
                        ));
                    }
                    let center_rotated = get_nth_rotation((
                                                              center.0 - b2.0,
                                                              center.1 - b2.1,
                                                              center.2 - b2.2,
                                                          ), *r21);
                    reduced_transformations.entry((k1, k4)).or_insert((*b1, *b2, *r21, transformed_probes, (
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