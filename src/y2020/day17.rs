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
        
        println!("Wish me luck");

        println!("{}", self.cube.next_cube());
        let part1 = 0;

        let part2 = 0;
        
        let x = vec![0];

        println!("{:?}", (part1.to_string(), part2.to_string()));
        (part1.to_string(), part2.to_string())
    }
}

struct Cube {
    cells: Vec<Vec<Vec<bool>>>,
}

impl Cube {
    fn from_str(input: &str) -> Cube {
        // input will have multiple lines
        let mut lines: Vec<_> = input.split('\n').collect();
        // Pad 2 empty rows on the top and bottom
        let width = lines[0].len() + 4;

        let mut rows = vec![Cube::off_row(width), Cube::off_row(width)];

        let mut cells_rows = lines.iter().map(|line| {
            let mut cells = line.chars().map(|c| Cube::cell_from_char(&c)).collect::<Vec<_>>();
            // Pad zeroes on the edges to simplify checking later
            cells.insert(0, Cube::cell_from_char(&'.'));
            cells.insert(0, Cube::cell_from_char(&'.'));
            cells.push(Cube::cell_from_char(&'.'));
            cells.push(Cube::cell_from_char(&'.'));
            cells
        }).collect::<Vec<_>>();

        rows.append(&mut cells_rows);

        rows.push(Cube::off_row(width));
        rows.push(Cube::off_row(width));

        Cube { cells: vec![Cube::off_layer(width, width), Cube::off_layer(width, width), rows, Cube::off_layer(width, width), Cube::off_layer(width, width)] }
    }
    
    fn next_cube(&self) -> Cube {
        // parallelisation bait
        let z_size = self.cells.len();
        let y_size = self.cells[0].len();
        let x_size = self.cells[0][0].len();
        let mut result = Cube {cells: vec![] };
        // empty layers on either end
        result.cells.push(Cube::off_layer(x_size + 2, y_size + 2));
        for z in 1..z_size-1 {
            let mut layer = vec![];
            // empty row at top and bottom of each layer
            layer.push(Cube::off_row(x_size + 2));
            for y in 1..y_size-1 {
                let mut row = vec![];
                // empty cell on either end of row
                row.push(false);
                for x in 1..x_size-1 {
                   row.push(Cube::from_area(x, y, z, &self.cells)) 
                }
                row.push(false);
                layer.push(row);
            }
            layer.push(Cube::off_row(x_size + 2));
            result.cells.push(layer);
        }
        result.cells.push(Cube::off_layer(x_size + 2, y_size + 2));
        result
    }

    fn cell_from_char(input: &char) -> bool {
        input.eq(&'#')
    }
    
    fn cell_to_char(input: bool) -> char { if input {'#'} else {'.'}}

    fn off_row(len: usize) -> Vec<bool> {
        let mut r = vec![];
        for _ in 0..len { r.push(false) }
        r
    }

    fn off_layer(width: usize, height: usize) -> Vec<Vec<bool>> {
        let mut r = vec![];
        for _ in 0..height { r.push(Cube::off_row(width)) }
        r
    }
    
    fn from_area(x_c :usize, y_c: usize, z_c: usize, area: &Vec<Vec<Vec<bool>>>) -> bool {
        let me = area[z_c][y_c][x_c];
        let mut neighbours = 0;
        for x in x_c-1..=x_c+1 {
            for y in y_c-1..=y_c+1 {
                for z in z_c-1..=z_c+1 {
                    if x == x_c && y == y_c && z == z_c { continue; }
                    if area[z][y][x] { neighbours += 1; }
                }
            }
        }

        if me {
            neighbours == 2 || neighbours == 3
        } else {
            neighbours == 3
        }
    }
}

impl Display for Cube {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        let mut z_depth = 0 as isize - (self.cells.len() as isize / 2);
        for layer in &self.cells {
            writeln!(f, "z = {}", z_depth);

            for row in layer {
                for cell in row {
                    write!(f, "{}", Cube::cell_to_char(*cell));
                }
                writeln!(f);
            }
            writeln!(f);
            z_depth += 1;
        }
        Ok(())
    }
}