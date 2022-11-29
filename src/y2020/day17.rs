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
        let width = lines[0].len();
        cells_rows.insert(0, );
        let mut cells_rows = lines.iter().map(|line| {
            let mut cells = line.chars().map(|c| Cell::from_char(&c)).collect::<Vec<_>>();
            // Pad zeroes on the edges to simplify checking later
            cells.insert(0, Cell::from_char(&'.'));
            cells.push(Cell::from_char(&'.'));
            cells
        }).collect::<Vec<_>>();
        let cells = vec![cells_rows]; // always 2d input
        Cube { extents_z: 1, cells }
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
}

impl Display for Cube {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        for layer in 0..((2*self.extents_z)-1) {
            let z_label = layer as isize - self.extents_z as isize;
            writeln!(f, "z = {}", z_label);

            for row in &self.cells[layer] {
                for cell in row {
                    write!(f, "{}", cell);
                }
                writeln!(f);
            }
            writeln!(f);
        }
        Ok(())
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        write!(f, "{}", if self.state { '#' } else { '.' })
    }
}