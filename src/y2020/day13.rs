use std::{fs, ops::Index};
use super::super::day::Day;

pub struct Day13 {
    earliest_time: usize,
    buses: Vec<Option<usize>>,
}

impl Day13 {
    pub fn new() -> Day13 {
        let input = fs::read_to_string("src/y2020/input13").expect("File Read Error");
    
        //let input = "939\n7,13,x,x,59,x,31,19";

        let mut lines = input.trim().split('\n').map(|s| s.trim());

        let earliest_time = lines.next().expect("").parse::<usize>().expect("");
        let buses = lines.next().expect("").split(',').map(|b| {
            if b == "x" { return None }
            Some(b.parse::<usize>().expect(""))
        }).collect();

        Day13 { earliest_time, buses }
    }

    fn find_part_1(&self) -> usize {
        let mut time = self.earliest_time;
        let mut bus: Option<usize> = None;
        while bus.is_none() {
            time += 1;
            bus = self.buses.iter().find_map(|b_opt| {
                if let Some(b) = b_opt {
                    if time % b == 0 { 
                        return Some(*b)
                    } else { 
                        return None;
                    }
                } else {
                    None
                }
            });
        }
        let wait_time = time - self.earliest_time;
        wait_time * bus.unwrap()
    }

    fn find_part_2(&self) -> usize {
        // Apparently we need to use "Chinese Remainder Theorem" for this. Good luck, future me :)
        let timestep: usize = self.buses.iter().filter_map(|b| *b).max().unwrap(); // Take the biggest jumps possible
        let offset = self.buses.iter().position(|b_opt| 
            if let Some(b) = b_opt {
                if *b == timestep { true } else { false }
            } else { false }
        ).unwrap();
        let mut time = timestep - offset; // This should mean we check the right intervals to look from left to right
        println!("{}. Starting at {}", timestep, time);
        'outer: loop {
            time += timestep;
            //println!("Checking {}", time);
            for (i, bus) in self.buses.iter().enumerate() {
                if let Some(b) = bus {
                    if (time + i) % b != 0 { continue 'outer; }
                }
            }

            break;
        }
        time
    }
}

impl Day for Day13 {
    fn day_name(&self) -> String { String::from("13") }
    fn answer1(&self) -> String { String::from("2845") }
    fn answer2(&self) -> String { String::from("?") }

    fn solve(&mut self) -> (String, String) {
        let part1 = self.find_part_1();
        let part2 = self.find_part_2();

        println!("{:?}", (part1.to_string(), part2.to_string()));
        (part1.to_string(), part2.to_string())
    }
}
