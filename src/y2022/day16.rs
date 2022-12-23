use std::{fmt::Display, collections::HashMap, ops::{RangeInclusive, RangeTo}, io::Write, time::Instant};

use super::super::day::Day;

pub struct Day16
{
    valves: HashMap<String, Valve>,
}

impl Day16 {
    pub fn new() -> Day16
    {
        let input = include_str!("input16");
        let input = include_str!("input16_example");

        let valves = input.trim().split('\n')
            .map(|line| {
                let v = Valve::from_str(line);
                (v.id.to_string(), v)
            }).collect::<HashMap<_,_>>();


        Day16 { valves }
    }
}

impl Day for Day16 {
    fn day_name(&self) -> String { String::from("16") }
    fn answer1(&self) -> String { String::from("?") }
    fn answer2(&self) -> String { String::from("?") }

    fn solve(&mut self) -> (String, String)
    {
        for valve in self.valves.values() {
            println!("{valve}");
        }

        let ans1 = 0;

        let ans2 = 0;

        println!("{ans1}, {ans2}");
        (ans1.to_string() , ans2.to_string())
    }
}

impl Day16 {
}

#[derive(Debug)]
struct Valve {
    id: String,
    rate: u32,
    tunnel_ids: Vec<String>,
}

impl Valve {
    fn from_str(input: &str) -> Valve {
        println!("{input}");
        let mut parts = input.split("valves ").collect::<Vec<_>>();
        if parts.len() == 1 {
            // single valve
            parts = input.split("valve ").collect::<Vec<_>>();
        }
        let tunnel_ids = parts[1].split(", ").map(|s| s.to_string()).collect::<Vec<_>>();
        let parts = parts[0].split(' ').collect::<Vec<_>>();
        let id = parts[1].to_string();
        let rate_s = parts[4];
        let rate = rate_s[5..rate_s.len()-1].parse::<u32>().unwrap();
        Valve { id, rate, tunnel_ids }
    }
}

impl Display for Valve {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({}) -> {}", self.id, self.rate,
            self.tunnel_ids.iter().fold("".to_string(), |acc, x| acc + x + ",")
        )
    }
}