use std::{fmt::Display, ops::Add };
use std::hash::{Hash, Hasher};
use crate::search::*;

use super::super::day::Day;

//const HEIGHT: u8 = 35;
//const WIDTH: u8 = 100;

const HEIGHT: u8 = 4;
const WIDTH: u8 = 6;

const CYCLE: usize = HEIGHT as usize * WIDTH as usize;
const TARGET: Coord = Coord { x: WIDTH, y: HEIGHT}; // Target the cell above, the rules treat the actual target as a wall
const START: Coord = Coord { x: 1, y: 0 };

pub struct Day24
{
    blizzards: Vec<Blizzard>,
}

impl Day24 {
    pub fn new() -> Day24
    {
        let input = include_str!("input24");
        let input = include_str!("input24_example");

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
    fn answer2(&self) -> String { String::from("?") }

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
        shortest_path.len().to_string()
    }

    fn part2(&mut self) -> String {
        let mut blizzards = BlizzardContainer::new(self.blizzards.clone());
        let get_next_states = | s: &State | s.next_states(&mut blizzards);
        let initial_state = State { player: START, timestamp: 0 };
        let heuristic = | s: &State | {
            let h = (s.player.x.abs_diff(WIDTH) + s.player.y.abs_diff(HEIGHT)) as usize;
            println!("Heuristic for {} = {}", s.player, h);
            h
        };
        let heuristic_reverse = | s: &State | {
            let h = (s.player.x.abs_diff(1) + s.player.y) as usize;
            println!("H reverse of {} = {}", s.player, h);
            h
        };
        let is_end_state = | s: &State | s.player == TARGET;
        let is_start_state = | s: &State | {
            let winner = s.player == START;
            println!("Is {} a winner? {winner}", s.player);
            winner
        };

        let start_to_end = astar_search(&initial_state, 
            heuristic,
            get_next_states, 
            is_end_state, 
            usize::MAX
        ).unwrap().len();

        let end_state = State { player: TARGET, timestamp: start_to_end };
        let mut blizzards = BlizzardContainer::new(self.blizzards.clone());
        let get_next_states = | s: &State | s.next_states(&mut blizzards);
        let end_to_start = astar_search(&end_state, 
            heuristic_reverse,
            get_next_states, 
            is_start_state, 
            usize::MAX
        ).unwrap().len();

        let restart_state = State { player: START, timestamp: start_to_end + end_to_start };
        let mut blizzards = BlizzardContainer::new(self.blizzards.clone());
        let get_next_states = | s: &State | s.next_states(&mut blizzards);
        let start_to_end_2 = astar_search(&restart_state, 
            heuristic,
            get_next_states, 
            is_end_state, 
            usize::MAX
        ).unwrap().len();

        let total = start_to_end + end_to_start + start_to_end_2;
        println!("{start_to_end} + {end_to_start} + {start_to_end_2} = {total}");
        total.to_string()
    }
}

#[derive(Clone, Eq)]
struct State {
    player: Coord,
    timestamp: usize,
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.player.hash(state);
        (self.timestamp % CYCLE as usize).hash(state);
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        let my_ts = self.timestamp % CYCLE;
        let other_ts = other.timestamp % CYCLE;
        self.player == other.player 
        && my_ts == other_ts
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

        println!("From...\n");
        self._print(blizzards);
        println!("To...\n");
        for s in &moves {
            s._print(blizzards);
            println!();
        }
        moves
    }

    fn try_move(&self, dir: &Direction, blizzards: &[Blizzard]) -> Option<State> {
        let proposed = self.player + dir.offset();
        if proposed != START && (proposed.x == 0 || proposed.y == 0 || proposed.x == WIDTH + 1 || proposed.y == HEIGHT + 1) { return None; }
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
enum Direction {
    Up, Down, Left, Right, None
}

impl Direction {
    fn from_char(c: char) -> Direction {
        match c  {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("Unknown char type")
        }
    }

    fn offset(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0 , 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::None => (0, 0),
        }
    }

    fn all() -> Vec<Direction> {
        vec![Direction::Down, Direction::Right, Direction::Up, Direction::Left, Direction::None]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Blizzard {
    loc: Coord,
    btype: Direction,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct BlizzardContainer {
    blizzards: Vec<Vec<Blizzard>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Coord {
    x: u8,
    y: u8,
}

impl Coord {
    fn new(x: u8, y: u8) -> Coord {
        Coord { x, y }
    }
}

impl PartialEq<(u8, u8)> for Coord {
    fn eq(&self, other: &(u8, u8)) -> bool {
        self.x == other.0 && self.y == other.1
    }
}

impl Add<(isize, isize)> for Coord {
    fn add(self, rhs: (isize, isize)) -> Self {
        let x = ((self.x as isize) + rhs.0).max(0) as u8;
        let y = ((self.y as isize) + rhs.1).max(0) as u8;
        Coord { x, y }
    }

    type Output = Self;
}

impl BlizzardContainer {
    fn new(initial_state: Vec<Blizzard>) -> BlizzardContainer {
        BlizzardContainer { blizzards: vec![initial_state] }
    }

    fn get(&mut self, timestamp: usize) -> Vec<Blizzard> {
        let timestamp = timestamp % CYCLE;
        while timestamp >= self.blizzards.len() {
            println!("Creating map for {}", self.blizzards.len());
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
        let mut next = self.loc + self.btype.offset();
        if next.x == 0 { next.x = WIDTH; }
        if next.x == WIDTH + 1 { next.x = 1;}
        if next.y == 0 { next.y = HEIGHT; }
        if next.y == HEIGHT + 1 { next.y = 1;}
        Blizzard { btype: self.btype, loc: next}
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

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
            Direction::None => '.',
        })
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}