use std::fmt::{Display, Formatter, Write};
use std::str::FromStr;

use smallvec::{IntoIter, SmallVec};

use crate::j23::{AmphipodPosition, AmphipodState, AmphipodType};
use crate::j23::AmphipodPosition::{HallWay, Room};
use crate::j23::dijkstra::GameState;

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Default, Debug, Hash)]
pub struct GameStateP1 {
    amphipod_states: [AmphipodState; 8],
}

impl GameState for GameStateP1 {
    type It = IntoIter<[(GameStateP1, usize); 64]>;

    fn is_game_over(&self) -> bool {
        for elt in &self.amphipod_states {
            match elt {
                AmphipodState { pos: AmphipodPosition::HallWay(_), t: _ } => return false,
                AmphipodState { pos: AmphipodPosition::Room(_, r_id), t: AmphipodType::Amber } if *r_id == 1 || *r_id == 2 || *r_id == 3 => return false,
                AmphipodState { pos: AmphipodPosition::Room(_, r_id), t: AmphipodType::Bronze } if *r_id == 0 || *r_id == 2 || *r_id == 3 => return false,
                AmphipodState { pos: AmphipodPosition::Room(_, r_id), t: AmphipodType::Copper } if *r_id == 0 || *r_id == 1 || *r_id == 3 => return false,
                AmphipodState { pos: AmphipodPosition::Room(_, r_id), t: AmphipodType::Desert } if *r_id == 0 || *r_id == 1 || *r_id == 2 => return false,
                AmphipodState { pos: AmphipodPosition::Room(_, 0), t: AmphipodType::Amber } |
                AmphipodState { pos: AmphipodPosition::Room(_, 1), t: AmphipodType::Bronze } |
                AmphipodState { pos: AmphipodPosition::Room(_, 2), t: AmphipodType::Copper } |
                AmphipodState { pos: AmphipodPosition::Room(_, 3), t: AmphipodType::Desert }
                => {}
                _ => panic!()
            }
        }
        return true;
    }

    fn neighbours(&self) -> Self::It {
        let mut neighbours = SmallVec::<[(GameStateP1, usize); 64]>::new();
        for (id, elt) in self.amphipod_states.iter().enumerate() {
            let target_hallway = match &elt.t {
                AmphipodType::Amber => 2usize,
                AmphipodType::Bronze => 4usize,
                AmphipodType::Copper => 6usize,
                AmphipodType::Desert => 8usize,
            };
            let target_room = match &elt.t {
                AmphipodType::Amber => 0usize,
                AmphipodType::Bronze => 1usize,
                AmphipodType::Copper => 2usize,
                AmphipodType::Desert => 3usize,
            };
            let move_cost_multiplier = match &elt.t {
                AmphipodType::Amber => 1usize,
                AmphipodType::Bronze => 10usize,
                AmphipodType::Copper => 100usize,
                AmphipodType::Desert => 1000usize,
            };
            if let &HallWay(p) = &elt.pos {
                if p < target_hallway {
                    for t in (p + 1)..=target_hallway {
                        if self.amphipod_states.iter().any(|elt2| match elt2.pos {
                            HallWay(t2) if t2 == t => true,
                            HallWay(_) => false,
                            AmphipodPosition::Room(_, _) => false
                        }) {
                            break;
                        }
                        if t == target_hallway {
                            if self.amphipod_states.iter().any(|elt2| match elt2.pos {
                                AmphipodPosition::Room(0, room) if room == target_room => true,
                                _ => false,
                            }) || self.amphipod_states.iter().any(|elt2| match elt2.pos {
                                AmphipodPosition::Room(1, room) if room == target_room => true,
                                _ => false,
                            } && elt2.t != elt.t) {
                                continue;
                            }
                            let mut clone = self.clone();
                            let mut elt_clone = elt.clone();
                            elt_clone.pos = Room(0, target_room);
                            clone.amphipod_states[id] = elt_clone;
                            neighbours.push((clone, move_cost_multiplier * (t - p + 1)));
                            if self.amphipod_states.iter().any(|elt2| match elt2.pos {
                                AmphipodPosition::Room(1, room) if room == target_room => true,
                                _ => false,
                            }) {
                                continue;
                            }
                            let mut clone = self.clone();
                            let mut elt_clone = elt.clone();
                            elt_clone.pos = Room(1, target_room);
                            clone.amphipod_states[id] = elt_clone;
                            neighbours.push((clone, move_cost_multiplier * (t - p + 2)));
                            continue;
                        }
                        continue;
                    }
                } else {
                    if p > 0 {
                        for t in (target_hallway..=(p - 1)).rev() {
                            if self.amphipod_states.iter().any(|elt2| match elt2.pos {
                                HallWay(t2) if t2 == t => true,
                                HallWay(_) => false,
                                AmphipodPosition::Room(_, _) => false
                            }) {
                                break;
                            }
                            if t == target_hallway {
                                if self.amphipod_states.iter().any(|elt2| match elt2.pos {
                                    AmphipodPosition::Room(0, room) if room == target_room => true,
                                    _ => false,
                                }) || self.amphipod_states.iter().any(|elt2| match elt2.pos {
                                    AmphipodPosition::Room(1, room) if room == target_room => true,
                                    _ => false,
                                } && elt2.t != elt.t) {
                                    continue;
                                }
                                let mut clone = self.clone();
                                let mut elt_clone = elt.clone();
                                elt_clone.pos = Room(0, target_room);
                                clone.amphipod_states[id] = elt_clone;
                                neighbours.push((clone, move_cost_multiplier * (p - t + 1)));
                                if self.amphipod_states.iter().any(|elt2| match elt2.pos {
                                    AmphipodPosition::Room(1, room) if room == target_room => true,
                                    _ => false,
                                }) {
                                    continue;
                                }
                                let mut clone = self.clone();
                                let mut elt_clone = elt.clone();
                                elt_clone.pos = Room(1, target_room);
                                clone.amphipod_states[id] = elt_clone;
                                neighbours.push((clone, move_cost_multiplier * (p - t + 2)));
                                continue;
                            }
                            continue;
                        }
                    }
                }
            } else if let &Room(row, col) = &elt.pos {
                if row == 1 && col == target_room {
                    continue;
                }

                if row == 0 && col == target_room && self.amphipod_states.iter().any(|elt2| match elt2.pos {
                    Room(1, col2) if col2 == col => true && elt2.t == elt.t,
                    _ => false,
                }) {
                    continue;
                }


                if row == 1 && self.amphipod_states.iter().any(|elt2| match elt2.pos {
                    Room(0, col2) if col2 == col => true,
                    _ => false,
                }) {
                    continue;
                }
                let p = col * 2 + 2;

                for t in (p + 1)..=10 {
                    if self.amphipod_states.iter().any(|elt2| match elt2.pos {
                        HallWay(t2) if t2 == t => true,
                        HallWay(_) => false,
                        AmphipodPosition::Room(_, _) => false
                    }) {
                        break;
                    }
                    if t == target_hallway {
                        if self.amphipod_states.iter().any(|elt2| match elt2.pos {
                            AmphipodPosition::Room(0, room) if room == target_room => true,
                            _ => false,
                        }) || self.amphipod_states.iter().any(|elt2| match elt2.pos {
                            AmphipodPosition::Room(1, room) if room == target_room => true,
                            _ => false,
                        } && elt2.t != elt.t) {
                            continue;
                        }
                        let mut clone = self.clone();
                        let mut elt_clone = elt.clone();
                        elt_clone.pos = Room(0, target_room);
                        clone.amphipod_states[id] = elt_clone;
                        neighbours.push((clone, move_cost_multiplier * (t - p + 1 + 1 + row)));
                        if self.amphipod_states.iter().any(|elt2| match elt2.pos {
                            AmphipodPosition::Room(1, room) if room == target_room => true,
                            _ => false,
                        }) {
                            continue;
                        }
                        let mut clone = self.clone();
                        let mut elt_clone = elt.clone();
                        elt_clone.pos = Room(1, target_room);
                        clone.amphipod_states[id] = elt_clone;
                        neighbours.push((clone, move_cost_multiplier * (t - p + 2 + 1 + row)));
                        continue;
                    }
                    if t == 2 || t == 4 || t == 6 || t == 8 {
                        continue;
                    }
                    let mut clone = self.clone();
                    let mut elt_clone = elt.clone();
                    elt_clone.pos = HallWay(t);
                    clone.amphipod_states[id] = elt_clone;
                    neighbours.push((clone, move_cost_multiplier * (t - p + 1 + row)));
                }

                for t in (0..=(p - 1)).rev() {
                    if self.amphipod_states.iter().any(|elt2| match elt2.pos {
                        HallWay(t2) if t2 == t => true,
                        HallWay(_) => false,
                        AmphipodPosition::Room(_, _) => false
                    }) {
                        break;
                    }
                    if t == target_hallway {
                        if self.amphipod_states.iter().any(|elt2| match elt2.pos {
                            AmphipodPosition::Room(0, room) if room == target_room => true,
                            _ => false,
                        }) || self.amphipod_states.iter().any(|elt2| match elt2.pos {
                            AmphipodPosition::Room(1, room) if room == target_room => true,
                            _ => false,
                        } && elt2.t != elt.t) {
                            continue;
                        }
                        let mut clone = self.clone();
                        let mut elt_clone = elt.clone();
                        elt_clone.pos = Room(0, target_room);
                        clone.amphipod_states[id] = elt_clone;
                        neighbours.push((clone, move_cost_multiplier * (p - t + 1 + 1 + row)));
                        if self.amphipod_states.iter().any(|elt2| match elt2.pos {
                            AmphipodPosition::Room(1, room) if room == target_room => true,
                            _ => false,
                        }) {
                            continue;
                        }
                        let mut clone = self.clone();
                        let mut elt_clone = elt.clone();
                        elt_clone.pos = Room(1, target_room);
                        clone.amphipod_states[id] = elt_clone;
                        neighbours.push((clone, move_cost_multiplier * (p - t + 2 + 1 + row)));
                        continue;
                    }
                    if t == 2 || t == 4 || t == 6 || t == 8 {
                        continue;
                    }
                    let mut clone = self.clone();
                    let mut elt_clone = elt.clone();
                    elt_clone.pos = HallWay(t);
                    clone.amphipod_states[id] = elt_clone;
                    neighbours.push((clone, move_cost_multiplier * (p - t + 1 + row)));
                }
            }
        }

        neighbours.into_iter()
    }
}

impl Display for GameStateP1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&"#############\n")?;
        f.write_char('#')?;
        for p in 0usize..=10 {
            f.write_char(match self.amphipod_states.iter().find(|elt| match elt.pos {
                HallWay(h) if h == p => true,
                _ => false
            }) {
                None => '.',
                Some(elt) => match elt.t {
                    AmphipodType::Amber => 'A',
                    AmphipodType::Bronze => 'B',
                    AmphipodType::Copper => 'C',
                    AmphipodType::Desert => 'D',
                }
            })?;
        }
        f.write_str(&"#\n")?;
        f.write_str(&"###")?;
        for col in 0..4 {
            f.write_char(match self.amphipod_states.iter().find(|elt| match elt.pos {
                Room(0, col2) if col == col2 => true,
                _ => false
            }) {
                None => '.',
                Some(elt) => match elt.t {
                    AmphipodType::Amber => 'A',
                    AmphipodType::Bronze => 'B',
                    AmphipodType::Copper => 'C',
                    AmphipodType::Desert => 'D',
                }
            })?;
            f.write_char('#')?;
        }
        f.write_str(&"##\n")?;

        f.write_str(&"  #")?;
        for col in 0..4 {
            f.write_char(match self.amphipod_states.iter().find(|elt| match elt.pos {
                Room(1, col2) if col == col2 => true,
                _ => false
            }) {
                None => '.',
                Some(elt) => match elt.t {
                    AmphipodType::Amber => 'A',
                    AmphipodType::Bronze => 'B',
                    AmphipodType::Copper => 'C',
                    AmphipodType::Desert => 'D',
                }
            })?;
            f.write_char('#')?;
        }
        f.write_str(&"  \n")?;
        f.write_str(&"  #########  ")?;
        Ok(())
    }
}

impl FromStr for GameStateP1 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut gs = GameStateP1::default();
        let lines = s.lines().skip(2).take(2);

        for (row, line) in lines.enumerate() {
            for col in 0..4 {
                gs.amphipod_states[row * 4usize + col] = AmphipodState {
                    pos: AmphipodPosition::Room(row, col),
                    t: match line.chars().skip(3usize + col * 2usize).next().unwrap() {
                        'A' => AmphipodType::Amber,
                        'B' => AmphipodType::Bronze,
                        'C' => AmphipodType::Copper,
                        'D' => AmphipodType::Desert,
                        _ => panic!()
                    },
                }
            }
        }
        Ok(gs)
    }
}
