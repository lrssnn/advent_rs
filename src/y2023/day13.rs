use std::fmt::Display;

use super::super::day::Day;

pub struct Day13
{
    maps: Vec<Map>,
}

impl Day13 {
    pub fn new() -> Day13
    {
        //let input = "#.##..##.\n..#.##.#.\n##......#\n##......#\n..#.##.#.\n..##..##.\n#.#.##.#.\n\n#...##..#\n#....#..#\n..##..###\n#####.##.\n#####.##.\n..##..###\n#....#..#";
        let input = include_str!("../../input/y2023/13");

        let maps = input.split("\n\n").map(Map::from_str).collect();

        Day13 { maps }
    }
}

impl Day for Day13 {
    fn day_name(&self) -> String { String::from("13") }
    fn answer1(&self) -> String { String::from("34100") }
    fn answer2(&self) -> String { String::from("??") }

    fn part1(&mut self) -> String {
        // for map in &self.maps {
        //     //println!("{map}");
        //     println!("Vertical: {:?}, Horizontal: {:?}", map.vertical_reflections(), map.horizontal_reflections());
        //     println!();
        // }
        self.maps.iter().map(Map::score).sum::<usize>().to_string()
    }

    fn part2(&mut self) -> String {
        String::new()
    }
}

struct Map {
    lines: Vec<Vec<bool>>,
}

impl Map {
    fn from_str(input: &str) -> Map {
        let lines = input.lines().map(|l| l.chars().map(|c| c == '#').collect()).collect();
        Map { lines }
    }

    fn score(&self) -> usize {
        let horizontal = self.horizontal_reflections();
        let vertical = self.vertical_reflections();
        // adding len because the answer is in 1 based indexing
        (100 * (horizontal.iter().sum::<usize>() + horizontal.len())) + (vertical.iter().sum::<usize>() + vertical.len())
    }

    fn vertical_reflections(&self) -> Vec<usize> {
        let mut res = vec![];
        for col in 0..(self.lines[0].len() - 1) {
            if self.reflects_vert_after(col) {
                res.push(col);
            }
        }
        res
    }

    fn reflects_vert_after(&self, col: usize) -> bool {
        let reflectable_distance = (col + 1).min(self.lines[0].len() - col - 1);
        for line in &self.lines {
            for offset in 0..reflectable_distance {
                if line[col - offset] != line [col + 1 + offset] {
                    return false;
                }
            }
        }
        true
    }

    fn horizontal_reflections(&self) -> Vec<usize> {
        let mut res = vec![];
        for row in 0..(self.lines.len() - 1) {
            if self.reflects_horizontal_after(row) {
                res.push(row);
            }
        }
        res
    }

    fn reflects_horizontal_after(&self, row: usize) -> bool {
        let reflectable_distance = (row + 1).min(self.lines.len() - row - 1);
        for col in 0..self.lines[0].len() {
            for offset in 0..reflectable_distance {
                if self.lines[row - offset][col] != self.lines[row + 1 + offset][col] {
                    return false;
                }
            }
        }
        true
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.lines.iter()
            .map(|l| l.iter()
                .map(|&c| if c { "#" } else { "." } )
                .fold(String::new(), |acc, e| acc + e)
        ).reduce(|acc, e| acc + "\n" + &e).unwrap())
    }
}