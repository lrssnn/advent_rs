use std::fmt::Display;

use rayon::prelude::{IntoParallelRefIterator, ParallelIterator, IntoParallelRefMutIterator};

use super::super::day::Day;

pub struct Day23
{
    elves: Vec<Elf>,
    directions: Vec<Vec<Direction>>,
}

impl Day23 {
    pub fn new() -> Day23
    {
        let input = include_str!("input23");
        //let input = include_str!("input23_example");
        //let input = include_str!("input23_example_small");

        let elves = input.lines().enumerate().flat_map(|(row, line)| {
            line.chars().enumerate().filter_map(move |(col, c)| if c == '#' { Some(Elf { x: col as isize, y: row as isize} )} else { None } )
        }).collect();

        Day23 { elves, directions: Direction::default() }
    }

    #[allow(unused)]
    fn print_elves(&self) {
        let min_x = self.elves.iter().map(|e| e.x).min().unwrap();
        let max_x = self.elves.iter().map(|e| e.x).max().unwrap();
        let min_y = self.elves.iter().map(|e| e.y).min().unwrap();
        let max_y = self.elves.iter().map(|e| e.y).max().unwrap();

        for row in min_y..=max_y {
            for col in min_x..=max_x {
                if self.elves.iter().any(|e| e.x == col && e.y == row) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn score(&self) -> usize {
        let min_x = self.elves.iter().map(|e| e.x).min().unwrap();
        let max_x = self.elves.iter().map(|e| e.x).max().unwrap();
        let min_y = self.elves.iter().map(|e| e.y).min().unwrap();
        let max_y = self.elves.iter().map(|e| e.y).max().unwrap();

        let mut score = 0;
        for row in min_y..=max_y {
            for col in min_x..=max_x {
                if !self.elves.iter().any(|e| e.x == col && e.y == row) {
                    score += 1;
                }
            }
        }
        score
    }

    fn elf_has_any_neighbour(&self, elf: &Elf) -> bool {
        for other in &self.elves {
            if other == elf { continue; }
            if elf.is_adjacent_to(other) { return true; }
        }
        false
    }

    fn elf_has_neighbour(&self, elf: &Elf, direction: Direction) -> bool {
        let offset = direction.offset();
        self.elves.iter().any(|other| other.x - offset.0 == elf.x && other.y - offset.1 == elf.y)
    }

    fn process_turns(&mut self, turns: usize) -> usize {
        for _turn in 1..=turns {
            let no_moves = self.process_turn();

            if no_moves { return _turn; }
            self.directions.reverse();

            // rotate the directions queue
            let first = self.directions.pop().unwrap();
            self.directions.reverse();
            self.directions.push(first);
        }

        return turns
    }

    fn process_turn(&mut self) -> bool {
        let proposals: Vec<(Elf, (isize, isize))> = self.elves.par_iter().filter_map(|elf| {
            if !self.elf_has_any_neighbour(elf) {
                return None;
            }
            for dir_set in &self.directions {
                if dir_set.iter().all(|&dir| !self.elf_has_neighbour(elf, dir)) {
                    let offset = dir_set[0].offset();
                    let target = (elf.x + offset.0, elf.y + offset.1);
                    return Some((*elf, target));
                }
            }
            return None;
        }).collect();

        // Deduplicate proposals aiming for the same place
        let proposals: Vec<(Elf, (isize, isize))> = proposals.par_iter().filter(|(_, pos)| 
            proposals.iter().filter(|(_,other)| other.0 == pos.0 && other.1 == pos.1).count() == 1)
            .cloned().collect();

        if proposals.is_empty() { return true; }

        // Apply the moves
        self.elves.par_iter_mut().for_each(|elf| {
            if let Some((_, pos)) = proposals.iter().find(|(other, _)| other.x == elf.x && other.y == elf.y) {
                elf.x = pos.0;
                elf.y = pos.1;
            }
        });

        return false;
    }
}

impl Day for Day23 {
    fn day_name(&self) -> String { String::from("23") }
    fn answer1(&self) -> String { String::from("3947") }
    fn answer2(&self) -> String { String::from("1012") }

    fn part1(&mut self) -> String {
        self.process_turns(10);
        self.score().to_string()
    }

    fn part2(&mut self) -> String {
        let turns = self.process_turns(1_000_000);
        (turns + 10).to_string()
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    N, S, E, W,
    NE, NW, SE, SW,
}

impl Direction {
    fn default() -> Vec<Vec<Direction>> {
        vec![
            vec![Direction::N, Direction::NE, Direction::NW],
            vec![Direction::S, Direction::SE, Direction::SW],
            vec![Direction::W, Direction::NW, Direction::SW],
            vec![Direction::E, Direction::NE, Direction::SE],
        ]
    }

    fn offset(&self) -> (isize, isize) {
        match self {
            Direction::N =>  ( 0, -1),
            Direction::S =>  ( 0,  1),
            Direction::E =>  ( 1,  0),
            Direction::W =>  (-1,  0),
            Direction::NE => ( 1, -1),
            Direction::NW => (-1, -1),
            Direction::SE => ( 1,  1),
            Direction::SW => (-1,  1),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Elf {
    x: isize,
    y: isize,
}

impl Elf {
    fn is_adjacent_to(&self, other: &Elf) -> bool {
        // We assume we will not be called with ourself
        self.x.abs_diff(other.x) <= 1 && self.y.abs_diff(other.y) <= 1
    }
}

impl Display for Elf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
