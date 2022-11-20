use std::{fs, fmt::Display};
use super::super::day::Day;
use rayon::prelude::*;

//const ROWS: usize = 10;
const ROWS: usize = 99;

//const COLS: usize = 10;
const COLS: usize = 95;

pub struct Day11 {
    state: State,
}

impl Day11 {
    pub fn new() -> Day11 {
        let input = fs::read_to_string("src/y2020/input11").expect("File Read Error");

        //let input = "L.LL.LL.LL\nLLLLLLL.LL\nL.L.L..L..\nLLLL.LL.LL\nL.LL.LL.LL\nL.LLLLL.LL\n..L.L.....\nLLLLLLLLLL\nL.LLLLLL.L\nL.LLLLL.LL";

        let state = State::from_string(&input);
        
        Day11 { state }
    }

    fn simulate_to_steady_state(&mut self) {
        loop {
            let (state, changed) = self.state.next_state();
            self.state = state;
            //println!("{}", self.state);
            if !changed { break; }
        }
    }

    fn simulate_to_steady_state_los(&mut self) {
        loop {
            let (state, changed) = self.state.next_state_los();
            self.state = state;
            //println!("{}", self.state);
            if !changed { break; }
        }
    }
}

impl Day for Day11 {
    fn day_name(&self) -> String { String::from("11") }
    fn answer1(&self) -> String { String::from("2453") }
    fn answer2(&self) -> String { String::from("2159") }

    fn solve(&mut self) -> (String, String) {
        let start_state = self.state.clone();
        self.simulate_to_steady_state();

        let part1 = self.state.count_occupied();

        // reset for part 2
        self.state = start_state;
        self.simulate_to_steady_state_los();
        let part2 = self.state.count_occupied();
        
        //println!("{:?}", (part1.to_string(), part2.to_string()));
        (part1.to_string(), part2.to_string())
    }
}

#[derive(Clone)]
struct State {
    seats: Vec<Vec<Seat>>,
}

impl State {
    fn from_string(input: &str) -> State {
        // input has multiple lines
        let lines = input.trim().split('\n').map(|s| s.trim());
        let seats: Vec<Vec<Seat>> = lines.map(|line| {
            line.chars().map(|c| match c {
                '.' => Seat::Floor,
                'L' => Seat::Empty,
                '#' => Seat::Taken,
                _   => panic!("Invalid Character!"),
            })
            .collect()
        })
        .collect();
        State { seats }
    }

    fn count_occupied(&self) -> usize {
        let mut occ = 0;
        for row in &self.seats {
            for seat in row {
                if seat.eq(&Seat::Taken) { occ += 1; }
            }
        }
        occ
    }

    fn next_state(&self) -> (State, bool) {
        let seats: Vec<Vec<Seat>> = self.seats
        .par_iter()
        .enumerate().map(|(r, row)| {
            row
            .iter()
            .enumerate()
            .map(|(c, seat)| seat.next_value(r, c, &self.seats))
            .collect()
        })
        .collect();

        let something_changed = self.something_changed(&seats);

        (State { seats }, something_changed)
    }

    fn next_state_los(&self) -> (State, bool) {
        let seats: Vec<Vec<Seat>> = self.seats
        .par_iter()
        .enumerate().map(|(r, row)| {
            row
            .iter()
            .enumerate()
            .map(|(c, seat)| seat.next_value_los(r, c, &self.seats))
            .collect()
        })
        .collect();

        let something_changed = self.something_changed(&seats);

        (State { seats }, something_changed)
    }

    fn something_changed(&self, other: &[Vec<Seat>]) -> bool {
        for row_pair in self.seats.iter().zip(other.iter()) {
            for seat_pair in row_pair.0.iter().zip(row_pair.1.iter()) {
                if seat_pair.0 != seat_pair.1 { return true; }
            }
        }
        false
    }
}

#[derive(Copy, Clone, PartialEq)]
enum Seat {
    Floor,
    Taken,
    Empty
}

impl Seat {
    fn next_value(&self, row: usize, col: usize, seats: &[Vec<Seat>]) -> Seat {
        match self {
            Self::Floor => Self::Floor,
            Self::Empty => self.next_from_empty(row, col, seats),
            Self::Taken => self.next_from_taken(row, col, seats),
        }
    }

    fn next_value_los(&self, row: usize, col: usize, seats: &[Vec<Seat>]) -> Seat {
        match self {
            Self::Floor => Self::Floor,
            Self::Empty => self.next_from_empty_los(row, col, seats),
            Self::Taken => self.next_from_taken_los(row, col, seats),
        }
    }

    fn next_from_empty(&self, row: usize, col: usize, seats: &[Vec<Seat>]) -> Seat {
        let neighbours = self.count_neighbours(row, col, seats);
        if neighbours == 0 { Self::Taken } else { Self::Empty }
    }

    fn next_from_empty_los(&self, row: usize, col: usize, seats: &[Vec<Seat>]) -> Seat {
        let neighbours = self.count_neighbours_los(row, col, seats);
        if neighbours == 0 { Self::Taken } else { Self::Empty }
    }

    fn next_from_taken(&self, row: usize, col: usize, seats: &[Vec<Seat>]) -> Seat {
        let neighbours = self.count_neighbours(row, col, seats);
        if neighbours >= 4 { Self::Empty } else { Self::Taken }
    }

    fn next_from_taken_los(&self, row: usize, col: usize, seats: &[Vec<Seat>]) -> Seat {
        let neighbours = self.count_neighbours_los(row, col, seats);
        if neighbours >= 5 { Self::Empty } else { Self::Taken }
    }

    fn count_neighbours(&self, row: usize, col: usize, seats: &[Vec<Seat>]) -> usize {
        self.up_left_of(row, col, seats)
        + self.up_of(row, col, seats)
        + self.up_right_of(row, col, seats)
        + self.left_of(row, col, seats)
        + self.right_of(row, col, seats)
        + self.down_left_of(row, col, seats)
        + self.down_of(row, col, seats)
        + self.down_right_of(row, col, seats)
    }

    fn count_neighbours_los(&self, row: usize, col: usize, seats: &[Vec<Seat>]) -> usize {
        self.up_left_of_los(row, col, seats)
        + self.up_of_los(row, col, seats)
        + self.up_right_of_los(row, col, seats)
        + self.left_of_los(row, col, seats)
        + self.right_of_los(row, col, seats)
        + self.down_left_of_los(row, col, seats)
        + self.down_of_los(row, col, seats)
        + self.down_right_of_los(row, col, seats)
    }

    // This is pretty dumb, but it feels like it might optimise well?
    fn up_left_of(&self, row: usize, col: usize, seats: &[Vec<Seat>]) -> usize {
        if row == 0 { return 0 }
        if col == 0 { return 0 }
        if seats[row - 1][col - 1] == Seat::Taken { return 1 }
        0
    }

    fn up_of(&self, row: usize, col: usize, seats: &[Vec<Seat>]) -> usize {
        if row == 0 { return 0 }
        if seats[row - 1][col] == Seat::Taken { return 1 }
        0
    }

    fn up_right_of(&self, row: usize, col: usize, seats: &[Vec<Seat>]) -> usize {
        if row == 0 { return 0 }
        if col == COLS - 1 { return 0 }
        if seats[row - 1][col + 1] == Seat::Taken { return 1 }
        0
    }

    fn left_of(&self, row: usize, col: usize, seats: &[Vec<Seat>]) -> usize {
        if col == 0 { return 0 }
        if seats[row][col - 1] == Seat::Taken { return 1 }
        0
    }

    fn right_of(&self, row: usize, col: usize, seats: &[Vec<Seat>]) -> usize {
        if col == COLS - 1 { return 0 }
        if seats[row][col + 1] == Seat::Taken { return 1 }
        0
    }

    fn down_left_of(&self, row: usize, col: usize, seats: &[Vec<Seat>]) -> usize {
        if row == ROWS - 1 { return 0 }
        if col == 0 { return 0 }
        if seats[row + 1][col - 1] == Seat::Taken { return 1 }
        0
    }

    fn down_of(&self, row: usize, col: usize, seats: &[Vec<Seat>]) -> usize {
        if row == ROWS - 1 { return 0 }
        if seats[row + 1][col] == Seat::Taken { return 1 }
        0
    }

    fn down_right_of(&self, row: usize, col: usize, seats: &[Vec<Seat>]) -> usize {
        if row == ROWS - 1 { return 0 }
        if col == COLS - 1 { return 0 }
        if seats[row + 1][col + 1] == Seat::Taken { return 1 }
        0
    }

    // los = line of sight
    fn up_left_of_los(&self, row: usize, col: usize, seats: &[Vec<Seat>]) -> usize {
        let mut r = row;
        let mut c = col;
        loop {
            if r == 0 { return 0 }
            if c == 0 { return 0 }
            r -= 1;
            c -= 1;
            if seats[r][c] == Seat::Taken { return 1 }
            if seats[r][c] == Seat::Empty { return 0 }
        }
    }

    fn up_of_los(&self, row: usize, col: usize, seats: &[Vec<Seat>]) -> usize {
        let mut r = row;
        loop {
            if r == 0 { return 0 }
            r -= 1;
            if seats[r][col] == Seat::Taken { return 1 }
            if seats[r][col] == Seat::Empty { return 0 }
        }
    }

    fn up_right_of_los(&self, row: usize, col: usize, seats: &[Vec<Seat>]) -> usize {
        let mut r = row;
        let mut c = col;
        loop {
            if r == 0 { return 0 }
            if c == COLS - 1 { return 0 }
            r -= 1;
            c += 1;
            if seats[r][c] == Seat::Taken { return 1 }
            if seats[r][c] == Seat::Empty { return 0 }
        }
    }

    fn left_of_los(&self, row: usize, col: usize, seats: &[Vec<Seat>]) -> usize {
        let mut c = col;
        loop {
            if c == 0 { return 0 }
            c -= 1;
            if seats[row][c] == Seat::Taken { return 1 }
            if seats[row][c] == Seat::Empty { return 0 }
        }
    }

    fn right_of_los(&self, row: usize, col: usize, seats: &[Vec<Seat>]) -> usize {
        let mut c = col;
        loop {
            if c == COLS - 1 { return 0 }
            c += 1;
            if seats[row][c] == Seat::Taken { return 1 }
            if seats[row][c] == Seat::Empty { return 0 }
        }
    }

    fn down_left_of_los(&self, row: usize, col: usize, seats: &[Vec<Seat>]) -> usize {
        let mut r = row;
        let mut c = col;
        loop {
            if r == ROWS - 1 { return 0 }
            if c == 0 { return 0 }
            r += 1;
            c -= 1;
            if seats[r][c] == Seat::Taken { return 1 }
            if seats[r][c] == Seat::Empty { return 0 }
        }
    }

    fn down_of_los(&self, row: usize, col: usize, seats: &[Vec<Seat>]) -> usize {
        let mut r = row;
        loop {
            if r == ROWS - 1 { return 0 }
            r += 1;
            if seats[r][col] == Seat::Taken { return 1 }
            if seats[r][col] == Seat::Empty { return 0 }
        }
    }

    fn down_right_of_los(&self, row: usize, col: usize, seats: &[Vec<Seat>]) -> usize {
        let mut r = row;
        let mut c = col;
        loop {
            if r == ROWS - 1 { return 0 }
            if c == COLS - 1 { return 0 }
            r += 1;
            c += 1;
            if seats[r][c] == Seat::Taken { return 1 }
            if seats[r][c] == Seat::Empty { return 0 }
        }
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.seats {
            for seat in row {
                write!(f, "{}", seat).expect("!!!");
            }
            writeln!(f).expect("!!!");
        }
        Ok(())
    }
}

impl Display for Seat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Floor => '.',
            Self::Taken => '#',
            Self::Empty => 'L',
        })
    }
}