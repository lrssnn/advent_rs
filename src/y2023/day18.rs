use std::{fmt::Display, collections::{HashSet, HashMap}};

use crate::two_dimensional::{direction::Direction, coord::Coord as GenericCoord};

type Coord = GenericCoord<isize>;

use super::super::day::Day;

pub struct Day18
{
    instructions: Vec<Instruction>,
}

impl Day18 {
    pub fn new() -> Day18
    {
        //let input = "R 6 (#70c710)\nD 5 (#0dc571)\nL 2 (#5713f0)\nD 2 (#d2c081)\nR 2 (#59c680)\nD 2 (#411b91)\nL 5 (#8ceee2)\nU 2 (#caa173)\nL 1 (#1b58a2)\nU 2 (#caa171)\nR 2 (#7807d2)\nU 3 (#a77fa3)\nL 2 (#015232)\nU 2 (#7a21e3)";
        let input = include_str!("../../input/y2023/18");

        let instructions = input.lines().map(Instruction::from_str).collect();

        Day18 { instructions }
    }
}

impl Day for Day18 {
    fn day_name(&self) -> String { String::from("18") }
    fn answer1(&self) -> String { String::from("41019") }
    fn answer2(&self) -> String { String::from("??") }

    fn part1(&mut self) -> String {
        // for instruction in &self.instructions {
        //     println!("{instruction}");
        // }
        let points = points_from_instructions(&self.instructions);
        //print_points(&points);
        count_points(&points).to_string()
        //String::new()
    }

    fn part2(&mut self) -> String {
        String::new()
    }
}

fn points_from_instructions(instructions: &[Instruction]) -> HashMap<isize, Vec<isize>> {
    let mut result = HashMap::new();
    let mut current_loc = Coord::new(0, 0);

    result.insert(0, vec![0]);

    for instruction in instructions {
        for _step in 0..instruction.distance {
            current_loc = current_loc + instruction.direction;

            if let Some(xs) = result.get_mut(&current_loc.y) {
                xs.push(current_loc.x);
            } else {
                result.insert(current_loc.y, vec![current_loc.x]);
            }
        }
    }
    result
}

fn count_points(points: &HashMap<isize, Vec<isize>>) -> usize {

    let min_y = *points.keys().min().unwrap();
    let min_x = *points.values().min_by_key(|xs| xs.iter().min().unwrap()).unwrap().iter().min().unwrap();

    let max_y = *points.keys().max().unwrap();
    let max_x = *points.values().max_by_key(|xs| xs.iter().max().unwrap()).unwrap().iter().max().unwrap();

    let mut total = 0;
    for y in min_y..=max_y {
        let mut crossings = 0;
        for x in min_x..=max_x {
            if points.get(&y).unwrap().iter().any(|&e| e == x) {
                // print!("#");
                total += 1;
                if points.get(&(y - 1)).unwrap_or(&Vec::new()).iter().any(|&e| e == x) {
                    crossings += 1;
                }
            } else {
                if crossings % 2 == 1 {
                    total += 1;
                    // print!("x");
                } else {
                    // print!(".");
                };
            }
        }
        // println!();
    }
    total
}

fn _print_points(points: &HashMap<isize, Vec<isize>>) {
    // gross!
    let min_y = *points.keys().min().unwrap();
    let min_x = *points.values().min_by_key(|xs| xs.iter().min().unwrap()).unwrap().iter().min().unwrap();

    let max_y = *points.keys().max().unwrap();
    let max_x = *points.values().max_by_key(|xs| xs.iter().max().unwrap()).unwrap().iter().max().unwrap();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if points.get(&y).unwrap().iter().any(|&e| e == x) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

struct Instruction {
    direction: Direction,
    distance: u16,
    colour: String,
}

impl Instruction {
    fn from_str(input: &str) -> Instruction {
        let mut parts = input.split(' ');
        let direction = match parts.next().unwrap() {
            "R" => Direction::Right,
            "L" => Direction::Left,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => panic!("Unknown direction"),
        };

        let distance = parts.next().unwrap().parse().unwrap();

        let colour = parts.next().unwrap().to_string();

        Instruction { direction, distance, colour }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.direction, self.distance, self.colour)
    }
}
