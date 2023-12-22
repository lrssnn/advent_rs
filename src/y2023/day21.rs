use std::collections::HashSet;

use crate::two_dimensional::{coord::Coord as GenericCoord, direction::Direction};

use super::super::day::Day;

type Coord = GenericCoord<isize>;

//const NUM_STEPS: usize = 6;
const NUM_STEPS: usize = 64;

pub struct Day21
{
    map: Vec<Vec<bool>>,
    start: Coord,
}

impl Day21 {
    pub fn new() -> Day21
    {
        let input = "...........\n.....###.#.\n.###.##..#.\n..#.#...#..\n....#.#....\n.##..S####.\n.##..#...#.\n.......##..\n.##.#.####.\n.##..##.##.\n...........";
        //let input = include_str!("../../input/y2023/21");
        let (map, start) = map_from_str(&input);

        Day21 { map, start }
    }
}

impl Day for Day21 {
    fn day_name(&self) -> String { String::from("21") }
    fn answer1(&self) -> String { String::from("3782") }
    fn answer2(&self) -> String { String::from("??") }

    fn part1(&mut self) -> String {
        //print(&self.map, &self.start, &HashSet::new());
        let mut starting_set = HashSet::new();
        starting_set.insert(self.start.clone());
        let mut state = State { possible_locs: starting_set, steps_taken: 0 };
        for _step in 1..=NUM_STEPS {
            state = self.next_state(&state);
            // println!("After {step} steps...");
            // print(&self.map, &self.start, &state.possible_locs);
        }
        state.possible_locs.len().to_string()
    }

    fn part2(&mut self) -> String {
        // Yes, redoing the first 64 steps, or... 0.0002 % of the total work
        let mut starting_set = HashSet::new();
        starting_set.insert(self.start.clone());
        let interested_in = vec![6, 10, 50, 100, 500, 1000, 5000];
        let mut state = State { possible_locs: starting_set, steps_taken: 0 };
        for step in 1..=26501365 {
            state = self.next_state(&state);
            if interested_in.contains(&step) {
                println!("After {step} steps => {}", state.possible_locs.len());
            }
        }
        state.possible_locs.len().to_string()
    }
}

struct State {
    possible_locs: HashSet<Coord>,
    steps_taken: usize,
}

impl Day21 {
    fn next_state(&self, state: &State) -> State {
        let possible_locs = state.possible_locs.iter().flat_map(|coord| self.legal_moves(coord)).collect();
        State { possible_locs, steps_taken: state.steps_taken + 1}
    }

    fn legal_moves(&self, coord: &Coord) -> HashSet<Coord> {
        let mut result = HashSet::new();
        for d in Direction::all() {
            if d == Direction::None { continue; }
            let next = *coord + d;
            let y = next.y.abs() as usize % self.map.len() as usize;
            let x = next.x.abs() as usize % self.map[0].len() as usize;
            if self.map[y][x] {
                result.insert(next);
            }
        }
        result
    }
}

fn map_from_str(input: &str) -> (Vec<Vec<bool>>, Coord) {
    let mut start = Coord::new(0, 0);
    let mut result = vec![];
    for (r, row) in input.lines().enumerate() {
        let mut line = vec![];
        for (c, col) in row.chars().enumerate() {
            line.push(match col {
                '.' => true,
                '#' => false,
                'S' => {
                    start = Coord::new(c as isize, r as isize);
                    true
                },
                _ => panic!("invalid char")
            })
        }
        result.push(line);
    }
    (result, start)
}

fn print(map: &Vec<Vec<bool>>, start: &Coord, reachable: &HashSet<Coord>) {
    for (r, row) in map.iter().enumerate() {
        for (c, open) in row.iter().enumerate() {
            let this_coord = Coord::new(c as isize, r as isize);
            if reachable.contains(&this_coord) {
                print!("O");
            } else if start.eq(&this_coord) {
                print!("S");
            } else if *open {
                print!(".");
            } else {
                print!("#");
            }
        }
        println!();
    }
}