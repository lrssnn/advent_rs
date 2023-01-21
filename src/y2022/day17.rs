use std::{fmt::Display, collections::HashMap};

use super::super::day::Day;

const GAME_SIZE_LIMIT: usize = 40; // This is tuned based on feels and vibes. The smaller the faster, too small and you will change the result

pub struct Day17
{
    jets: Vec<Jet>,
}

impl Day17 {
    pub fn new() -> Day17
    {
        let input = include_str!("input17");
        //let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

        let jets = input.chars()
            .map(Jet::from_char).collect::<Vec<_>>();

        Day17 { jets }
    }
}

impl Day for Day17 {
    fn day_name(&self) -> String { String::from("17") }
    fn answer1(&self) -> String { String::from("3085") }
    fn answer2(&self) -> String { String::from("?") }

    fn part1(&mut self) -> String {
        let mut game = Game::new(self.jets.clone());

        for _turn in 1..=2022 {
            game.run_rock();
        }

        game.height().to_string()
    }

    fn part2(&mut self) -> String {
        return "".to_string();
        let mut game = Game::new(self.jets.clone());

        let mut seen_states: HashMap<State, (usize, usize)> = HashMap::new();

        const TURNS_TARGET: usize = 1_000_000_000_000;

        for turn in 1..=TURNS_TARGET {

            game.run_rock();

            let state = game.get_state();
            if let Some((turns_processed_last_seen, size_last_seen)) = seen_states.get(&state) {
                let cycle_turns = turn - turns_processed_last_seen;
                let cycle_growth = game.height() - size_last_seen;

                println!("The turn is {turn}, last saw it at {turns_processed_last_seen} ({cycle_turns} turns ago)");

                let remaining_turns = TURNS_TARGET - turn;

                let needed_cycles = remaining_turns / cycle_turns;

                let remainder_turns = remaining_turns % cycle_turns;

                println!("{remainder_turns} left over");

                let skipped_growth = cycle_growth * needed_cycles;

                println!("Grew {cycle_growth} in those turns {}: total growth", skipped_growth);

                println!("Processing {remainder_turns} remaining turns");
                let height_before = game.height();
                for _turn in 0..remainder_turns {
                    game.run_rock();
                }

                let grew = game.height() - height_before;

                let answer = game.height() + skipped_growth;
                println!("Correct?: {}", answer == 1514285714288);
                return answer.to_string();
            } else {
                seen_states.insert(state, (turn, game.height()));
            }
        }

        println!("I doubt it!");
        let ans2 = game.height();
        ans2.to_string()
    }
}

impl Day17 {
}

// For detecting cycles
#[derive(Debug, PartialEq, Eq, Hash)]
struct State { 
    spawn_counter: u8,
    jet_counter: usize,
    rows_top: [[Cell; 7]; 5],
}

struct Game {
    rows: Vec<[Cell; 7]>,
    spawn_counter: u8,
    jets: Vec<Jet>,
    jet_counter: usize,
    dead_rows: usize,
}

impl Game {
    fn new(jets: Vec<Jet>) -> Game {
        Game { rows: Vec::new(), spawn_counter: 0, jets, jet_counter: 0, dead_rows: 0}
    }

    fn height(&self) -> usize {
        self.rows.len() + self.dead_rows
    }

    fn run_rock(&mut self) {
        self.spawn();
        //println!("After spawn: ");
        //println!("{self}");

        let mut done = false;
        while !done {
            done = self.tick();
            //println!("After Tick: ");
            //println!("{self}");
        }

        // Remove any completely empty lines
        self.rows.reverse();
        while let Some(first) = self.rows.pop() {
            if !first.iter().all(|&c| c == Cell::Empty) {
                self.rows.push(first);
                break;
            }
        }
        self.rows.reverse();
        //println!("After Truncate: ");
        //println!("{self}");

        // Chop rows off the bottom, we can assume that beyond a certain point, the rows are static
        if self.rows.len() > GAME_SIZE_LIMIT {
            let dead_rows = self.rows.len() - GAME_SIZE_LIMIT;
            self.rows.truncate(GAME_SIZE_LIMIT);
            self.dead_rows += dead_rows;
        }
    }

    fn spawn(&mut self) {
        let shape_rows = Cell::get_shape(self.spawn_counter);
        self.spawn_counter = (self.spawn_counter + 1) % 5;

        // let's assume that we truncate empty rows before spawning
        // So to spawn we just need to pop these rows on the top of the board
        // with the 3 empty rows
        self.rows.reverse();
        self.rows.push([Cell::Empty; 7]);
        self.rows.push([Cell::Empty; 7]);
        self.rows.push([Cell::Empty; 7]);
        for row in shape_rows.iter().rev() {
            self.rows.push(*row);
        }
        self.rows.reverse();
    }

    fn tick(&mut self) -> bool {
        self.apply_jet();

        if self.should_rest() {
            self.rest();
            true
        } else {
            self.apply_gravity();
            false
        }
    }

    fn apply_jet(&mut self) {
        let jet = &self.jets[self.jet_counter];
        self.jet_counter = (self.jet_counter + 1) % self.jets.len();

        match jet {
            Jet::Left => self.apply_jet_left(),
            Jet::Right => self.apply_jet_right(),
        }
        //println!("{self}");
    }

    fn apply_jet_left(&mut self) {
        // if any cells cannot move left, do not apply the jet
        for row in &self.rows {
            for i in 0..7 {
                let cell = row[i];
                if cell == Cell::Active && ( i == 0 || row[i - 1] == Cell::Still ) {
                    //println!("Not applying a jet left...");
                    return;
                }
            }
        }

        // Shift every active cell 1 to the left
        //println!("Applying a jet left...");
        for row in &mut self.rows {
            for i in 1..7 {
                if row[i] == Cell::Active {
                    row[i - 1] = Cell::Active;
                    row[i] = Cell::Empty;
                }
            }
        }
    }

    fn apply_jet_right(&mut self) {
        // if any cells cannot move right, do not apply the jet
        for row in &self.rows {
            for i in 0..7 {
                let cell = row[i];
                if cell == Cell::Active {
                    // Check for still, because the active rocks don't matter
                    if i == 6 || row[i + 1] == Cell::Still {
                        //println!("Not applying a jet right...");
                        return;
                    }
                }
            }
        }

        // Shift every active cell 1 to the right
        //println!("Applying jet right...");
        for row in &mut self.rows {
            for i in (0..6).rev() {
                if row[i] == Cell::Active {
                    row[i + 1] = Cell::Active;
                    row[i] = Cell::Empty;
                }
            }
        }
    }

    fn apply_gravity(&mut self) {
        for row_i in (0..(self.rows.len() - 1)).rev(){
            for col_i in 0..7 {
                let cell = self.rows[row_i][col_i];
                if cell == Cell::Active {
                    self.rows[row_i + 1][col_i] = Cell::Active;
                    self.rows[row_i][col_i] = Cell::Empty;
                }
            }
        }
    }

    fn should_rest(&self) -> bool {
        let rows = self.rows.len();
        for row_i in (0..rows).rev(){
            for col_i in 0..7 {
                let cell = self.rows[row_i][col_i];
                if cell == Cell::Active && (row_i == rows - 1 || self.rows[row_i + 1][col_i] == Cell::Still) {
                    //println!("Rock Should rest because of {row_i},{col_i}");
                    return true;
                }
            }
        }
        false
    }

    fn rest(&mut self) {
        for row_i in 0..self.rows.len(){
            for col_i in 0..7 {
                let cell = self.rows[row_i][col_i];
                if cell == Cell::Active {
                    self.rows[row_i][col_i] = Cell::Still;
                }
            }
        }
    }

    fn get_state(&self) -> State {
        State {
            spawn_counter: self.spawn_counter,
            jet_counter: self.jet_counter,
            rows_top: [
                *self.rows.get(0).unwrap_or(&[Cell::Empty; 7]), 
                *self.rows.get(1).unwrap_or(&[Cell::Empty; 7]), 
                *self.rows.get(2).unwrap_or(&[Cell::Empty; 7]), 
                *self.rows.get(3).unwrap_or(&[Cell::Empty; 7]), 
                *self.rows.get(4).unwrap_or(&[Cell::Empty; 7]), 
            ],
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum Cell {
    Empty,
    Active,
    Still
}

impl Cell {
    fn get_shape(shape_id: u8) -> Vec<[Cell; 7]> {
        match shape_id {
            0 => Cell::shape_horizontal(),
            1 => Cell::shape_plus(),
            2 => Cell::shape_corner(),
            3 => Cell::shape_vertical(),
            4 => Cell::shape_square(),
            _ => panic!("Invalid spawn counter"),
        }
    }

    fn shape_horizontal() -> Vec<[Cell; 7]> {
        // ..@@@@.
        vec![
            [Cell::Empty, Cell::Empty, Cell::Active, Cell::Active, Cell::Active, Cell::Active, Cell::Empty]
        ]
    }

    fn shape_plus() -> Vec<[Cell; 7]> {
        // ...@...
        // ..@@@..
        // ...@...
        vec![
            [Cell::Empty, Cell::Empty, Cell::Empty,  Cell::Active, Cell::Empty,  Cell::Empty, Cell::Empty],
            [Cell::Empty, Cell::Empty, Cell::Active, Cell::Active, Cell::Active, Cell::Empty, Cell::Empty],
            [Cell::Empty, Cell::Empty, Cell::Empty,  Cell::Active, Cell::Empty,  Cell::Empty, Cell::Empty],
        ]
    }

    fn shape_corner() -> Vec<[Cell; 7]> {
        // ....@..
        // ....@..
        // ..@@@..
        vec![
            [Cell::Empty, Cell::Empty, Cell::Empty,  Cell::Empty,  Cell::Active, Cell::Empty, Cell::Empty],
            [Cell::Empty, Cell::Empty, Cell::Empty,  Cell::Empty,  Cell::Active, Cell::Empty, Cell::Empty],
            [Cell::Empty, Cell::Empty, Cell::Active, Cell::Active, Cell::Active, Cell::Empty, Cell::Empty],
        ]
    }

    fn shape_vertical() -> Vec<[Cell; 7]> {
        // ..@....
        // ..@....
        // ..@....
        // ..@....
        vec![
            [Cell::Empty, Cell::Empty, Cell::Active, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            [Cell::Empty, Cell::Empty, Cell::Active, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            [Cell::Empty, Cell::Empty, Cell::Active, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            [Cell::Empty, Cell::Empty, Cell::Active, Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
        ]
    }

    fn shape_square() -> Vec<[Cell; 7]> {
        // ..@@...
        // ..@@...
        vec![
            [Cell::Empty, Cell::Empty, Cell::Active, Cell::Active, Cell::Empty, Cell::Empty, Cell::Empty],
            [Cell::Empty, Cell::Empty, Cell::Active, Cell::Active, Cell::Empty, Cell::Empty, Cell::Empty]
        ]
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

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Cell::Active => '@',
            Cell::Still => '#',
            Cell::Empty => '.',
        })
    }
}