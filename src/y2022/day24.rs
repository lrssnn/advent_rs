use std::fmt::Display;
use std::hash::Hash;
use crate::search::*;
use crate::two_dimensional::coord::*;
use crate::two_dimensional::direction::*;

use super::super::day::Day;

const HEIGHT: u8 = 35;
const WIDTH: u8 = 100;

//const HEIGHT: u8 = 4;
//const WIDTH: u8 = 6;

const CYCLE: usize = HEIGHT as usize * WIDTH as usize;
const TARGET: Coord<u8> = Coord { x: WIDTH, y: HEIGHT + 1}; // Target the cell above, the rules treat the actual target as a wall
const START: Coord<u8> = Coord { x: 1, y: 0 };

pub struct Day24
{
    blizzards: Vec<Blizzard>,
}

impl Day24 {
    pub fn new() -> Day24
    {
        let input = include_str!("input24");
        //let input = include_str!("input24_example");

        let mut blizzards = Vec::new();

        let lines = input.lines();
        for (y, line) in lines.enumerate().skip(1).take(HEIGHT as usize) {
            for (x, c) in line.chars().enumerate().skip(1).take(WIDTH as usize) {
                if c != '.' {
                   blizzards.push(Blizzard::new(x as u8, y as u8, c));
                }
            }
        }

        Day24 { blizzards, }
    }
}

impl Day for Day24 {
    fn day_name(&self) -> String { String::from("24") }
    fn answer1(&self) -> String { String::from("249") }
    fn answer2(&self) -> String { String::from("735") }

    fn part1(&mut self) -> String {
        let mut blizzards = BlizzardContainer::new(self.blizzards.clone());
        let initial_state = State { player: START, timestamp: 0 };
        let heuristic = | s: &State | (s.player.x.abs_diff(WIDTH) + s.player.y.abs_diff(HEIGHT)) as usize;
        let get_next_states = | s: &State | s.next_states(&mut blizzards);
        let is_desired_state = | s: &State | s.player == TARGET;
        let shortest_path = astar_search(&initial_state, 
            heuristic,
            get_next_states, 
            is_desired_state, 
            usize::MAX
        ).unwrap();
        shortest_path.last().unwrap().timestamp.to_string()
    }

    fn part2(&mut self) -> String {
        let mut blizzards = BlizzardContainer::new(self.blizzards.clone());
        let get_next_states = | s: &State | s.next_states(&mut blizzards);
        let initial_state = State { player: START, timestamp: 0 };

        let heuristic =         | s: &State | (s.player.x.abs_diff(TARGET.x) + s.player.y.abs_diff(TARGET.y)) as usize;
        let heuristic_reverse = | s: &State | (s.player.x.abs_diff(START.x)  + s.player.y.abs_diff(START.y))  as usize;

        let is_end_state   = | s: &State | s.player == TARGET;
        let is_start_state = | s: &State | s.player == START;

        let start_to_end_path = astar_search(&initial_state, 
            heuristic,
            get_next_states, 
            is_end_state, 
            1000
        ).unwrap();

        let start_to_end_ts = start_to_end_path[start_to_end_path.len() - 1].timestamp;
        
        let end_state = State { player: TARGET, timestamp: start_to_end_ts };

        let mut blizzards = BlizzardContainer::new(self.blizzards.clone());

        let get_next_states = | s: &State | s.next_states(&mut blizzards);
        let end_to_start_path = astar_search(&end_state, 
            heuristic_reverse,
            get_next_states, 
            is_start_state, 
            1000
        ).unwrap();

        let end_to_start_ts = end_to_start_path[end_to_start_path.len() - 1].timestamp;

        let restart_state = State { player: START, timestamp: end_to_start_ts };
        let mut blizzards = BlizzardContainer::new(self.blizzards.clone());
        let get_next_states = | s: &State | s.next_states(&mut blizzards);
        let start_to_end_2_path = astar_search(&restart_state, 
            heuristic,
            get_next_states, 
            is_end_state, 
            1000
        ).unwrap();

        let start_to_end_2_ts = start_to_end_2_path[start_to_end_2_path.len() - 1].timestamp;

        start_to_end_2_ts.to_string()
    }
}

#[derive(Clone, PartialEq, Eq)]
struct State {
    player: Coord<u8>,
    timestamp: usize,
}

impl Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.player.hash(state);
        (self.timestamp % CYCLE).hash(state);
    }
}

impl State {
    fn next_states(&self, blizzards: &mut BlizzardContainer) -> Vec<State> {
        let ts = self.timestamp + 1;
        let next_blizzards = blizzards.get(ts);

        let moves: Vec<State> = Direction::all()
            .iter()
            .filter_map(|dir| self.try_move(dir, &next_blizzards))
            .collect();

        moves
    }

    fn try_move(&self, dir: &Direction, blizzards: &[Blizzard]) -> Option<State> {
        let proposed = self.player + *dir;
        if proposed != START && proposed != TARGET && (proposed.x == 0 || proposed.y == 0 || proposed.x == WIDTH + 1 || proposed.y == HEIGHT + 1) { return None; }
        if blizzards.iter().all(|blizz| blizz.loc != proposed) {
            let new_state = State { player: proposed, timestamp: self.timestamp + 1 };
            Some(new_state)
        } else {
            None
        }
    }

    fn _print(&self, blizzards: &mut BlizzardContainer) {
        println!("t =  {}", self.timestamp);
        // Top wall
        for x in 0..=WIDTH {
            if self.player.x == x && self.player.y == 0 {
                print!("E");
            }
            else if x == 1 {
                print!(".");
            }
            else {
                print!("#");
            }
        }
        println!("#");

        // Main rows
        for y in 1..=HEIGHT {
            print!("#");
            for x in 1..=WIDTH {
                if self.player.x == x && self.player.y == y {
                    print!("E");
                }
                else if let Some(b) = blizzards.get(self.timestamp).iter().find(|blizzard| blizzard.loc == (x, y)) {
                     print!("{b}");
                } else {
                    print!(".");
                }
            }
            println!("#");
        }

        // Bottom wall
        for x in 0..=WIDTH {
            if self.player.x == x && self.player.y == HEIGHT + 1 {
                print!("E");
            }
            else if x == WIDTH {
                print!(".");
            }
            else {
                print!("#");
            }
        }
        println!("#")
     }
}


#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Blizzard {
    loc: Coord<u8>,
    btype: Direction,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct BlizzardContainer {
    blizzards: Vec<Vec<Blizzard>>,
}


impl BlizzardContainer {
    fn new(initial_state: Vec<Blizzard>) -> BlizzardContainer {
        BlizzardContainer { blizzards: vec![initial_state] }
    }

    fn get(&mut self, timestamp: usize) -> Vec<Blizzard> {
        let timestamp = timestamp % CYCLE;
        while timestamp >= self.blizzards.len() {
            //println!("Creating map for {}", self.blizzards.len());
            self.blizzards.push(Blizzard::next_blizzards(&self.blizzards[self.blizzards.len() - 1]));
        }
        self.blizzards[timestamp].clone()
    }
}

impl Blizzard {
    fn new(x: u8, y: u8, c: char) -> Blizzard {
        Blizzard { loc: Coord::new(x, y), btype: Direction::from_char(c) }
    }

    fn next_loc(&self) -> Blizzard {
        Blizzard { 
            btype: self.btype, 
            loc: self.loc.wrapped_add(self.btype, (1, WIDTH), (1, HEIGHT))
        }
    }

    fn next_blizzards(bzs: &[Blizzard]) -> Vec<Blizzard> {
        bzs.iter().map(|blizzard| blizzard.next_loc()).collect()
    }
}

impl Display for Blizzard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.btype)
    }
}