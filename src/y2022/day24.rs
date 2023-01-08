use std::fmt::Display;

use super::super::day::Day;

pub struct Day24
{
    width: usize,
    height: usize,
    blizzards: Vec<Blizzard>,
}

impl Day24 {
    pub fn new() -> Day24
    {
        let input = include_str!("input24");
        let input = include_str!("input24_example");

        let mut lines = input.lines();
        let height = lines.clone().count() - 2;

        let first = lines.next().unwrap();
        let width = first.len() - 2; // Ignoring the walls

        let mut blizzards = Vec::new();

        for (y, line) in lines.take(height).enumerate() {
            for (x, c) in line.chars().skip(1).take(width).enumerate() {
                if c != '.' {
                   blizzards.push(Blizzard::new(x, y, c));
                }
            }
        }

        Day24 { width, height, blizzards, }
    }

    #[allow(unused)]
    fn print_board(&self) {
        // Top wall
        print!("#.");
        for _ in 0..self.width { print!("#"); }
        println!();

        // Main rows
        for y in 0..self.height {
            print!("#");
            for x in 0..self.width {
                if let Some(b) = self.blizzards.iter().find(|blizzard| blizzard.loc == (x, y)) {
                    print!("{b}");
                } else {
                    print!(".");
                }
            }
            println!("#");
        }

        // Bottom wall
        for _ in 0..self.width { print!("#"); }
        print!(".#");
        println!();
    }
}

impl Day for Day24 {
    fn day_name(&self) -> String { String::from("24") }
    fn answer1(&self) -> String { String::from("?") }
    fn answer2(&self) -> String { String::from("?") }

    fn part1(&mut self) -> String {
        println!("{}x{}", self.width, self.height);
        self.print_board();
        "unsolved".to_string()
    }

    fn part2(&mut self) -> String {
        "unsolved".to_string()
    }
}

#[derive(Clone, Copy, Debug)]
enum BlizzardType {
    Up, Down, Left, Right,
}

impl BlizzardType {
    fn from_char(c: char) -> BlizzardType {
        match c  {
            '^' => BlizzardType::Up,
            'v' => BlizzardType::Down,
            '<' => BlizzardType::Left,
            '>' => BlizzardType::Right,
            _ => panic!("Unknown char type")
        }
    }

    fn offset(&self) -> (isize, isize) {
        match self {
            BlizzardType::Up => (0, -1),
            BlizzardType::Down => (0 , 1),
            BlizzardType::Left => (-1, 0),
            BlizzardType::Right => (1, 0),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Blizzard {
    loc: Coord,
    btype: BlizzardType,
}

#[derive(Clone, Copy, Debug)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn new(x: usize, y: usize) -> Coord {
        Coord { x, y }
    }
}

impl PartialEq<(usize, usize)> for Coord {
    fn eq(&self, other: &(usize, usize)) -> bool {
        self.x == other.0 && self.y == other.1
    }
}

impl Blizzard {
    fn new(x: usize, y: usize, c: char) -> Blizzard {
        Blizzard { loc: Coord::new(x, y), btype: BlizzardType::from_char(c) }
    }
}

impl Display for Blizzard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.btype)
    }
}

impl Display for BlizzardType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            BlizzardType::Up => '^',
            BlizzardType::Down => 'v',
            BlizzardType::Left => '<',
            BlizzardType::Right => '>',
        })
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}