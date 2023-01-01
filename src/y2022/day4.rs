use std::{fmt::Display, ops::RangeInclusive};
use super::super::day::Day;

pub struct Day4
{
    assignments: Vec<Assignment>,
}

impl Day4 {
    pub fn new() -> Day4
    {
        let input = include_str!("input4");
        //let input = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8";

        let lines = input.trim().split('\n');
        let assignments = lines.map(Assignment::from_str).collect();
        Day4 { assignments }
    }
}

impl Day for Day4 {
    fn day_name(&self) -> String { String::from("04") }
    fn answer1(&self) -> String { String::from("456") }
    fn answer2(&self) -> String { String::from("808") }

    fn part1(&mut self) -> String {
        //for a in &self.assignments { println!("{}", a); }
        self.assignments.iter().filter(|c| c.has_redundant()).count().to_string()
    }

    fn part2(&mut self) -> String {
        self.assignments.iter().filter(|c| c.has_overlap()).count().to_string()
    }
}

struct Assignment {
    a: RangeInclusive<usize>,
    b: RangeInclusive<usize>,
}

impl Assignment {
    fn from_str(input: &str) -> Assignment {
        let mut parts = input.split(',');
        let a = range_from_str(parts.next().expect("Malformed Input"));
        let b = range_from_str(parts.next().expect("Malformed Input"));
        Assignment {
            a, b
        }
    }

    fn has_redundant(&self) -> bool {
        (self.a.contains(self.b.start()) && self.a.contains(self.b.end()))
        ||
        (self.b.contains(self.a.start()) && self.b.contains(self.a.end()))
    }

    fn has_overlap(&self) -> bool {
        (self.a.contains(self.b.start()) || self.a.contains(self.b.end()))
        ||
        (self.b.contains(self.a.start()) || self.b.contains(self.a.end()))
    }
}

fn range_from_str(input: &str) -> RangeInclusive<usize> {
    // input should look like 'x-y'
    let mut parts = input.split('-');
    let min_s = parts.next().expect("Malformed Input");
    let max_s = parts.next().expect("Malformed Input");
    min_s.parse().expect("Bad integer")..=max_s.parse().expect("Bad Integer")
}

impl Display for Assignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}, {}-{} ({})", self.a.start(), self.a.end(), self.b.start(), self.b.end(), self.has_redundant())
    }
}