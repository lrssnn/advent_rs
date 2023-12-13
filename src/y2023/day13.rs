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
    fn answer2(&self) -> String { String::from("33106") }

    fn part1(&mut self) -> String {
        self.maps.iter().map(|m| m.score(false)).sum::<usize>().to_string()
    }

    fn part2(&mut self) -> String {
        self.maps.iter().map(|m| m.score(true)).sum::<usize>().to_string()
    }
}

fn transpose(input: &[Vec<bool>]) -> Vec<Vec<bool>> {
    (0..input[0].len())
        .map(|i| input.iter().map(|inner| inner[i]).collect::<Vec<_>>())
        .collect()
}

struct Map {
    lines: Vec<Vec<bool>>,
}

impl Map {
    fn from_str(input: &str) -> Map {
        let lines = input.lines().map(|l| l.chars().map(|c| c == '#').collect()).collect();
        Map { lines }
    }

    fn score(&self, with_substitution: bool) -> usize {
        if let Some(horizontal) = Self::find_reflection(&self.lines, with_substitution) {
            return 100 * (horizontal + 1)
        }
        Self::find_reflection(&transpose(&self.lines), with_substitution).unwrap() + 1
    }

    fn find_reflection(lines: &[Vec<bool>], with_substitution: bool) -> Option<usize> {
        // This only looks for reflections around a row (i.e. the reflection line is horizontal)
        // And relies on the reflection being inbetween rows
        for row in 0..(lines.len() - 1) {
            let (found, used_substitution) = Self::reflects_after(lines, row);
            if found && (used_substitution == with_substitution) {
                return Some(row)
            }
        }
        None
    }

    fn reflects_after(lines: &[Vec<bool>], row: usize) -> (bool, bool) {
        let mut used_substitution = false;
        let reflectable_distance = (row + 1).min(lines.len() - row - 1);
        for col in 0..lines[0].len() {
            for offset in 0..reflectable_distance {
                if lines[row - offset][col] != lines[row + 1 + offset][col] {
                    if !used_substitution {
                        used_substitution = true;
                    } else {
                        return (false, true);
                    }
                }
            }
        }
        (true, used_substitution)
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