use std::collections::HashMap;
use super::super::day::Day;

pub struct Day15 {
    startup: Vec<usize>,
}

impl Day15 {
    pub fn new() -> Day15 {
        let input = "0,14,1,3,7,9";
    
        //let input = "0,3,6";

        let numbers = input.trim().split(',').map(|s| s.parse().expect("Parse Error")).collect();

        Day15 { startup: numbers }
    }

    fn run(&self) -> (usize, usize) {
        let mut memory = HashMap::new();
        let mut turn = 1;
        let mut prev = 0;

        // Startup
        for s in &self.startup {
            memory.insert(*s, (turn, 0));
            turn += 1;
            prev = *s;
        }

        let mut part1 = 9999999;

        // Play
        for turn in turn..30000001 {
            let memory_entry = memory.get(&prev).expect("Prev should always be in mem");
            let last_spoken_turn = memory_entry.0;
            let previous_spoken_turn = memory_entry.1;
            if previous_spoken_turn == 0 {
                //println!("Turn {}: {} is new", turn, prev);
                prev = 0;
            } else {
                //println!("Turn {}: {} was spoken on turn {}, before that, {}", turn, prev, last_spoken_turn, previous_spoken_turn);
                prev = last_spoken_turn - previous_spoken_turn;
            }
            //println!("  speaking {}\n", prev);
            Self::update_memory(&mut memory, prev, turn);
            if turn == 2020 { part1 = prev; }
        }

        (part1, prev)
    }

    fn update_memory(memory: &mut HashMap<usize, (usize, usize)>, speaking: usize, turn: usize) {
        if let Some(memory_entry) = memory.get(&speaking) {
            let last_spoken_turn = memory_entry.0;
            memory.insert(speaking, (turn, last_spoken_turn));
            //println!("Self.memory[{}] = ({}, {})", speaking, turn, last_spoken_turn );
        } else {
            memory.insert(speaking, (turn, 0));
            //println!("Self.memory[{}] = ({}, 0)", speaking, turn );
        }

    }
}

impl Day for Day15 {
    fn day_name(&self) -> String { String::from("15") }
    fn answer1(&self) -> String { String::from("763") }
    fn answer2(&self) -> String { String::from("1876406") }

    fn solve(&mut self) -> (String, String) {
        let (part1, part2) = self.run();

        //println!("{:?}", (part1.to_string(), part2.to_string()));
        (part1.to_string(), part2.to_string())
    }
}