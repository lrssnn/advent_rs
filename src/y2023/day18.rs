use std::{fmt::Display, collections::HashMap};

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
        //_print_points(&points);
        // for row in points.keys() {

        //     let right = evaluate_row_dumb(*row, &points, false);
        //     let new = evaluate_row((row, points.get(&row).unwrap()), false);

        //     if right != new {
        //         println!("{new} - should be {right}");
        //         _print_row(*row, &points);
        //         evaluate_row_dumb(*row, &points, true);
        //         evaluate_row((row, points.get(&row).unwrap()), true);
        //         println!();
        //     }
        // }

        count_points_dumb(&points).to_string()
    }

    #[allow(unreachable_code)]
    fn part2(&mut self) -> String {
        println!();
        let decoded = self.instructions.iter().map(|i| {
            let decoded = i.decoded();
            //println!("{i} -> {decoded}");
            decoded
        }).collect::<Vec<_>>();
        let points = points_from_instructions(&decoded);
        //print_points(&points);
        count_points_dumb(&points).to_string()
    }
}

fn points_from_instructions(instructions: &[Instruction]) -> HashMap<isize, Vec<isize>> {
    let mut result = HashMap::new();
    let mut current_loc = Coord::new(0, 0);

    result.insert(0, vec![0]);

    for (i, instruction) in instructions.iter().enumerate() {
        for _step in 0..instruction.distance {
            current_loc = current_loc + instruction.direction;

            if let Some(xs) = result.get_mut(&current_loc.y) {
                xs.push(current_loc.x);
            } else {
                result.insert(current_loc.y, vec![current_loc.x]);
            }
        }
        print!("\rProcessed instruction {i} out of {}   ", instructions.len());
    }
    println!();
    result
}

fn count_points_dumb(points: &HashMap<isize, Vec<isize>>) -> usize {
    let min_y = *points.keys().min().unwrap();
    let max_y = *points.keys().max().unwrap();

    let mut total = 0;
    let rows = max_y - min_y;
    for y in min_y..=max_y {
        total += evaluate_row_dumb(y, points, false);
        print!("\r{}/{rows}", y - min_y);
    }
    total
}

fn evaluate_row_dumb(y: isize, points: &HashMap<isize, Vec<isize>>, print: bool) -> usize {
    let min_x = *points.values().min_by_key(|xs| xs.iter().min().unwrap()).unwrap().iter().min().unwrap();
    let max_x = *points.values().max_by_key(|xs| xs.iter().max().unwrap()).unwrap().iter().max().unwrap();

    let mut crossings = 0;
    let mut total = 0;
    for x in min_x..=max_x {
        if points.get(&y).unwrap().iter().any(|&e| e == x) {
            if print { print!("#"); }
            total += 1;
            if points.get(&(y - 1)).unwrap_or(&Vec::new()).iter().any(|&e| e == x) {
                crossings += 1;
            }
        } else {
            if crossings % 2 == 1 {
                total += 1;
                if print { print!("x"); }
            } else {
                if print { print!("."); }
            };
        }
    }
    if print { println!(); }
    total
}

fn _count_points(points: &HashMap<isize, Vec<isize>>) -> usize {
    points.iter().map(|row| _evaluate_row(row, false)).sum()
}

fn _evaluate_row(row: (&isize, &Vec<isize>), print: bool) -> usize {
    let mut vals = row.1.clone();
    vals.sort();
    vals.dedup();
    if print { println!("{vals:?}"); }
    // find the contiguous groups as (start, end)
    let mut groups = vec![];
    let mut this_group_start = 0;
    for i in 0..(vals.len() - 1) {
        if vals[i + 1] - vals[i] <= 1 {
            // Contiguous
        } else {
            groups.push((this_group_start, i));
            this_group_start = i + 1;
        }
    }

    groups.push((this_group_start, vals.len() - 1));

    if print { println!("{groups:?}"); }

    let mut total = 0;
    for i in (0..groups.len()).step_by(2) {
        if i + 1 == groups.len() {continue;}
        let this_group = groups[i];
        let next_group = groups[i + 1];
        if print { println!("-> {} to {} & {} to {}", vals[this_group.0], vals[this_group.1], vals[next_group.0], vals[next_group.1]); }
        total += vals[next_group.0] - vals[this_group.1] - 1;
    }
    total as usize + vals.len() // TODO WRONG!
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
        //println!("\nScore: {}\n", evaluate_row((&y, &points.get(&y).unwrap())));
    }
}

fn _print_row(y: isize, points: &HashMap<isize, Vec<isize>>) {
    // gross!
    let min_x = *points.values().min_by_key(|xs| xs.iter().min().unwrap()).unwrap().iter().min().unwrap();

    let max_x = *points.values().max_by_key(|xs| xs.iter().max().unwrap()).unwrap().iter().max().unwrap();

    for x in min_x..=max_x {
        if points.get(&y).unwrap().iter().any(|&e| e == x) {
            print!("#");
        } else {
            print!(".");
        }
    }
    println!();
}


struct Instruction {
    direction: Direction,
    distance: usize,
    colour: String,
}

impl Instruction {
    fn from_str(input: &str) -> Instruction {
        let mut parts = input.split(' ');
        let direction = match parts.next().unwrap() {
            "R" => Direction::Right,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "U" => Direction::Up,
            _ => panic!("Unknown direction"),
        };

        let distance = parts.next().unwrap().parse().unwrap();

        let colour = parts.next().unwrap().to_string();

        Instruction { direction, distance, colour }
    }

    fn decoded(&self) -> Self {
        // The initial parse leaves the brackets
        let distance = usize::from_str_radix(&self.colour[2..7], 16).unwrap();
        let direction = match &self.colour[7..8] {
            "0" => Direction::Right,
            "1" => Direction::Down,
            "2" => Direction::Left,
            "3" => Direction::Up,
            _ => panic!("Unknown direction"),
        };

        Instruction { direction, distance, colour: self.colour.to_string() }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.direction, self.distance, self.colour)
    }
}
