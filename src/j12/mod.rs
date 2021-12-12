use std::collections::{HashMap, HashSet};
use smallvec::SmallVec;

fn compute_paths(s: &str, allow_one_small_cave_to_be_explored_twice: bool) -> usize {
    #[derive(PartialEq)]
    enum Size {
        Small,
        Large,
    }

    let mut nodes = HashSet::new();
    let mut neighbours = HashMap::new();
    let mut size = HashMap::new();

    for line in s.lines() {
        let mut line_it = line.split('-');
        let node_1 = line_it.next().unwrap();
        let node_2 = line_it.next().unwrap();

        nodes.insert(node_1);
        nodes.insert(node_2);
        neighbours.entry(node_1).or_insert(HashSet::new()).insert(node_2);
        neighbours.entry(node_2).or_insert(HashSet::new()).insert(node_1);
        size.insert(node_1, if node_1.to_lowercase() == node_1 { Size::Small } else { Size::Large });
        size.insert(node_2, if node_2.to_lowercase() == node_2 { Size::Small } else { Size::Large });
    }

    let mut all_to_visit = SmallVec::<[(SmallVec<[&&str;24]>, bool);2048]>::new();
    all_to_visit.push((SmallVec::<[&&str;24]>::new(), false));
    all_to_visit[0].0.push(nodes.iter().filter(|&&s|s == "start").next().unwrap());

    let mut all_paths = Vec::new();
    while let Some((mut to_visit, mut b)) = all_to_visit.pop() {
        // dbg!("Next !");
        while let Some(&n) = to_visit.last() {
            let mut path_invalidated = false;
            let mut local_b = b;
            // dbg!(&to_visit);
            match *n {
                "end" => {
                    all_paths.push(to_visit);
                    break;
                }
                _ => {
                    let mut consumed = false;
                    for n in neighbours.get(n).unwrap().iter() {
                        if *n == "start" {
                            continue;
                        }
                        match (to_visit.iter().filter(|&&c|c == n).count(),
                               size.get(n).unwrap(),
                               b) {
                            (_, Size::Large, _) | (0, _, _) => {
                                if consumed {
                                    let mut clone = to_visit.clone();
                                    clone.pop();
                                    clone.push(n);
                                    all_to_visit.push((clone, b));
                                } else {
                                    to_visit.push(n);
                                    consumed = true;
                                }
                            },
                            (1, Size::Small, false) if allow_one_small_cave_to_be_explored_twice => {
                                if consumed {
                                    let mut clone = to_visit.clone();
                                    clone.pop();
                                    clone.push(n);
                                    all_to_visit.push((clone, true));
                                } else {
                                    to_visit.push(n);
                                    consumed = true;
                                    local_b = true;
                                }
                            },
                            (_, _, _) => {
                                if consumed {
                                    continue
                                }
                                else {
                                    consumed = true;
                                    to_visit.push(n);
                                    path_invalidated = true;
                                }
                            }
                        }
                    }
                }
            }
            b = local_b;
            if path_invalidated {
                break;
            }
        }
    }

    all_paths.len()
}

pub fn _p1(s: &str) -> usize {
    compute_paths(s, false)
}

pub fn p1() -> usize {
    _p1(include_str!("j12.txt"))
}

pub fn _p2(s: &str) -> usize {
    compute_paths(s, true)
}

pub fn p2() -> usize {
    _p2(include_str!("j12.txt"))
}

#[cfg(test)]
mod j12_tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(10, _p1(include_str!("j12_small_test.txt")));
        assert_eq!(19, _p1(include_str!("j12_test.txt")));
        assert_eq!(226, _p1(include_str!("j12_larger_test.txt")));
        assert_eq!(3761, _p1(include_str!("j12.txt")));
    }

    #[test]
    fn test_p2() {
        assert_eq!(36, _p2(include_str!("j12_small_test.txt")));
        assert_eq!(103, _p2(include_str!("j12_test.txt")));
        assert_eq!(3509, _p2(include_str!("j12_larger_test.txt")));
        assert_eq!(99138, _p2(include_str!("j12.txt")));
    }
}