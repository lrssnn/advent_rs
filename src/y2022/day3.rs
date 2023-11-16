use std::{collections::HashSet, fmt::Display};
use super::super::day::Day;

mod tests {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn example_pt1() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";
        let result = Day3::new_with_input(input).part1();
        assert_eq!("157", result); 
    }

    #[test]
    fn real_pt1() {
        let result = Day3::new().part1();
        assert_eq!("8515", result); 
    }

    #[test]
    fn example_pt2() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";
        let result = Day3::new_with_input(input).part2();
        assert_eq!("70", result); 
    }

    #[test]
    fn real_pt2() {
        let result = Day3::new().part2();
        assert_eq!("2434", result); 
    }
}
pub struct Day3
{
    rucks: Vec<Rucksack>,
}

impl Day3 {
    #[allow(dead_code)]
    pub fn new() -> Day3 {
        Day3::new_with_input(include_str!("input3"))
    }

    pub fn new_with_input(input: &str) -> Day3
    {
        let lines = input.trim().split('\n');
        let rucks = lines.map(Rucksack::from_str).collect();
        Day3 { rucks }
    }
}

impl Day for Day3 {
    fn day_name(&self) -> String { String::from("03") }
    fn answer1(&self) -> String { String::from("8515") }
    fn answer2(&self) -> String { String::from("2434") }

    fn part1(&mut self) -> String {
        self.rucks.iter().map(|r| score_char(r.outlier())).sum::<usize>().to_string()
    }

    fn part2(&mut self) -> String {
        (0..(self.rucks.len() / 3))
            .map(|i| score_char(
                Rucksack::find_common(
                    [&self.rucks[i*3], &self.rucks[(i*3) + 1], &self.rucks[(i*3) + 2]]
                )
            )
        )
        .sum::<usize>()
        .to_string()
    }
}

struct Rucksack {
    compartment1: HashSet<char>,
    compartment2: HashSet<char>,
    // equivalent to compartment1 union compartment2
    items: HashSet<char>,
}

impl Rucksack {
    fn from_str(input: &str) -> Rucksack {
        let divider = input.len() / 2;
        //println!("{} -> {} | {}", input, &input[0..divider], &input[divider+1..]);
        let compartment1 = HashSet::from_iter(input[0..divider].chars());
        let compartment2 = HashSet::from_iter(input[divider..].chars());
        let items = compartment1.union(&compartment2).copied().collect();
        Rucksack {compartment1, compartment2, items}
    }

    fn outlier(&self) -> char {
        /*
        for c in &self.compartment1 {
            print!("{}", c);
        }
        println!("");
        for c in &self.compartment2 {
            print!("{}", c);
        }
        println!("");
        println!("");
        */
        *self.compartment1.intersection(&self.compartment2).next().expect("No intersection!")
    }

    fn find_common(rucks: [&Rucksack; 3]) -> char {
        /*
        println!("");
        println!("{}", rucks[0]);
        println!("{}", rucks[1]);
        println!("{}", rucks[2]);
        */
        for item in &rucks[0].items {
            if rucks[1].items.contains(item) && rucks[2].items.contains(item) {
                return *item;
            }
        }
        panic!("No shared item!");
    }
}

fn score_char(input: char) -> usize {
    match input {
        'a'..='z' => (input as usize) - 96,
        'A'..='Z' => (input as usize) - 38,
        _ => panic!("Unexpected char")
    }
}

impl Display for Rucksack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {:?}", self.compartment1, self.compartment2)
    }
}