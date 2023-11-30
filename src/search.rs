use std::collections::{HashMap, BinaryHeap};
use std::hash::Hash;

use num::Zero;

pub fn astar_search<
    T: Eq + Hash + Clone,
    H: Fn(&T) -> usize,
    S: FnMut(&T) -> Vec<T>,
    W: Fn(&T) -> bool>
    (initial_state: &T, heuristic: H, mut get_next_states: S, is_desired_state: W, give_up_threshold: usize) -> Option<Vec<T>> {
    // Copied from pseudoCode on wikipedia, plus slight rust help from `pathfinding` crate source.
    // But that is more optimised. Complate lack of borrows here in favour of copies feels so not rust

    // The set of discovered points that need to be expanded. Intially, only start is known.
    // If this becomes a BinaryHeap, it becomes faster to find 'current' below
    let mut open_set = BinaryHeap::new();
    open_set.push(CostHolder {
        heuristic_cost: Zero::zero(),
        true_cost: Zero::zero(),
        state: initial_state.clone(),
    });

    // For Point p, came_from[p] is the point immediately preceding it on the cheapest path from
    // start to p currently known and the cost to get there
    let mut came_from: HashMap<T, (T, usize)> = HashMap::new();

    while let Some(CostHolder { true_cost, state: current, ..}) = open_set.pop() {

        if is_desired_state(&current) {
            return Some(reconstruct_path(&came_from, current));
        }

        for neighbour in get_next_states(&current) {
            let neighbour_cost = true_cost + 1;

            // Give up early if we have gone longer than our currently known shortest
            if neighbour_cost >= give_up_threshold {
                continue;
            }
            
            if neighbour_cost < came_from.get(&neighbour).unwrap_or(&(initial_state.clone(), usize::MAX)).1 {
                came_from.insert(neighbour.clone(), (current.clone(), neighbour_cost));
                
                open_set.push(CostHolder { 
                    heuristic_cost: neighbour_cost + heuristic(&neighbour), 
                    true_cost: neighbour_cost,
                     state: neighbour 
                });
            }
        }
    }
    None
}

fn reconstruct_path<T: Eq + Hash + Clone>(came_from: &HashMap<T, (T, usize)>, endpoint: T) -> Vec<T> {
    // Following the path set out in came_from
    let mut result: Vec<T> = vec![];
    let mut current = endpoint.clone();
    loop {
        result.push(current.clone());
        match came_from.get(&current) {
            Some((parent, _)) => {
                current = parent.clone();
            },
            None => {
                result.reverse();
                return result; 
            }
        }
    }
}

struct CostHolder<S, T> {
    heuristic_cost: S,
    true_cost: S,
    state: T,
}

impl<S: PartialEq, T> PartialEq for CostHolder<S, T> {
    fn eq(&self, other: &Self) -> bool {
        self.heuristic_cost.eq(&other.heuristic_cost)
        && self.true_cost.eq(&other.true_cost)
    }
}

impl <S: PartialEq, T> Eq for CostHolder<S, T> {}

impl <S: Ord, T> PartialOrd for CostHolder<S, T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl <S: Ord, T> Ord for CostHolder<S, T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match other.heuristic_cost.cmp(&self.heuristic_cost) {
            std::cmp::Ordering::Equal => self.true_cost.cmp(&other.true_cost),
            s => s,
        }
    }
}