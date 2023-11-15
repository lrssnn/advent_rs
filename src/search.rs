use std::collections::{HashSet, HashMap};
use std::hash::Hash;

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
    let mut open_set = HashSet::new();
    open_set.insert(initial_state.clone());

    // For Point p, came_from[p] is the point immediately preceding it on the cheapest path from
    // start to p currently known
    let mut came_from: HashMap<T, T> = HashMap::new();

    // for point p, g_score[p] is the cost of the cheapest path from start to n currently known
    let mut g_score: HashMap<T, usize> = HashMap::new();
    g_score.insert(initial_state.clone(), 0);

    // f_score would be g_score modified by a heuristic, but we don't have one, so just ignore it
    let mut f_score: HashMap<T, usize> = HashMap::new();
    f_score.insert(initial_state.clone(), heuristic(initial_state));

    while !open_set.is_empty() {
        let current = open_set
            .iter()
            .min_by(|a, b| f_score.get(a).unwrap_or(&usize::MAX).cmp(f_score.get(b).unwrap_or(&usize::MAX)))
            .unwrap()
            .clone();

        if is_desired_state(&current) {
            return Some(reconstruct_path(&came_from, current));
        }

        open_set.remove(&current);

        for neighbour in get_next_states(&current) {
            // We would calculate weight here, but we don't have any
            let tentative_g_score = g_score.get(&current).unwrap() + 1;
            // Give up early if we have gone longer than our currently known shortest
            let give_up = tentative_g_score >= give_up_threshold;
            
            if !give_up && tentative_g_score < *g_score.get(&neighbour).unwrap_or(&usize::MAX) {
                came_from.insert(neighbour.clone(), current.clone());
                g_score.insert(neighbour.clone(), tentative_g_score);
                f_score.insert(neighbour.clone(), tentative_g_score + heuristic(&neighbour));
                
                open_set.insert(neighbour);
            }
        }
    }
    None
}

fn reconstruct_path<T: Eq + Hash + Clone>(came_from: &HashMap<T, T>, endpoint: T) -> Vec<T> {
    // Following the path set out in came_from
    let mut result: Vec<T> = vec![];
    let mut current = endpoint.clone();
    loop {
        result.push(current.clone());
        match came_from.get(&current) {
            Some(parent) => {
                current = parent.clone();
            },
            None => {
                result.reverse();
                return result; 
            }
        }
    }
}