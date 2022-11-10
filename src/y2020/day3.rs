use std::{fs, fmt::Display};
use super::super::day::Day;

const WIDTH: usize = 31;
const HEIGHT: usize = 323;

pub struct Day3
{
    map: Map,
}

impl Day3 {
    pub fn new() -> Day3
    {
        let input = fs::read_to_string("src/y2020/input3")
            .expect("File Read Error");

        let map = Map::from_string(input);
        Day3 { map }
    }

    pub fn get_part2_answer(&self) -> usize {
        self.map.count_trees(1, 1)
        * self.map.count_trees(3, 1)
        * self.map.count_trees(5, 1)
        * self.map.count_trees(7, 1)
        * self.map.count_trees(1, 2)
    }
}

impl Day for Day3 {
    fn day_name(&self) -> String { String::from("03") }
    fn answer1(&self) -> String { String::from("211") }
    fn answer2(&self) -> String { String::from("3584591857") }

    fn solve(&mut self) -> (String, String) {
        let part1 = self.map.count_trees(3, 1);
        let part2 = self.get_part2_answer();
        (part1.to_string(), part2.to_string())
    }
}

struct Map {
    values: [[bool; WIDTH]; HEIGHT],
}

impl Map {
    fn from_string(input: String) -> Map {
        let mut values = [[false; WIDTH]; HEIGHT];
        let lines: Vec<&str> = input.trim()
            .split('\n')
            .collect();

        for i in 0..HEIGHT {
            let line = lines[i];
            for j in 0..WIDTH {
                let char = line.chars().nth(j).expect("Line not long enough");
                if char == '#' {
                    values[i][j] = true;
                }
            }
        }

        Map {values}
    }

    fn count_trees(&self, dx: usize, dy: usize) -> usize {
        let mut row = 0;
        let mut col = 0;
        let mut trees = 0;
        while row < HEIGHT {
            if self.has_tree(row, col) { trees += 1; }
            row += dy;
            col += dx;
        }
        trees
    }

    fn has_tree(&self, row: usize, col: usize) -> bool {
        // Apply wrapping/tiling
        let col_wrap = col % WIDTH;
        // index into map
        self.values[row][col_wrap]
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // This feels slow, but Display is only for debugging
        write!(
            f, 
            "{}", 
            &self.values.map(|line| 
                line.map(|b| if b {'#'} else {'.'})
                .iter()
                .collect::<String>()
            ).join("\n"))
    }
}