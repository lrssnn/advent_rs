use std::{fmt::Display, collections::HashMap};

use super::super::day::Day;

pub struct Day14
{
    map: Vec<Vec<Cell>>,
}

impl Day14 {
    pub fn new() -> Day14
    {
        //let input = "O....#....\nO.OO#....#\n.....##...\nOO.#O....O\n.O.....O#.\nO.#..O.#.#\n..O..#O..O\n.......O..\n#....###..\n#OO..#....";
        let input = include_str!("../../input/y2023/14");

        let map = input.lines()
            .map(|line| line.chars().map(Cell::from_char).collect())
            .collect();

        Day14 { map }
    }
}

impl Day for Day14 {
    fn day_name(&self) -> String { String::from("14") }
    fn answer1(&self) -> String { String::from("108759") }
    fn answer2(&self) -> String { String::from("89089") }

    fn part1(&mut self) -> String {
        let mut map = self.map.clone();
        tilt_north_until_stable(&mut map);
        evaluate_load(&mut map).to_string()
    }

    fn part2(&mut self) -> String {
        // println!("0");
        // self._print();
        let mut seen_states = HashMap::<Vec<Vec<Cell>>, usize>::new();
        for cycle in 0..1_000_000_000 {
            spin_cycle_once(&mut self.map);
            if seen_states.contains_key(&self.map) {
                let then = seen_states.get(&self.map).unwrap();
                let loop_length = cycle - then;
                let cycles_left = 1_000_000_000 - cycle;
                let cycles_remaining = (cycles_left % loop_length) - 1;
                for _remainder in 0..cycles_remaining {
                    spin_cycle_once(&mut self.map);
                }
                break;
            }
            seen_states.insert(self.map.clone(), cycle);
        }
        evaluate_load(&mut self.map).to_string()
    }
}

impl Day14 {
    fn _print(&self) {
        for line in &self.map {
            for cell in line {
                print!("{cell}");
            }
            println!();
        }
    }
}

fn spin_cycle_once(map: &mut Vec<Vec<Cell>>) {
    tilt_north_until_stable(map);
    tilt_west_until_stable(map);
    tilt_south_until_stable(map);
    tilt_east_until_stable(map);
}

fn tilt_north_until_stable(map: &mut Vec<Vec<Cell>>) {
    while tilt_north(map) {}
}

fn tilt_south_until_stable(map: &mut Vec<Vec<Cell>>) {
    while tilt_south(map) {}
}

fn tilt_east_until_stable(map: &mut Vec<Vec<Cell>>) {
    while tilt_east(map) {}
}

fn tilt_west_until_stable(map: &mut Vec<Vec<Cell>>) {
    while tilt_west(map) {}
}

fn tilt_north(map: &mut Vec<Vec<Cell>>) -> bool {
    let mut any_moves = false;

    for row in 0..(map.len() - 1) {
        for col in 0..map[0].len() {
            if map[row][col] == Cell::Empty && map[row + 1][col] == Cell::Round {
                map[row][col] = Cell::Round;
                map[row + 1][col] = Cell::Empty;
                any_moves = true;
            }
        }
    }

    any_moves
}

fn tilt_south(map: &mut Vec<Vec<Cell>>) -> bool {
    let mut any_moves = false;

    for row in (1..(map.len())).rev() {
        for col in 0..map[0].len() {
            if map[row][col] == Cell::Empty && map[row - 1][col] == Cell::Round {
                map[row][col] = Cell::Round;
                map[row - 1][col] = Cell::Empty;
                any_moves = true;
            }
        }
    }

    any_moves
}

fn tilt_west(map: &mut Vec<Vec<Cell>>) -> bool {
    let mut any_moves = false;

    for col in 0..(map[0].len() - 1) {
        for row in 0..map.len() {
            if map[row][col] == Cell::Empty && map[row][col + 1] == Cell::Round {
                map[row][col] = Cell::Round;
                map[row][col + 1] = Cell::Empty;
                any_moves = true;
            }
        }
    }

    any_moves
}

fn tilt_east(map: &mut Vec<Vec<Cell>>) -> bool {
    let mut any_moves = false;

    for col in (1..map[0].len()).rev() {
        for row in 0..map.len() {
            if map[row][col] == Cell::Empty && map[row][col - 1] == Cell::Round {
                map[row][col] = Cell::Round;
                map[row][col - 1] = Cell::Empty;
                any_moves = true;
            }
        }
    }

    any_moves
}

fn evaluate_load(map: &Vec<Vec<Cell>>) -> usize {
    let max_load = map.len();
    map.iter().enumerate()
        .map(|(r, row)| (max_load - r) * row.iter()
            .filter(|&c| *c == Cell::Round)
            .count())
        .sum()
}

#[derive(Copy, Clone, PartialEq, Hash, Eq)]
enum Cell {
    Round,
    Square,
    Empty,
}

impl Cell {
    fn from_char(input: char) -> Cell {
        match input {
            'O' => Cell::Round,
            '#' => Cell::Square,
            '.' => Cell::Empty,
            _ => panic!("Unknown Cell")
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Cell::Round => "O",
            Cell::Square => "#",
            Cell::Empty => ".",
        })
    }
}