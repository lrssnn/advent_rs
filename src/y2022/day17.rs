use std::{fmt::Display, collections::HashMap};

use super::super::day::Day;

pub struct Day17
{
    jets: Vec<Jet>,
    chamber: Chamber,
}

impl Day17 {
    pub fn new() -> Day17
    {
        let input = include_str!("input17");
        //let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

        let jets = input.chars()
            .map(Jet::from_char).collect::<Vec<_>>();

        let chamber = Chamber::new(input.chars().collect());

        Day17 { jets, chamber }
    }
}

impl Day for Day17 {
    fn day_name(&self) -> String { String::from("17") }
    fn answer1(&self) -> String { String::from("3085") }
    //fn answer1(&self) -> String { String::from("3068") } // Example
    //fn answer2(&self) -> String { String::from("1514285714288") } // Example
    fn answer2(&self) -> String { String::from("?") }

    fn part1(&mut self) -> String {
        let mut part1 = self.chamber.clone();

        for _ in 0..2022 {
            part1.drop_one();
        }

        return part1.height().to_string();
        // let mut game = Game::new(self.jets.clone());

        // for _turn in 1..=2022 {
        //     game.run_rock();
        // }

        // game.height().to_string()
    }

    fn part2(&mut self) -> String {
let mut part2 = self.chamber.clone();

        // state will be: current piece number, current jet index, top 4 rows of chamber
        // if we get a repeat, then we found a cycle
        //      -- delta_height: height from previous cycle to this one
        //      -- delta_drops: how many drops were needed to get to delta_height
        //      -- offset_height: how high the tower was when the cycle began for the first time
        //      -- offset_drops: how many drops it took to get to offset_height

        let mut cycle_finder = HashMap::new();

        // map state to (height, drops)
        cycle_finder.insert((part2.piecenum, part2.jetnum, 0usize), (0usize, 0usize));

        let mut drops = 0;
        loop {
            part2.drop_one();
            drops += 1;
            let height = part2.height();
            if height < 4 {
                continue;
            }

            let shape = ((part2.rocks[height - 1] as usize) << 24)
                | ((part2.rocks[height - 2] as usize) << 16)
                | ((part2.rocks[height - 3] as usize) << 8)
                | (part2.rocks[height - 4] as usize);

            if let Some(entry) = cycle_finder.get(&(part2.piecenum, part2.jetnum, shape)) {
                // println!("piece = {}", part2.piecenum);
                // println!("jetnum = {}", part2.jetnum);
                // println!("shape = {}", shape);

                // println!("drops until start of loop = {}", entry.1);
                // println!("height of tower when the loop started = {}", entry.0);
                let delta_height = height - entry.0;
                let delta_drops = drops - entry.1;
                // println!(
                // "There is an increase of {delta_height} rows for every {delta_drops} drops"
                // );
                let remaining_drops = Chamber::OUCH - entry.1;
                // println!("There are still {remaining_drops} left to go");

                let needed_drops = remaining_drops / delta_drops;
                let leftover_drops = remaining_drops % delta_drops;
                let integral_height = entry.0 + delta_height * needed_drops;

                // println!(
                // "The height will reach {integral_height} but there are still {leftover_drops} drops left"
                // );

                for _ in 0..leftover_drops {
                    part2.drop_one();
                }
                let leftover_height = part2.height() - height;
                // println!("After {leftover_drops} more drops, we added {leftover_height} rows");

                return (integral_height + leftover_height).to_string();
            } else {
                cycle_finder.insert((part2.piecenum, part2.jetnum, shape), (height, drops));
            }
        }
    }
        /*
        let mut game = Game::new(self.jets.clone());

        let mut seen_states: HashMap<State, (usize, usize)> = HashMap::new();

        const TURNS_TARGET: usize = 1_000_000_000_000;

        let mut turn = 0;
        loop {
            game.run_rock();
            turn += 1;
            let height = game.height();
            if height < 4 {
                continue;
            }

            let state = game.get_state();
            if let Some((turn_last_seen, height_last_seen)) = seen_states.get(&state) {
                let cycle_turns = turn - turn_last_seen;
                let cycle_height = height - height_last_seen;

                println!("The turn is {turn}, last saw it at {turn_last_seen} ({cycle_turns} turns ago)");

                let remaining_turns = TURNS_TARGET - turn_last_seen;

                let needed_cycles = remaining_turns / cycle_turns;

                let remainder_turns = remaining_turns % cycle_turns;

                println!("{remainder_turns} left over");

                let skipped_growth = cycle_height * needed_cycles;

                println!("Grew {cycle_height} per cycle: {} total growth", skipped_growth);

                println!("Processing {remainder_turns} remaining turns");
                for _turn in 0..remainder_turns {
                    game.run_rock();
                }

                let leftover_height = game.height();// - height;

                let answer = height_last_seen + skipped_growth + leftover_height;

                println!("example correct?: {}", answer == 1514285714288);
                println!("larger than known too small (real)?: {}", answer > 1514534883741);
                println!("equals known too small (real)?: {}", answer == 1514534883741);
                println!("equals known bad (real)?: {}", answer == 1514534883742);
                return answer.to_string();
            } else {
                seen_states.insert(state, (turn, height));
            }
        }
    }
    */
}

impl Day17 {
}

// For detecting cycles
#[derive(Debug, PartialEq, Eq, Hash)]
struct State { 
    spawn_counter: usize,
    jet_counter: usize,
    rows_top: [u8; 4],
}

struct Game {
    rows: Vec<u8>,
    spawn_counter: usize,
    jets: Vec<Jet>,
    jet_counter: usize,
}

impl Game {
    const PIECES: [[u8; 4]; 5] = [
        [0b0000000, 0b0000000, 0b0000000, 0b0011110], // Horizontal
        [0b0000000, 0b0001000, 0b0011100, 0b0001000], // Plus
        [0b0000000, 0b0000100, 0b0000100, 0b0011100], // Corner
        [0b0010000, 0b0010000, 0b0010000, 0b0010000], // Vertical
        [0b0000000, 0b0000000, 0b0011000, 0b0011000], // Square
    ];

    fn new(jets: Vec<Jet>) -> Game {
        Game { rows: vec![0, 0, 0, 0, 0, 0 ,0], spawn_counter: 0, jets, jet_counter: 0 }
    }

    fn height(&self) -> usize {
        // Make sure we aren't counting empty rows
        let mut height = self.rows.len();
        while height > 0 && self.rows[height - 1] == 0 {
            height -= 1;
        }
        height
    }

    fn run_rock(&mut self) {
        let mut piece = Self::PIECES[self.spawn_counter];
        self.spawn_counter = (self.spawn_counter + 1) % 5;

        // Make room at the top for the new piece
        let mut last = self.rows.len() - 7; // Need to have 7 empty spaces
        while self.rows[last] != 0 {
            self.rows.push(0);
            last += 1;
        }

        let mut bottom = self.rows.len() - 4;

        // Move and drop until we can't anymore
        loop {
            let jet = self.jets[self.jet_counter];
            self.jet_counter = (self.jet_counter + 1) % self.jets.len();

            match jet {
                Jet::Left => {
                    if self.can_go_left(bottom, &piece) {
                        for p in piece.iter_mut() { *p  <<= 1; }
                    }
                },
                Jet::Right => {
                    if self.can_go_right(bottom, &piece) {
                        for p in piece.iter_mut() { *p  >>= 1; }
                    }
                }
            }

            if bottom > 0 && self.can_go_to(bottom - 1, &piece) {
                bottom -= 1;
            } else {
                break; // Time to commit the piece and move on
            }
        }

        let mut prow = 4;
        while prow > 0 {
            prow -= 1;
            self.rows[bottom] |= piece[prow];
            bottom += 1;
        }
    }

     fn can_go_left(&self, mut bottom: usize, piece: &[u8; 4]) -> bool {
        let mut prow = 4;
        while prow > 0 {
            prow -= 1;
            if (piece[prow] & 0x40) != 0 || (self.rows[bottom] & (piece[prow] << 1)) != 0 {
                return false;
            }
            bottom += 1;
        }
        true
    }

    fn can_go_right(&self, mut bottom: usize, piece: &[u8; 4]) -> bool {
        let mut prow = 4;
        while prow > 0 {
            prow -= 1;
            if (piece[prow] & 0x01) != 0 || (self.rows[bottom] & (piece[prow] >> 1)) != 0 {
                return false;
            }
            bottom += 1;
        }
        true
    }

    fn can_go_to(&self, mut bottom: usize, piece: &[u8; 4]) -> bool {
        let mut prow = 4;
        while prow > 0 {
            prow -= 1;
            if (self.rows[bottom] & piece[prow]) != 0 {
                return false;
            }
            bottom += 1;
        }
        true
    }
    fn spawn(&mut self) {
    }

    fn tick(&mut self) -> bool {
        false
    }

    fn apply_jet(&mut self) {
    }

    fn apply_jet_left(&mut self) {
    }

    fn apply_jet_right(&mut self) {
    }

    fn apply_gravity(&mut self) {
    }

    fn get_state(&self) -> State {
        let height = self.height();
        State {
            spawn_counter: self.spawn_counter,
            jet_counter: self.jet_counter,
            rows_top: [
                self.rows[height - 1],
                self.rows[height - 2],
                self.rows[height - 3],
                self.rows[height - 4],
            ],
        }
    }
}

#[derive(Clone, Copy)]
enum Jet {
    Left,
    Right
}

impl Jet {
    fn from_char(input: char) -> Jet {
        match input {
            '<' => Self::Left,
            '>' => Self::Right,
            _ => panic!("Invalid char"),
        }
    }
}

impl Display for Jet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Left => '<',
            Self::Right => '>',
        })
    }
}

/*
impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.rows {
            write!(f, "|").unwrap();
            for cell in row {
                write!(f, "{cell}").unwrap();
            }
            writeln!(f, "|").unwrap();
        }
        write!(f, "+-------+").unwrap();
        Ok(())
    }
}
*/
#[derive(Default, Clone)]
struct Chamber {
    jets: Vec<char>,
    rocks: Vec<u8>,
    piecenum: usize,
    jetnum: usize,
}

impl Chamber {
    const PIECES: [[u8; 4]; 5] = [
        [0b0000000, 0b0000000, 0b0000000, 0b0011110],
        [0b0000000, 0b0001000, 0b0011100, 0b0001000],
        [0b0000000, 0b0000100, 0b0000100, 0b0011100],
        [0b0010000, 0b0010000, 0b0010000, 0b0010000],
        [0b0000000, 0b0000000, 0b0011000, 0b0011000],
    ];

    const OUCH: usize = 1_000_000_000_000;

    fn new(jets: Vec<char>) -> Self {
        Self {
            jets,
            rocks: vec![0, 0, 0, 0, 0, 0, 0],
            piecenum: 0,
            jetnum: 0,
        }
    }

    fn drop_one(&mut self) {
        let mut piece = Self::PIECES[self.piecenum];
        self.piecenum = (self.piecenum + 1) % Self::PIECES.len();

        // make room at the top for the new piece
        let mut last = self.rocks.len() - 7;
        while self.rocks[last] != 0 {
            self.rocks.push(0);
            last += 1;
        }

        let mut bottom = self.rocks.len() - 4;

        loop {
            // start off by using the jet to move the piece left or right
            let jet = self.jets[self.jetnum];
            self.jetnum = (self.jetnum + 1) % self.jets.len();

            match jet {
                '<' => {
                    if self.can_go_left(bottom, &piece) {
                        for p in piece.iter_mut() {
                            *p <<= 1;
                        }
                    }
                }
                '>' => {
                    if self.can_go_right(bottom, &piece) {
                        for p in piece.iter_mut() {
                            *p >>= 1;
                        }
                    }
                }
                _ => panic!("bad input '{jet}'"),
            }

            // drop the piece by one if it can
            if bottom > 0 && self.can_go_to(bottom - 1, &piece) {
                bottom -= 1;
            } else {
                break;
            }
        }

        let mut prow = 4;
        while prow > 0 {
            prow -= 1;
            self.rocks[bottom] |= piece[prow];
            bottom += 1;
        }
    }

    fn can_go_left(&self, mut bottom: usize, piece: &[u8; 4]) -> bool {
        let mut prow = 4;
        while prow > 0 {
            prow -= 1;
            if (piece[prow] & 0x40) != 0 || (self.rocks[bottom] & (piece[prow] << 1)) != 0 {
                return false;
            }
            bottom += 1;
        }
        true
    }

    fn can_go_right(&self, mut bottom: usize, piece: &[u8; 4]) -> bool {
        let mut prow = 4;
        while prow > 0 {
            prow -= 1;
            if (piece[prow] & 0x01) != 0 || (self.rocks[bottom] & (piece[prow] >> 1)) != 0 {
                return false;
            }
            bottom += 1;
        }
        true
    }

    fn can_go_to(&self, mut bottom: usize, piece: &[u8; 4]) -> bool {
        let mut prow = 4;
        while prow > 0 {
            prow -= 1;
            if (self.rocks[bottom] & piece[prow]) != 0 {
                return false;
            }
            bottom += 1;
        }
        true
    }

    fn height(&self) -> usize {
        let mut top = self.rocks.len();
        while top > 0 && self.rocks[top - 1] == 0 {
            top -= 1;
        }
        top
    }

    fn _print_piece(piece: &[u8; 4]) {
        for p in piece {
            Self::_print_row(*p);
            println!();
        }
    }

    fn _print_row(row: u8) {
        let mut bit = 0x40;
        while bit > 0 {
            print!("{}", if (bit & row) != 0 { '#' } else { '.' });
            bit >>= 1;
        }
    }

    fn _draw(&self) {
        let mut top = self.rocks.len();
        while top > 0 {
            top -= 1;
            print!("|");
            Self::_print_row(self.rocks[top]);
            println!("|");
        }
        println!("+-------+");
    }
}