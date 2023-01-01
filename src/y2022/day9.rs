use std::{fmt::Display, collections::HashSet};

use super::super::day::Day;

pub struct Day9
{
    instructions: Vec<Instruction>,
}

impl Day9 {
    pub fn new() -> Day9
    {
        let input = include_str!("input9");
        //let input = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2";

        let instructions = input.trim().split('\n')
            .map(Instruction::from_str).collect();

        Day9 { instructions }
    }
}

impl Day for Day9 {
    fn day_name(&self) -> String { String::from("09") }
    fn answer1(&self) -> String { String::from("5874") }
    fn answer2(&self) -> String { String::from("2467") }

    fn part1(&mut self) -> String {
        self.process().to_string()
    }

    fn part2(&mut self) -> String {
        self.process_long().to_string()
    }
}

impl Day9 {
    fn process(&self) -> usize {
        let mut touched_coords: HashSet<Coord> = HashSet::new();

        let mut head = Coord {x: 0, y: 0};
        let mut tail = Coord {x: 0, y: 0};
        touched_coords.insert(tail);

        for instruction in &self.instructions {
            //println!("== {instruction} ==");
            for _ in 0..instruction.repeats {
                //println!("Head: {}, Tail: {}", head, tail);
                head = head.step(&instruction.direction);
                tail = tail.seek(head);
                touched_coords.insert(tail);
            }
        }

        //for coord in &touched_coords { println!("{coord}");}
        touched_coords.len()
    }

    fn process_long(&self) -> usize {
        // Ugly, should make this generic to rope length...
        let mut touched_coords: HashSet<Coord> = HashSet::new();

        let mut head = Coord {x: 0, y: 0};
        let mut one = Coord {x: 0, y: 0};
        let mut two = Coord {x: 0, y: 0};
        let mut three = Coord {x: 0, y: 0};
        let mut four = Coord {x: 0, y: 0};
        let mut five = Coord {x: 0, y: 0};
        let mut six = Coord {x: 0, y: 0};
        let mut seven = Coord {x: 0, y: 0};
        let mut eight = Coord {x: 0, y: 0};
        let mut tail = Coord {x: 0, y: 0};
        touched_coords.insert(tail);

        for instruction in &self.instructions {
            //println!("== {instruction} ==");
            for _ in 0..instruction.repeats {
                //println!("Head: {}, Tail: {}", head, tail);
                head = head.step(&instruction.direction);
                one = one.seek(head);
                two = two.seek(one);
                three = three.seek(two);
                four = four.seek(three);
                five = five.seek(four);
                six = six.seek(five);
                seven = seven.seek(six);
                eight = eight.seek(seven);
                tail = tail.seek(eight);
                touched_coords.insert(tail);
            }
        }

        //for coord in &touched_coords { println!("{coord}");}
        touched_coords.len()
    }
}

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
struct Coord {
    x: isize,
    y: isize
}

impl Coord {
    fn step(&self, d: &Direction) -> Coord {
        match d {
            Direction::Right => Coord { x: self.x + 1, y: self.y },
            Direction::Left => Coord { x: self.x - 1, y: self.y },
            Direction::Up => Coord { x: self.x, y: self.y + 1 },
            Direction::Down => Coord { x: self.x, y: self.y - 1 },
        }
    }

    fn seek(&self, other: Coord) -> Coord {
        // feels like we can simplify the logic here
        let x_offset = other.x - self.x;
        let y_offset = other.y - self.y;

        let x_abs = x_offset.abs();
        let y_abs = y_offset.abs();

        let x_direction = x_offset.signum();
        let y_direction = y_offset.signum();

        // If the head is ever two steps directly up, down, left, or right from the tail
        // the tail must move one step in that direction
        if x_offset == 0 && y_abs > 1 {
            return Coord { x: self.x, y: self.y + y_direction }
        }
        if y_offset == 0 && x_abs > 1 {
            return Coord { x: self.x + x_direction, y: self.y }
        }

        // Otherwise, if the head and tail aren't touching, and aren't in the same row or column
        // the tail always moves one step diagonally to keep up

        // if the head and tail are touching, don't move
        if x_offset == 0 && y_offset == 0 { return *self }

        if x_abs > 1 || y_abs > 1 {
            return Coord { x: self.x + x_direction, y: self.y + y_direction }
        }
        *self
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

struct Instruction {
    direction: Direction,
    repeats: usize,
}

impl Instruction {
    fn from_str(input: &str) -> Instruction {
        let mut parts = input.split(' ');
        let direction = Direction::from_str(parts.next().expect("!"));
        let repeats = parts.next().expect("!").parse::<usize>().expect("?");
        Instruction { direction, repeats }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.direction, self.repeats)
    }
}

enum Direction {
    Right, Left, Up, Down
}

impl Direction {
    fn from_str(input: &str) -> Direction {
        match input {
            "R" => Self::Right,
            "L" => Self::Left,
            "U" => Self::Up,
            "D" => Self::Down,
            _ => panic!("Invalid input"),
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Down => "D",
            Self::Up => "U",
            Self::Left => "L",
            Self::Right => "R",
        })
    }
}