use std::collections::{HashMap, HashSet};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use smallvec::SmallVec;

fn compute_paths(s: &str, allow_one_small_cave_to_be_explored_twice: bool) -> usize {
    #[derive(PartialEq, Eq, Hash, Clone)]
    enum Node {
        Start,
        End,
        Small(u64),
        Large(u64),
    }

    fn name_to_node(name: &str) -> Node {
        match name {
            "start" => Node::Start,
            "end" => Node::End,
            _ => {
                let mut hasher = DefaultHasher::new();
                name.hash(&mut hasher);
                let h = hasher.finish();
                if name.to_lowercase() == name {
                    Node::Small(h)
                } else {
                    Node::Large(h)
                }
            }
        }
    }

    let mut neighbours = HashMap::new();

    for line in s.lines() {
        let mut line_it = line.split('-');
        let node_1 = name_to_node(line_it.next().unwrap());
        let node_2 = name_to_node(line_it.next().unwrap());

        neighbours.entry(node_1.clone()).or_insert(HashSet::new()).insert(node_2.clone());
        neighbours.entry(node_2.clone()).or_insert(HashSet::new()).insert(node_1.clone());
    }

    let mut all_to_visit = SmallVec::<[(SmallVec<[Node; 24]>, bool); 1024]>::new();
    all_to_visit.push((SmallVec::<[Node; 24]>::new(), false));
    all_to_visit[0].0.push(Node::Start);

    let mut path_count = 0usize;
    while let Some((mut to_visit, mut b)) = all_to_visit.pop() {
        while let Some(n) = to_visit.last() {
            let mut path_invalidated = false;
            let mut local_b = b;
            match *n {
                Node::End => {
                    path_count += 1;
                    break;
                }
                _ => {
                    let mut consumed = false;
                    for n in neighbours.get(&n).unwrap().iter() {
                        if *n == Node::Start {
                            continue;
                        }
                        match (to_visit.iter().filter(|&c| *c == *n).count(),
                               n,
                               b) {
                            (_, Node::Large(_), _) | (0, _, _) => {
                                if consumed {
                                    let mut clone = to_visit.clone();
                                    clone.pop();
                                    clone.push(n.clone());
                                    all_to_visit.push((clone, b));
                                } else {
                                    to_visit.push(n.clone());
                                    consumed = true;
                                }
                            }
                            (1, Node::Small(_), false) if allow_one_small_cave_to_be_explored_twice => {
                                if consumed {
                                    let mut clone = to_visit.clone();
                                    clone.pop();
                                    clone.push(n.clone());
                                    all_to_visit.push((clone, true));
                                } else {
                                    to_visit.push(n.clone());
                                    consumed = true;
                                    local_b = true;
                                }
                            }
                            (_, _, _) => {
                                if consumed {
                                    continue;
                                } else {
                                    consumed = true;
                                    to_visit.push(n.clone());
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

    path_count
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