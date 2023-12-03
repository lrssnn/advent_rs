use std::ops::Range;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use super::super::day::Day;

pub struct Day03
{
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
}

impl Day03 {
    pub fn new() -> Day03
    {
        //let input = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";
        let input = include_str!("../../input/y2023/03");

        let numbers = Number::all_from_str(input);
        let symbols = Symbol::all_from_str(input);

        Day03 { numbers, symbols }
    }
}

impl Day for Day03 {
    fn day_name(&self) -> String { String::from("03") }
    fn answer1(&self) -> String { String::from("525119") }
    fn answer2(&self) -> String { String::from("76504829") }

    fn part1(&mut self) -> String {
        self.numbers.par_iter().filter(|n| has_adjacent_symbol(n, &self.symbols)).map(|n| n.value).sum::<usize>().to_string()
    }

    fn part2(&mut self) -> String {
        self.symbols.par_iter().map(|s| get_gear_ratio(&self.numbers, s)).sum::<usize>().to_string()
    }
}

struct Number {
    value: usize,
    row: usize,
    cols: Range<usize>,
}

struct Symbol {
    value: char,
    row: usize,
    col: usize,
}

impl Number {
    fn new(value_s: String, row: usize, cols: Range<usize>) -> Self {
        Self { value: value_s.parse::<usize>().unwrap() , row, cols }
    }

    fn all_from_str(input: &str) -> Vec<Number> {
        let mut numbers = vec![];
        for (i, line) in input.lines().enumerate() {
            let mut inside_number = false;
            let mut number_start = 0;
            let mut number_s = String::new();
            for (j, c) in line.chars().enumerate() {
                match (inside_number, c.is_ascii_digit()) {
                    (false, true) => {
                        // Start of number
                        number_start = j;
                        inside_number = true;
                        number_s += &c.to_string();
                    },
                    (true, true) => {
                        number_s += &c.to_string();
                    }
                    (true, false) => {
                        // End of number
                        numbers.push(Number::new(number_s, i, number_start..j));
                        inside_number = false;
                        number_start = 0;
                        number_s = String::new();
                    },
                    _ => (), // Outside a number
                }
            }
            // Make sure to capture numbers that end at the end of the line
            if inside_number { numbers.push(Number::new(number_s, i, number_start..line.len())); }
        }

        numbers
    }
}

impl Symbol {
    fn new(value: char, row: usize, col: usize) -> Self {
        Self { value , row, col }
    }

    fn all_from_str(input: &str) -> Vec<Symbol> {
        let mut symbols = vec![];
        for (i, line) in input.lines().enumerate() {
            for (j, c) in line.chars().enumerate() {
                if !c.is_ascii_digit() && c != '.' {
                    symbols.push(Symbol::new(c, i, j));
                }
            }
        }

        symbols
    }
}

fn has_adjacent_symbol(number: &Number, symbols: &[Symbol]) -> bool {
    symbols.iter().any(|s| are_adjacent(number, s))
}

fn get_gear_ratio(numbers: &[Number], symbol: &Symbol) -> usize {
    if symbol.value != '*' {
        return 0;
    }

    let adjacents = numbers.iter().filter(|n| are_adjacent(n, symbol)).collect::<Vec<_>>();
    
    if adjacents.len() != 2 {
        return 0;
    }

    adjacents[0].value * adjacents[1].value
}

fn are_adjacent(number: &Number, symbol: &Symbol) -> bool {
    let row_distance = number.row as isize - symbol.row as isize;
    match row_distance {
        -1..=1 => {
            (number.cols.start.saturating_sub(1)..(number.cols.end + 1)).contains(&symbol.col)
        }, 
        _ => false // Too far to be adjacent
    }
}