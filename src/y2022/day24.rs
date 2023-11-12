use std::{fmt::Display, ops::Add, io, collections::{HashMap, HashSet}, time::Instant};
use std::hash::{Hash, Hasher};

use super::super::day::Day;

const HEIGHT: u8 = 35;
const WIDTH: u8 = 100;
const CYCLE: usize = HEIGHT as usize * WIDTH as usize;

//const HEIGHT: u8 = 4;
//const WIDTH: u8 = 6;

const TARGET: Coord = Coord { x: WIDTH, y: HEIGHT}; // Target the cell above, the rules treat the actual target as a wall

pub struct Day24
{
    blizzards: Vec<Blizzard>,
}

impl Day24 {
    pub fn new() -> Day24
    {
        let input = include_str!("input24");
        //let input = include_str!("input24_example");

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
    fn answer1(&self) -> String { String::from("?") }
    fn answer2(&self) -> String { String::from("?") }

    fn part1(&mut self) -> String {
        println!("{}x{}", WIDTH, HEIGHT);
        println!("target = {TARGET}");
        let initial = State { player: Coord::new(1, 0), timestamp: 0 };
        initial.find_path_2(self.blizzards.clone()).to_string()
    }

    fn part2(&mut self) -> String {
        "unsolved".to_string()
    }
}

#[derive(Clone)]
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

impl Eq for State {}

impl State {
    fn next_states(&self, blizzards: &mut BlizzardContainer) -> Vec<State> {
        let ts = self.timestamp + 1;
        let next_blizzards = blizzards.get(ts);
        let mut moves: Vec<_> = Direction::all().iter().filter_map(|dir| self.try_move(dir, &next_blizzards)).collect();
        if self.can_wait(&next_blizzards) {
            moves.push(State { player: self.player, timestamp: ts });
        }

        moves
    }

    fn try_move(&self, dir: &Direction, blizzards: &[Blizzard]) -> Option<State> {
        let proposed = self.player + dir.offset();
        if proposed.x == 0 || proposed.y == 0 || proposed.x == WIDTH + 1 || proposed.y == HEIGHT + 1 { return None; }
        if blizzards.iter().all(|blizz| blizz.loc != proposed) {
            let new_state = State { player: proposed, timestamp: self.timestamp + 1 };
            Some(new_state)
        } else {
            None
        }
    }


    fn find_path_2(&self, initial_state: Vec<Blizzard>) -> usize {
        return 0;
        let mut to_check: Vec<(State, usize)> = vec![(self.clone(), 0)];
        let mut best_to_coord = HashMap::<Coord, usize>::new();
        let mut best_answer = usize::MAX;
        let mut turns = 0;
        let mut last_iteration = Instant::now();
        let mut duplicate_skips = 0;
        let mut distance_skips = 0;
        let mut s = String::new();
        let mut blizzards = BlizzardContainer::new(initial_state);
        while let Some((checking, dist)) = to_check.pop() {
            turns += 1;
            let remaining_dist = ((TARGET.x - checking.player.x) + (TARGET.y - checking.player.y)) as usize;
            if dist + remaining_dist >= best_answer {
                distance_skips += 1;
            } else 
            if best_to_coord.contains_key(&checking.player) {
                duplicate_skips += 1;
            } else {
                //seen.insert(checking.clone());
                //println!("Checking \n{checking}");
                if checking.player == TARGET { 
                    // println!("{checking}");
                    // println!("Press to continue...");
                    // io::stdin().read_line(&mut s);
                    best_answer = usize::min(best_answer, dist + 1); // The target is the cell above the exit, we need to take 1 more step
                    //println!("Best answer: {best_answer}");
                } else {
                    //println!("From:");
                    //checking.print(&mut blizzards);
                    let next_states = checking.next_states(&mut blizzards);
                    // for s in next_states.clone() {
                    //     println!("Could go to....");
                    //     s.print(&mut blizzards);
                    // }
                    // io::stdin().read_line(&mut s);
                    let with_distance = next_states.into_iter().map(|state| (state, dist + 1));
                    to_check.extend(with_distance);
                    //to_check.dedup_by_key(|e| e.0.clone());
                    if turns % 1000 == 0 {
                        println!("{} states to check | {duplicate_skips} dups | {distance_skips} abandoned | {} best answer | {}ms per iteration", to_check.len(), best_answer, last_iteration.elapsed().as_millis());
                        last_iteration = Instant::now();
                    }
                }
            }
        }

        println!("Best answer should not be 152606: {}", best_answer != 152606);
        println!("Best answer should be smaller than 88205: {}", best_answer < 88205);
        return best_answer;
    }

    fn can_wait(&self, next_blizzards: &[Blizzard]) -> bool {
        next_blizzards.iter().all(|blizz| blizz.loc != self.player)
    }

    fn print(&self, blizzards: &mut BlizzardContainer) {
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
            println!("Generating blizzard for {timestamp}");
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
        })
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}