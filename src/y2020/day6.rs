use std::collections::HashSet;
use std::{fs, fmt::Display};
use super::super::day::Day;

pub struct Day6
{
    answers: Vec<AnswerGroup>,
}

impl Day6 {
    pub fn new() -> Day6
    {
        let input = fs::read_to_string("src/y2020/input6")
            .expect("File Read Error");
        
        let answers: Vec<AnswerGroup> = input.trim().split("\n\n")
            .map(AnswerGroup::from_string)
            .collect();

        Day6 { answers }
    }
}

impl Day for Day6 {
    fn day_name(&self) -> String { String::from("06") }
    fn answer1(&self) -> String { String::from("6885") }
    fn answer2(&self) -> String { String::from("3550") }

    fn solve(&mut self) -> (String, String) {
        let mut part1 = 0;
        let mut part2 = 0;
        
        for group in &self.answers {
            let (any, all) = group.yesses();
            part1 += any;
            part2 += all;
        }

        (part1.to_string(), part2.to_string())
    }
}

struct AnswerGroup {
    answers: Vec<Vec<char>>,
}

impl AnswerGroup {
    fn from_string(input: &str) -> AnswerGroup {
        // input may be multiple lines
        let mut answers: Vec<Vec<char>> = vec!();
        
        for line in input.lines() {
            let items = line.chars().collect();
            answers.push(items);
        }
        
        AnswerGroup { answers }
    }
    
    fn yesses(&self) -> (usize, usize) {
        let mut yesses = HashSet::new();
        for person in &self.answers {
            for c in person {
                yesses.insert(c);
            }
        }
        let any = yesses.len();
        
        let mut unanimous = HashSet::new();
        for item in &yesses {
            if self.answers.iter().all(|p| p.contains(item)) {
                unanimous.insert(item);
            }
        }
        
        let all = unanimous.len();
        (any, all)
    }
}

impl Display for AnswerGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // This feels slow, but Display is only for debugging
        write!(
            f, 
            "{:?}",
            self.answers
        )
    }
}