use std::{fs, fmt::Display};
use super::super::day::Day;

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

    fn simulate_to_steady_state(&mut self, los_mode: bool) {
        loop {
            let (state, changed) = self.state.next_state(los_mode);
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
        self.simulate_to_steady_state(false);

        let part1 = self.state.count_occupied();

        // reset for part 2
        self.state = start_state;
        self.simulate_to_steady_state(true);
        let part2 = self.state.count_occupied();
        
        //println!("{:?}", (part1.to_string(), part2.to_string()));
        (part1.to_string(), part2.to_string())
    }
}

#[derive(Clone, Copy)]
struct State {
    seats: [[Seat; COLS]; ROWS],
}

impl State {
    fn from_string(input: &str) -> State {
        let mut seats = [[Seat::Floor; COLS]; ROWS];
        // input has multiple lines
        let mut lines = input.trim().split('\n').map(|s| s.trim());
        for row in 0..ROWS {
            let mut chars = lines.next().expect("Dimension Mismatch").chars();
            for col in 0..COLS {
                let c = chars.next().expect("Dimension Mismatch");
                seats[row][col] = match c {
                    '.' => Seat::Floor,
                    'L' => Seat::Empty,
                    '#' => Seat::Taken,
                    _   => panic!("Invalid Character!"),
                }
            }
        }
        State { seats }
    }

    fn count_occupied(&self) -> usize {
        let mut occ = 0;
        for row in self.seats {
            for seat in row {
                if seat == Seat::Taken { occ += 1; }
            }
        }
        occ
    }

    fn next_state(&self, los_mode: bool) -> (State, bool) {
        let mut something_changed = false;

        let mut seats = [[Seat::Floor; COLS]; ROWS];
        for row in 0..ROWS {
            for col in 0..COLS {
                seats[row][col] = self.seats[row][col].next_value(row, col, &self.seats, los_mode);
                if !something_changed && seats[row][col] != self.seats[row][col] {
                    something_changed = true;
                }
            }
        }
        (State { seats }, something_changed)
    }
}

#[derive(Copy, Clone, PartialEq)]
enum Seat {
    Floor,
    Taken,
    Empty
}

impl Seat {
    fn next_value(&self, row: usize, col: usize, seats: &[[Seat; COLS]; ROWS], los_mode: bool) -> Seat {
        match self {
            Self::Floor => Self::Floor,
            Self::Empty => self.next_from_empty(row, col, seats, los_mode),
            Self::Taken => self.next_from_taken(row, col, seats, los_mode),
        }
    }

    fn next_from_empty(&self, row: usize, col: usize, seats: &[[Seat; COLS]; ROWS], los_mode: bool) -> Seat {
        let neighbours = self.count_neighbours(row, col, seats, los_mode);
        if neighbours == 0 { Self::Taken } else { Self::Empty }
    }

    fn next_from_taken(&self, row: usize, col: usize, seats: &[[Seat; COLS]; ROWS], los_mode: bool) -> Seat {
        let neighbours = self.count_neighbours(row, col, seats, los_mode);
        if los_mode {
            if neighbours >= 5 { Self::Empty } else { Self::Taken }
        } else {
            if neighbours >= 4 { Self::Empty } else { Self::Taken }
        }
    }

    fn count_neighbours(&self, row: usize, col: usize, seats: &[[Seat; COLS]; ROWS], los_mode: bool) -> usize {
        if los_mode {
            self.up_left_of_los(row, col, seats)
            + self.up_of_los(row, col, seats)
            + self.up_right_of_los(row, col, seats)
            + self.left_of_los(row, col, seats)
            + self.right_of_los(row, col, seats)
            + self.down_left_of_los(row, col, seats)
            + self.down_of_los(row, col, seats)
            + self.down_right_of_los(row, col, seats)
        } else {
            self.up_left_of(row, col, seats)
            + self.up_of(row, col, seats)
            + self.up_right_of(row, col, seats)
            + self.left_of(row, col, seats)
            + self.right_of(row, col, seats)
            + self.down_left_of(row, col, seats)
            + self.down_of(row, col, seats)
            + self.down_right_of(row, col, seats)
        }
    }

    // This is pretty dumb, but it feels like it might optimise well?
    fn up_left_of(&self, row: usize, col: usize, seats: &[[Seat; COLS]; ROWS]) -> usize {
        if row == 0 { return 0 }
        if col == 0 { return 0 }
        if seats[row - 1][col - 1] == Seat::Taken { return 1 }
        0
    }

    fn up_of(&self, row: usize, col: usize, seats: &[[Seat; COLS]; ROWS]) -> usize {
        if row == 0 { return 0 }
        if seats[row - 1][col] == Seat::Taken { return 1 }
        0
    }

    fn up_right_of(&self, row: usize, col: usize, seats: &[[Seat; COLS]; ROWS]) -> usize {
        if row == 0 { return 0 }
        if col == COLS - 1 { return 0 }
        if seats[row - 1][col + 1] == Seat::Taken { return 1 }
        0
    }

    fn left_of(&self, row: usize, col: usize, seats: &[[Seat; COLS]; ROWS]) -> usize {
        if col == 0 { return 0 }
        if seats[row][col - 1] == Seat::Taken { return 1 }
        0
    }

    fn right_of(&self, row: usize, col: usize, seats: &[[Seat; COLS]; ROWS]) -> usize {
        if col == COLS - 1 { return 0 }
        if seats[row][col + 1] == Seat::Taken { return 1 }
        0
    }

    fn down_left_of(&self, row: usize, col: usize, seats: &[[Seat; COLS]; ROWS]) -> usize {
        if row == ROWS - 1 { return 0 }
        if col == 0 { return 0 }
        if seats[row + 1][col - 1] == Seat::Taken { return 1 }
        0
    }

    fn down_of(&self, row: usize, col: usize, seats: &[[Seat; COLS]; ROWS]) -> usize {
        if row == ROWS - 1 { return 0 }
        if seats[row + 1][col] == Seat::Taken { return 1 }
        0
    }

    fn down_right_of(&self, row: usize, col: usize, seats: &[[Seat; COLS]; ROWS]) -> usize {
        if row == ROWS - 1 { return 0 }
        if col == COLS - 1 { return 0 }
        if seats[row + 1][col + 1] == Seat::Taken { return 1 }
        0
    }

    // los = line of sight
    fn up_left_of_los(&self, row: usize, col: usize, seats: &[[Seat; COLS]; ROWS]) -> usize {
        let mut r = row;
        let mut c = col;
        loop {
            if r == 0 { return 0 }
            if c == 0 { return 0 }
            r = r - 1;
            c = c - 1;
            if seats[r][c] == Seat::Taken { return 1 }
            if seats[r][c] == Seat::Empty { return 0 }
        }
    }

    fn up_of_los(&self, row: usize, col: usize, seats: &[[Seat; COLS]; ROWS]) -> usize {
        let mut r = row;
        loop {
            if r == 0 { return 0 }
            r = r - 1;
            if seats[r][col] == Seat::Taken { return 1 }
            if seats[r][col] == Seat::Empty { return 0 }
        }
    }

    fn up_right_of_los(&self, row: usize, col: usize, seats: &[[Seat; COLS]; ROWS]) -> usize {
        let mut r = row;
        let mut c = col;
        loop {
            if r == 0 { return 0 }
            if c == COLS - 1 { return 0 }
            r = r - 1;
            c = c + 1;
            if seats[r][c] == Seat::Taken { return 1 }
            if seats[r][c] == Seat::Empty { return 0 }
        }
    }

    fn left_of_los(&self, row: usize, col: usize, seats: &[[Seat; COLS]; ROWS]) -> usize {
        let mut c = col;
        loop {
            if c == 0 { return 0 }
            c = c - 1;
            if seats[row][c] == Seat::Taken { return 1 }
            if seats[row][c] == Seat::Empty { return 0 }
        }
    }

    fn right_of_los(&self, row: usize, col: usize, seats: &[[Seat; COLS]; ROWS]) -> usize {
        let mut c = col;
        loop {
            if c == COLS - 1 { return 0 }
            c = c + 1;
            if seats[row][c] == Seat::Taken { return 1 }
            if seats[row][c] == Seat::Empty { return 0 }
        }
    }

    fn down_left_of_los(&self, row: usize, col: usize, seats: &[[Seat; COLS]; ROWS]) -> usize {
        let mut r = row;
        let mut c = col;
        loop {
            if r == ROWS - 1 { return 0 }
            if c == 0 { return 0 }
            r = r + 1;
            c = c - 1;
            if seats[r][c] == Seat::Taken { return 1 }
            if seats[r][c] == Seat::Empty { return 0 }
        }
    }

    fn down_of_los(&self, row: usize, col: usize, seats: &[[Seat; COLS]; ROWS]) -> usize {
        let mut r = row;
        loop {
            if r == ROWS - 1 { return 0 }
            r = r + 1;
            if seats[r][col] == Seat::Taken { return 1 }
            if seats[r][col] == Seat::Empty { return 0 }
        }
    }

    fn down_right_of_los(&self, row: usize, col: usize, seats: &[[Seat; COLS]; ROWS]) -> usize {
        let mut r = row;
        let mut c = col;
        loop {
            if r == ROWS - 1 { return 0 }
            if c == COLS - 1 { return 0 }
            r = r + 1;
            c = c + 1;
            if seats[r][c] == Seat::Taken { return 1 }
            if seats[r][c] == Seat::Empty { return 0 }
        }
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.seats {
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