use std::collections::HashMap;
use std::str::FromStr;

fn roll(last_roll: usize) -> usize {
    last_roll % 100 + 1
}

fn advance_position(last_pos: usize, steps: usize) -> usize {
    (last_pos - 1 + steps) % 10 + 1
}

#[test]
fn test_roll() {
    let mut last_roll = 0usize;
    for _ in 0..105 {
        last_roll = roll(last_roll);
        dbg!(last_roll);
    }
}

fn simulate(start_p1: usize, start_p2: usize,
            max_score: usize) -> usize {
    let mut p1_score = 0usize;
    let mut p2_score = 0usize;

    let mut p1_position = start_p1;
    let mut p2_position = start_p2;

    let mut dice_last_roll = 100;
    let mut rolls_count = 0usize;

    loop {
        for _ in 0..3 {
            dice_last_roll = roll(dice_last_roll);
            rolls_count += 1;
            p1_position = advance_position(p1_position, dice_last_roll);
        }
        p1_score += p1_position;

        if p1_score >= max_score {
            break;
        }

        for _ in 0..3 {
            dice_last_roll = roll(dice_last_roll);
            rolls_count += 1;
            p2_position = advance_position(p2_position, dice_last_roll);
        }
        p2_score += p2_position;

        if p2_score >= 1000 {
            break;
        }
    }

    rolls_count * p1_score.min(p2_score)
}

#[derive(Clone)]
struct State {
    p1_position: usize,
    p2_position: usize,
    p1_score: usize,
    p2_score: usize,
    p1_rolls: usize,
    p2_rolls: usize,
}

impl State {
    fn step(&self, roll: usize) -> Self {
        let mut clone = self.clone();
        match (self.p1_rolls, self.p2_rolls) {
            (0, _) => {
                clone.p1_rolls += 1;
                clone.p1_position = advance_position(clone.p1_position, roll);
                clone.p1_score += clone.p1_position;
            }
            (1, 0) => {
                clone.p2_rolls += 1;
                clone.p2_position = advance_position(clone.p2_position, roll);
                clone.p2_score += clone.p2_position;
                clone.p1_rolls = 0;
                clone.p2_rolls = 0;
            }
            _ => panic!()
        }
        clone
    }
}

#[test]
fn list_dice_possibilities() {
    let mut moves = HashMap::new();

    for i in 1..=3 {
        for j in 1..=3 {
            for k in 1..=3 {
                *moves.entry(i + j + k).or_insert(0usize) += 1;
            }
        }
    }
    dbg!(moves);
}


fn multiplier(n: usize) -> usize {
    match n {
        3 => 1,
        4 => 3,
        5 => 6,
        6 => 7,
        7 => 6,
        8 => 3,
        9 => 1,
        _ => panic!()
    }
}

fn simulate_universes(start_p1: usize, start_p2: usize,
                      max_score: usize) -> usize {
    let mut states = Vec::new();

    states.push((State {
        p1_position: start_p1,
        p2_position: start_p2,
        p1_score: 0,
        p2_score: 0,
        p1_rolls: 0,
        p2_rolls: 0,
    }, 1usize));

    let mut p1_wins = 0usize;
    let mut p2_wins = 0usize;

    while let Some((s, m)) = states.pop() {
        for d in [3, 4, 5, 6, 7, 8, 9].iter() {
            let ns = s.step(*d);
            let new_multiplier = multiplier(*d)*m;
            match (ns.p1_score >= ns.p2_score, ns.p1_score.max(ns.p2_score)) {
                (_, score) if score < max_score => {
                    states.push((ns, new_multiplier));
                }
                (true, _) => {
                    p1_wins += new_multiplier;
                }
                (false, _) => {
                    p2_wins += new_multiplier;
                }
            }
        }
    }

    p1_wins.max(p2_wins)
}

pub fn _p1(s: &'static str) -> usize {
    let start_p1_pos = usize::from_str(&s.lines().next().unwrap()[
        "Player 1 starting position: ".len()..
        ]).unwrap();
    let start_p2_pos = usize::from_str(&s.lines().skip(1).next().unwrap()[
        "Player 2 starting position: ".len()..
        ]).unwrap();

    simulate(start_p1_pos, start_p2_pos, 1000)
}

pub fn p1() -> usize {
    _p1(include_str!("j21.txt"))
}

pub fn _p2(s: &'static str) -> usize {
    let start_p1_pos = usize::from_str(&s.lines().next().unwrap()[
        "Player 1 starting position: ".len()..
        ]).unwrap();
    let start_p2_pos = usize::from_str(&s.lines().skip(1).next().unwrap()[
        "Player 2 starting position: ".len()..
        ]).unwrap();

    simulate_universes(start_p1_pos, start_p2_pos, 21)
}

pub fn p2() -> usize {
    _p2(include_str!("j21.txt"))
}

#[cfg(test)]
mod j21_tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(739785, _p1(include_str!("j21_test.txt")));
        assert_eq!(864900, _p1(include_str!("j21.txt")));
    }

    #[test]
    fn test_p2() {
        assert_eq!(444356092776315, _p2(include_str!("j21_test.txt")));
        assert_eq!(575111835924670, _p2(include_str!("j21.txt")));
    }
}