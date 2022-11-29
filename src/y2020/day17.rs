use std::{fs, fmt::Display, collections::HashMap};
use super::super::day::Day;

pub struct Day17 {
    cube: Cube,
}

impl Day17 {
    pub fn new() -> Day17 {
        //let input = fs::read_to_string("src/y2020/input17").expect("File Read Error");
    
        let input = ".#.\n..#\n###";

        let cube = Cube::from_str(input);

        Day17 { cube }
    }
}

impl Day for Day17 {
    fn day_name(&self) -> String { String::from("17") }
    fn answer1(&self) -> String { String::from("?") }
    fn answer2(&self) -> String { String::from("?") }

    fn solve(&mut self) -> (String, String) {
        println!("{}", self.cube);
        let part1 = 0;

        let part2 = 0;
        
        let x = vec![0];

        println!("{:?}", (part1.to_string(), part2.to_string()));
        (part1.to_string(), part2.to_string())
    }
}

struct Cube {
    cells: Vec<Vec<Vec<Cell>>>,
}

struct Cell {
    state: bool,
}

impl Cube {
    fn from_str(input: &str) -> Cube {
        // input will have multiple lines
        let mut lines: Vec<_> = input.split('\n').collect();
        // Pad empty rows on the top and bottom
        let width = lines[0].len() + 2;

        let mut rows = vec![Cell::off_row(width)];

        let mut cells_rows = lines.iter().map(|line| {
            let mut cells = line.chars().map(|c| Cell::from_char(&c)).collect::<Vec<_>>();
            // Pad zeroes on the edges to simplify checking later
            cells.insert(0, Cell::from_char(&'.'));
            cells.push(Cell::from_char(&'.'));
            cells
        }).collect::<Vec<_>>();

        rows.append(&mut cells_rows);

        rows.push(Cell::off_row(width));

        Cube { cells: vec![Cell::off_layer(width), rows, Cell::off_layer(width)] }
    }
    
    /*
    fn next_cube(&self) -> Cube {
        Cube {}
    }
    */
}

impl Cell {
    fn from_char(input: &char) -> Cell {
        Cell { state: input.eq(&'#') }
    }

    fn off() -> Cell { Cell { state: false } }

    fn off_row(len: usize) -> Vec<Cell> {
        let mut r = vec![];
        for _ in 0..len { r.push(Cell::off()) }
        r
    }

    fn off_layer(width: usize) -> Vec<Vec<Cell>> {
        let mut r = vec![];
        for _ in 0..width { r.push(Cell::off_row(width)) }
        r
    }
}

impl Display for Cube {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        let mut z_depth = 0 as isize - (self.cells.len() as isize / 2);
        for layer in &self.cells {
            writeln!(f, "z = {}", z_depth);

            for row in layer {
                for cell in row {
                    write!(f, "{}", cell);
                }
                writeln!(f);
            }
            writeln!(f);
            z_depth += 1;
        }
        Ok(())
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        write!(f, "{}", if self.state { '#' } else { '.' })
    }
}