use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fmt::Display;
use std::hash::Hash;

pub trait GameState: Clone + Hash + Eq + PartialEq + PartialOrd + Ord + Display {
    type It: Iterator<Item=(Self, usize)>;
    fn is_game_over(&self) -> bool;
    fn neighbours(&self) -> Self::It;
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State<T: GameState> {
    cost: usize,
    game_state: T,
}

impl<T: GameState> Ord for State<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.game_state.cmp(&other.game_state))
    }
}

impl<T: GameState> PartialOrd for State<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn shortest_path<T: GameState>(start: T) -> Option<usize> {
    let mut dist = HashMap::new();

    let mut heap = BinaryHeap::new();

    dist.insert(start.clone(), 0);
    heap.push(State { cost: 0, game_state: start });

    while let Some(State { cost, game_state }) = heap.pop() {

        if game_state.is_game_over() { return Some(cost); }

        if cost > *dist.get(&game_state).unwrap() { continue; }

        for (neighbour, n_cost) in &mut game_state.neighbours() {
            let next = State { cost: cost + n_cost, game_state: neighbour };

            let old_cost = dist.entry(next.game_state.clone()).or_insert(usize::MAX);
            if next.cost < *old_cost {
                *old_cost = next.cost;
                heap.push(next);
            }
        }
    }

    None
}