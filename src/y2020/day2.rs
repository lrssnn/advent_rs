use std::{fs, fmt::Display};
use super::super::day::Day;

pub struct Day2
{
    passwords: Vec<Password>,
}

impl Day2 {
    pub fn new() -> Day2
    {
        let passwords = fs::read_to_string("src/y2020/input2")
            .expect("File Read Error")
            .trim()
            .split('\n')
            .map(|line| Password::from_string(line.to_string()))
            .collect();
        Day2 {passwords}
    }
}

impl Day for Day2 {
    fn day_name(&self) -> String { String::from("02") }
    fn answer1(&self) -> String { String::from("398") }
    fn answer2(&self) -> String { String::from("562") }

    fn solve(&self) -> (String, String) {
        let part1 = self.passwords.iter().filter(|p| p.satisfies_criteria1()).count();
        let part2 = self.passwords.iter().filter(|p| p.satisfies_criteria2()).count();
        (part1.to_string(), part2.to_string())
    }
}

struct Password {
    target: char,
    min: usize,
    max: usize,
    value: String,
}

impl Password {
    fn from_string(input: String) -> Password {
        let dash_i = input.find('-').expect("Malformed: No Dash");
        let space_i = input.find(' ').expect("Malformed: No Space");
        let colon_i = input.find(':').expect("Malformed: No Colon");

        let min_s = &input[0..dash_i];
        let max_s = &input[dash_i + 1..space_i];
        let target = &input.chars().nth(space_i + 1);
        let value = &input[colon_i + 2..];

        Password {
            target: target.expect("Parse Error"),
            min: min_s.parse::<usize>().expect("Parse Error"),
            max: max_s.parse::<usize>().expect("Parse Error"),
            value: value.to_string(),
        }
    }

    fn satisfies_criteria1(&self) -> bool {
        let count = self.value.chars().filter(|c| c.eq(&self.target)).count();
        count >= self.min && count <= self.max
    }

    fn satisfies_criteria2(&self) -> bool {
        let at1 = self.value.chars().nth(self.min - 1).expect("Malformed: Too short").eq(&self.target);
        let at2 = self.value.chars().nth(self.max - 1).expect("Malformed: Too short").eq(&self.target);
        (at1 && !at2) || (!at1 && at2)
    }
}

impl Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{} '{}': {}", self.min, self.max, self.target, self.value)
    }
}