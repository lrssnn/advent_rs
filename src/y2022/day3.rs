use std::{fs, collections::HashSet, fmt::Display};
use super::super::day::Day;

pub struct Day3
{
    rucks: Vec<Rucksack>,
}

impl Day3 {
    #[allow(dead_code)]
    pub fn new() -> Day3
    {
        let input = fs::read_to_string("src/y2022/input3").expect("File Read Error");
        //let input = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";

        let lines = input.trim().split('\n');
        let rucks = lines.map(Rucksack::from_str).collect();
        Day3 { rucks }
    }
}

impl Day for Day3 {
    fn day_name(&self) -> String { String::from("03") }
    fn answer1(&self) -> String { String::from("8515") }
    fn answer2(&self) -> String { String::from("2434") }

    fn solve(&mut self) -> (String, String)
    {
        /* 
        for r in &self.rucks {
            println!("{} ({})", r.outlier(), score_char(r.outlier()));
        }*/
        let ans1: usize = self.rucks.iter().map(|r| score_char(r.outlier())).sum();

        let mut rucks = self.rucks.iter();
        let mut sum = 0;
        while let Ok(chunk) = rucks.next_chunk::<3>() {
            sum += score_char(Rucksack::find_common(chunk))
        } 
        let ans2: usize = sum;

        //println!("{}, {}", ans1, ans2);
        (ans1.to_string(), ans2.to_string())
    }
}

struct Rucksack {
    compartment1: HashSet<char>,
    compartment2: HashSet<char>,
}

impl Rucksack {
    fn from_str(input: &str) -> Rucksack {
        let divider = input.len() / 2;
        //println!("{} -> {} | {}", input, &input[0..divider], &input[divider+1..]);
        let compartment1 = HashSet::from_iter(input[0..divider].chars());
        let compartment2 = HashSet::from_iter(input[divider..].chars());
        Rucksack {compartment1, compartment2}
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
        // This map to dereference sucks...
        let a: HashSet<char> = rucks[0].compartment1.union(&rucks[0].compartment2).copied().collect();
        let b: HashSet<char> = rucks[1].compartment1.union(&rucks[1].compartment2).copied().collect();
        let c: HashSet<&char> = rucks[2].compartment1.union(&rucks[2].compartment2).collect();
        **a.
            intersection(&b)
            .collect::<HashSet<_>>()
            .intersection(&c)
            .next()
            .expect("No Common element...")
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