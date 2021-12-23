use std::str::FromStr;
use crate::j23::dijkstra::shortest_path;

mod dijkstra;
mod p1;
mod p2;

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Hash)]
enum AmphipodPosition {
    HallWay(usize),
    Room(usize, usize),
}

impl Default for AmphipodPosition {
    fn default() -> Self {
        AmphipodPosition::Room(0, 0)
    }
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Hash)]
enum AmphipodType {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl Default for AmphipodType {
    fn default() -> Self {
        AmphipodType::Bronze
    }
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Default, Debug, Hash)]
struct AmphipodState {
    t: AmphipodType,
    pos: AmphipodPosition,
}

pub fn _p1(s: &'static str) -> usize {
    let gs = p1::GameStateP1::from_str(s).unwrap();

    shortest_path(gs).unwrap()
}

pub fn p1() -> usize {
    _p1(include_str!("j23.txt"))
}

pub fn _p2(s: &'static str) -> usize {
    let gs = p2::GameStateP2::from_str(s).unwrap();

    shortest_path(gs).unwrap()
}

pub fn p2() -> usize {
    _p2(include_str!("j23.txt"))
}

#[cfg(test)]
mod j23_tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(12521, _p1(include_str!("j23_test.txt")));
        assert_eq!(11320, _p1(include_str!("j23.txt")));
    }

    #[test]
    fn test_p2() {
        assert_eq!(44169, _p2(include_str!("j23_test.txt")));
        assert_eq!(49532, _p2(include_str!("j23.txt")));
    }
}