use std::{fmt::Display, ops::Add};

use super::super::day::Day;

//const HEIGHT: usize = 34;
//const WIDTH: usize = 100;

const HEIGHT: usize = 4;
const WIDTH: usize = 6;

const TARGET: Coord = Coord { x: WIDTH, y: HEIGHT}; // Target the cell above, the rules treat the actual target as a wall

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
        for (y, line) in lines.enumerate().skip(1).take(HEIGHT) {
            for (x, c) in line.chars().enumerate().skip(1).take(WIDTH) {
                if c != '.' {
                   blizzards.push(Blizzard::new(x, y, c));
                }
            }
        }

        Day24 { blizzards, }
    }
}

impl Day for Day24 {
    fn day_name(&self) -> String { String::from("24") }
    fn answer1(&self) -> String { String::from("?") }
    fn answer2(&self) -> String { String::from("?") }

    fn part1(&mut self) -> String {
        return "".to_string();
        println!("{}x{}", WIDTH, HEIGHT);
        println!("target = {TARGET}");
        let initial = State{ player: Coord::new(1, 0), blizzards: self.blizzards.clone() };
        //initial.find_path().expect("Didn't Find Any Path!").to_string()
        initial.find_path_2().to_string()
    }

    fn part2(&mut self) -> String {
        "unsolved".to_string()
    }
}

#[derive(Clone, PartialEq)]
struct State {
    blizzards: Vec<Blizzard>,
    player: Coord,
}

impl State {
    fn next_states(&self) -> Vec<State> {
        //println!("From:\n{self}\n");
        let next_blizzards = self.next_blizzards();
        let productives: Vec<_> = Direction::all().iter().filter_map(|dir| self.try_move(dir, &next_blizzards)).collect();

        if !productives.is_empty() {
            productives
        } else if self.can_wait(&next_blizzards) {
            // if we can't move towards the goal, we might want to wait
            vec![ State { player: self.player, blizzards: self.next_blizzards() }]
        } else {
            // We have to retreat
            Direction::backwards().iter().filter_map(|dir| self.try_move(dir, &next_blizzards)).collect()
        }
    }

    fn try_move(&self, dir: &Direction, blizzards: &[Blizzard]) -> Option<State> {
        let proposed = self.player + dir.offset();
        if proposed.x == 0 || proposed.y == 0 || proposed.x == WIDTH + 1 || proposed.y == HEIGHT + 1 { return None; }
        if blizzards.iter().all(|blizz| blizz.loc != proposed) {
            let new_state = State { player: proposed, blizzards: blizzards.to_vec() };
            Some(new_state)
        } else {
            None
        }
    }

    fn next_blizzards(&self) -> Vec<Blizzard> {
        self.blizzards.iter().map(|blizzard| blizzard.next_loc()).collect()
    }

    fn find_path(&self) -> Option<usize> {
        //println!("Find Path: {}", self.player);
        if self.player == TARGET { return Some(1); } // The target is the cell above the exit, it will take us 1 more step to exit
        if let Some(dist) = self.next_states().iter().filter_map(|s| s.find_path()).min() {
            Some(dist + 1)
        } else {
            None
        }
    }

    fn find_path_2(&self) -> usize {
        let mut to_check: Vec<(State, usize)> = vec![(self.clone(), 0)];
        while let Some((checking, dist)) = to_check.pop() {
            if checking.player == TARGET { return dist + 1; } // The target is the cell above the exit, we need to take 1 more step
            let next_states = checking.next_states();
            let with_distance = next_states.into_iter().map(|state| (state, dist + 1));
            to_check.extend(with_distance);
            to_check.dedup_by_key(|e| e.0.clone());
            println!("{} states to check", to_check.len());
        }

        panic!("Didn't find a path!");
    }

    fn can_wait(&self, next_blizzards: &[Blizzard]) -> bool {
        next_blizzards.iter().all(|blizz| blizz.loc != self.player)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    Up, Down, Left, Right,
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
        }
    }

    fn productive() -> Vec<Direction> {
        vec![Direction::Down, Direction::Right]
    }

    fn backwards() -> Vec<Direction> {
        vec![Direction::Up, Direction::Left]
    }

    fn all() -> Vec<Direction> {
        vec![Direction::Down, Direction::Right, Direction::Up, Direction::Left]
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Blizzard {
    loc: Coord,
    btype: Direction,
}

#[derive(Clone, Copy, Debug, PartialEq)]
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

impl Add<(isize, isize)> for Coord {
    fn add(self, rhs: (isize, isize)) -> Self {
        let x = ((self.x as isize) + rhs.0).max(0) as usize;
        let y = ((self.y as isize) + rhs.1).max(0) as usize;
        Coord { x, y }
    }

    type Output = Self;
}

impl Blizzard {
    fn new(x: usize, y: usize, c: char) -> Blizzard {
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
        })
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Top wall
        for x in 0..=WIDTH {
            if self.player.x == x && self.player.y == 0 {
                write!(f, "E").unwrap();
            }
            else if x == 1 {
                write!(f, ".").unwrap();
            }
            else {
                write!(f, "#").unwrap();
            }
        }
        writeln!(f, "#").unwrap();

        // Main rows
        for y in 1..=HEIGHT {
            write!(f, "#");
            for x in 1..=WIDTH {
                if self.player.x == x && self.player.y == y {
                    write!(f, "E").unwrap();
                }
                else if let Some(b) = self.blizzards.iter().find(|blizzard| blizzard.loc == (x, y)) {
                    write!(f, "{b}");
                } else {
                    write!(f, ".");
                }
            }
            writeln!(f, "#");
        }

        // Bottom wall
        for x in 0..=WIDTH {
            if self.player.x == x && self.player.y == HEIGHT + 1 {
                write!(f, "E").unwrap();
            }
            else if x == WIDTH {
                write!(f, ".").unwrap();
            }
            else {
                write!(f, "#").unwrap();
            }
        }
        writeln!(f, "#")
    }
}