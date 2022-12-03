use std::{fs, fmt::Display, collections::HashMap};
use super::super::day::Day;

pub struct Day17 {
    cube: Cube,
}

impl Day17 {
    pub fn new() -> Day17 {
        let input = fs::read_to_string("src/y2020/input17").expect("File Read Error");
    
        //let input = ".#.\n..#\n###";

        let cube = Cube::from_str(&input.trim());

        Day17 { cube }
    }
}

impl Day for Day17 {
    fn day_name(&self) -> String { String::from("17") }
    fn answer1(&self) -> String { String::from("375") }
    fn answer2(&self) -> String { String::from("?") }

    fn solve(&mut self) -> (String, String) {
        println!("{}", self.cube);
        
        println!("Wish me luck");
        for step in 1..=6 {
            self.cube = self.cube.next_cube();    
            println!("After step {}, {}", step, self.cube.count_active())
        }
        let part1 = self.cube.count_active();

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
        let lines: Vec<_> = input.split('\n').collect();
        // Pad 2 empty rows on the top and bottom
        let width = lines[0].len();


        let rows = lines.iter().map(|line| {
            line.chars().map(|c| Cube::cell_from_char(&c)).collect::<Vec<_>>()
        }).collect::<Vec<_>>();

        Cube { cells: vec![rows] }
    }
    
    fn next_cube(&self) -> Cube {
        // parallelisation bait
        // + 2 because we are growing by 1 in both directions (in all dimensions)
        let z_size = self.cells.len() + 2;
        let y_size = self.cells[0].len() + 2;
        let x_size = self.cells[0][0].len() + 2;
        let mut result = Cube {cells: vec![] };

        for z in 0..z_size {
            let mut layer = vec![];
            for y in 0..y_size {
                let mut row = vec![];
                for x in 0..x_size {
                   row.push(Cube::from_area(x, y, z, &self.cells)) 
                }
                layer.push(row);
            }
            result.cells.push(layer);
        }
        result
    }

    fn count_active(&self) -> usize {
        self.cells.iter().map(|layer| 
            layer.iter().map(|row| 
                row.iter().map(|cell| if *cell {1} else {0}).sum::<usize>()
            ).sum::<usize>()
        ).sum()
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
        let z_size = area.len();
        let y_size = area[0].len();
        let x_size = area[0][0].len();

        //println!("\n\n====\n({},{},{}) limits ({}, {}, {})", x_c, y_c, z_c, x_size-1, y_size-1, z_size-1);

        // The [x][y][z] coord is in relation to the NEW coord space, which is shifted by 1 in all dimensions compared to
        // the coord space of the 'area' input. Be careful
        let me = if x_c == 0 || y_c == 0 || z_c == 0 {
            //println!("({},{},{})(new space) is an edge (low), 'me' is false", x_c, y_c, z_c);
            false
        } else if x_c > x_size || y_c > y_size || z_c > z_size {
            //println!("({},{},{})(new space) is an edge (high), 'me' is false", x_c, y_c, z_c);
            false
        } else {
            //println!("({},{},{})(new space) is based on the neighbours of ({},{},{})(old space)", x_c, y_c, z_c, x_c - 1, y_c - 1, z_c - 1);
            area[z_c-1][y_c-1][x_c-1]
        };

        let x_shifted = x_c as isize - 1;
        let y_shifted = y_c as isize - 1;
        let z_shifted = z_c as isize - 1;
        ////println!("old space centre point is ({},{},{})", x_shifted, y_shifted, z_shifted);
        //println!("  ({},{},{}) from ({}, {}, {})", x_c, y_c, z_c, x_shifted, y_shifted, z_shifted);

        let mut neighbours = 0;
        for x_offset in -1..=1 {
            if x_shifted + x_offset < 0 || x_shifted + x_offset >= x_size as isize {
                ////println!("  escaping because of x: {}", x_shifted + x_offset);
                continue;
            } else {
                //println!("  x_offset = {} and x = {}", x_offset, x_shifted + x_offset);
            }
            for y_offset in -1..=1 {
                if y_shifted + y_offset < 0 || y_shifted + y_offset >= y_size as isize {
                    //println!("    escaping because of y: {}", y_shifted + y_offset);
                    continue;
                }
                for z_offset in -1..=1 {
                    if z_shifted + z_offset < 0 || z_shifted + z_offset >= z_size as isize {
                        //println!("      escaping because of z: {}", z_offset + z_shifted);
                        continue;
                    }
                    if x_offset == 0 && y_offset == 0 && z_offset == 0 { continue; }
                    let x = (x_shifted + x_offset) as usize;
                    let y = (y_shifted + y_offset) as usize;
                    let z = (z_shifted + z_offset) as usize;
                    //println!("Considering ({},{},{}) old space, which is {}", x, y, z, area[z][y][x]);
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