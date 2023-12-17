#[allow(unused_imports)]
use std::{fmt::Display, collections::{HashSet, HashMap}};

use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::two_dimensional::{direction::Direction, coord::Coord as GenericCoord};

use super::super::day::Day;

type Coord = GenericCoord<usize>;

pub struct Day16
{
    grid: Vec<Vec<Tile>>,
}

impl Day16 {
    pub fn new() -> Day16
    {
        //let input = ".|...\\....\n|.-.\\.....\n.....|-...\n........|.\n..........\n.........\\\n..../.\\\\..\n.-.-/..|..\n.|....-|.\\\n..//.|....";
        let input = include_str!("../../input/y2023/16");

        let grid = input.lines().map(|line| line.chars().map(Tile::from_char).collect()).collect();

        Day16 { grid }
    }

    fn count_energised(&self, beam_location: &Coord, beam_direction: &Direction) -> usize {
        let mut visited = HashSet::new();

        let mut beams = vec![(*beam_location, *beam_direction)];

        while let Some((mut location, mut direction)) = beams.pop() {
            loop {
                if visited.contains(&(location, direction)) {
                    // Cycle
                    break;
                }

                visited.insert((location, direction));

                let (next_direction, maybe_other_direction) = self.grid[location.y][location.x].process_beam(&direction);

                // Enqueue the second direction to come back to later
                if let Some(other_direction) = maybe_other_direction {
                    if let Some(other_location) = self.travel(&location, &other_direction) {
                        beams.push((other_location, other_direction));
                    }
                }

                // Travel if we can, otherwise break out of loop to get the next beam in the queue
                if let Some(next_location) = self.travel(&location, &next_direction) {
                    location = next_location;
                    direction = next_direction;
                } else {
                    break;
                }
            }
        }

        // For cycle detection visited is coord and direction, for scoring we only care about unique directions
        visited.iter().map(|(coord, _direction)| coord).collect::<HashSet<_>>().len()
    }

    fn travel(&self, beam_location: &Coord, beam_direction: &Direction) -> Option<Coord> {
        match beam_direction {
            Direction::Up => if beam_location.y == 0 { None } else { Some(Coord { x: beam_location.x, y: beam_location.y - 1 })}
            Direction::Down => if beam_location.y == self.grid.len() - 1 { None } else { Some(Coord { x: beam_location.x, y: beam_location.y + 1})}
            Direction::Left => if beam_location.x == 0 { None } else { Some(Coord { x: beam_location.x - 1, y: beam_location.y })}
            Direction::Right => if beam_location.x == self.grid[0].len() - 1 { None } else { Some(Coord { x: beam_location.x + 1, y: beam_location.y})}
            Direction::None => panic!(),
        }
    }
}

impl Day for Day16 {
    fn day_name(&self) -> String { String::from("16") }
    fn answer1(&self) -> String { String::from("7074") }
    fn answer2(&self) -> String { String::from("7530") }

    fn part1(&mut self) -> String {
        self.count_energised(&Coord { x: 0, y: 0 }, &Direction::Right).to_string()
    }

    fn part2(&mut self) -> String {
        let top = (0..self.grid[0].len()).map(|x| (Coord{ x, y: 0 }, Direction::Down) );
        let bottom = (0..self.grid[0].len()).map(|x| (Coord{ x, y: self.grid.len() - 1 }, Direction::Up) );
        let left = (0..self.grid.len()).map(|y| (Coord{ x: 0, y }, Direction::Right) );
        let right = (0..self.grid.len()).map(|y| (Coord{ x: self.grid[0].len() - 1, y }, Direction::Left) );

        let all_starts = top.chain(bottom).chain(left).chain(right).par_bridge();

        all_starts
            .map(|(loc, d)| self.count_energised(&loc, &d))
            .max().unwrap()
            .to_string()
    }
}

enum Tile {
    Empty,
    Mirror(Direction), // Direction defines direction beam will leave when entering from the left
    Splitter(Direction), // Direction Up or Right defines a vertical or horizontal splitter
}

impl Tile {
    fn from_char(input: char) -> Self {
        match input {
            '.' => Tile::Empty,
            '|' => Tile::Splitter(Direction::Up),
            '-' => Tile::Splitter(Direction::Right),
            '/' => Tile::Mirror(Direction::Up),
            '\\' => Tile::Mirror(Direction::Down),
            x => panic!("Unknown tile: {x}"),
        }
    }

    fn process_beam(&self, beam_direction: &Direction) -> (Direction, Option<Direction>) {
        match self {
            Tile::Empty => (*beam_direction, None),
            Tile::Mirror(d) => match d {
                Direction::Up => (match beam_direction {
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Down,
                    Direction::Right => Direction::Up,
                    _ => panic!(),
                }, None),
                Direction::Down => (match beam_direction {
                    Direction::Up => Direction::Left,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                    _ => panic!(),
                }, None),
                _ => panic!("Invalid mirror direction"),
            }
            Tile::Splitter(d) => match d {
                Direction::Up => match beam_direction {
                    Direction::Up | Direction::Down => (*beam_direction, None),
                    Direction::Left | Direction::Right => (Direction::Up, Some(Direction::Down)),
                    _ => panic!(),
                }
                Direction::Right => match beam_direction {
                    Direction::Left | Direction::Right => (*beam_direction, None),
                    Direction::Up | Direction::Down => (Direction::Left, Some(Direction::Right)),
                    _ => panic!(),
                }
                _ => panic!("Invalid splitter direction"),
            }
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Tile::Empty => '.',
            Tile::Mirror(d) => if d.eq(&Direction::Up) { '/' } else { '\\' },
            Tile::Splitter(d) => if d.eq(&Direction::Up) { '|' } else { '-' },
        })
    }
}