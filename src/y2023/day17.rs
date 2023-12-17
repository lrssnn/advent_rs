use std::collections::{HashMap, HashSet};

use rayon::current_num_threads;

use crate::{search::astar_search, two_dimensional::direction::Direction, two_dimensional::coord::Coord as GenericCoord};

type Coord = GenericCoord<usize>;

use super::super::day::Day;

pub struct Day17
{
    map: Vec<Vec<Tile>>,
}

impl Day17 {
    pub fn new() -> Day17
    {
        //let input = "2413432311323\n3215453535623\n3255245654254\n3446585845452\n4546657867536\n1438598798454\n4457876987766\n3637877979653\n4654967986887\n4564679986453\n1224686865563\n2546548887735\n4322674655533";
        let input = include_str!("../../input/y2023/17");

        let map = input.lines().map(|line| line.chars().map(Tile::from_char).collect()).collect();

        Day17 { map }
    }

    fn search(&self, initial_state: &State) -> usize {
        let desired_loc = Coord { x: self.map[0].len() - 1, y: self.map.len() - 1};
        let is_desired_state = | s: &State | s.location == desired_loc;

        let mut states_to_search = vec![initial_state.clone()];

        let mut seen_states = HashMap::new();

        let mut best_score = usize::MAX;

        while let Some(current_state) = states_to_search.pop() {
            //println!("\nLooking at {current_state:?}");
            if states_to_search.len() % 100 == 0 {print!("\rBest Score {best_score} | To Check {}           ", states_to_search.len());}
        
            if is_desired_state(&current_state) {
                best_score = best_score.min(current_state.accumulated_cost);
                continue;
            } else if current_state.accumulated_cost >= best_score {
                continue;
            }
            
            if let Some(previous_best_cost) = seen_states.get(&(current_state.location, current_state.last_dir)) {
                //println!("Have been here ({}, {}) before, cost {}", current_state.location, current_state.last_dir, previous_best_cost);
               if current_state.accumulated_cost < *previous_best_cost {
                    seen_states.insert((current_state.location, current_state.last_dir), current_state.accumulated_cost);
               } else {
                continue;
               }
            }

            seen_states.insert((current_state.location, current_state.last_dir), current_state.accumulated_cost);

            states_to_search.extend(current_state.next_states(&self.map));
            // println!("states to search after extend:");
            // println!("{states_to_search:?}");
        }

        best_score
    }
}

impl Day for Day17 {
    fn day_name(&self) -> String { String::from("17") }
    fn answer1(&self) -> String { String::from("??") }
    fn answer2(&self) -> String { String::from("??") }

    fn part1(&mut self) -> String {
        let initial_state = State { 
            location: Coord { x: 0, y: 0},
            steps: 0,
            last_dir: Direction::Right,
            accumulated_cost: 0,
        };

        self.search(&initial_state).to_string()
    }

    fn part2(&mut self) -> String {
        String::new()
    }
}

#[derive(Hash ,PartialEq, Eq, Clone, Copy, Debug)]
struct State {
    location: Coord,
    steps: u8,
    last_dir: Direction,
    accumulated_cost: usize,
}
impl State {
    fn next_states(&self, map: &[Vec<Tile>]) -> Vec<State> {
        let mut results = vec![];
        // Turn left, go straight (if possible), or turn right
        let bounds_x = (0, map[0].len() - 1);
        let bounds_y = (0, map.len() - 1);
        // println!("Valid options from {self:?}");
        if let Some(turned_left) = self.location.try_move(self.last_dir.turn_left(), bounds_x, bounds_y) {
            // println!("Can Turn Left");
            results.push(State { 
                location: turned_left,
                steps: 1,
                last_dir: self.last_dir.turn_left(),
                accumulated_cost: self.accumulated_cost + map[turned_left.y][turned_left.x].cost as usize,
             });
        }

        if let Some(turned_right) = self.location.try_move(self.last_dir.turn_right(), bounds_x, bounds_y) {
            // println!("Can Turn Right");
            results.push(State { 
                location: turned_right,
                steps: 1,
                last_dir: self.last_dir.turn_right(),
                accumulated_cost: self.accumulated_cost + map[turned_right.y][turned_right.x].cost as usize,
             });
        }

        if self.steps < 3 {
            if let Some(went_straight) = self.location.try_move(self.last_dir, bounds_x, bounds_y) {
                // println!("Can go straight");
                results.push(State {
                    location: went_straight,
                    steps: self.steps + 1,
                    last_dir: self.last_dir,
                    accumulated_cost: self.accumulated_cost + map[went_straight.y][went_straight.x].cost as usize,
                })
            }
        }

        results
    }
}

struct Tile {
    cost: u8,
}

impl Tile {
    fn from_char(input: char) -> Tile {
        let cost = input.to_string().parse().unwrap();
        Tile { cost }
    }
}