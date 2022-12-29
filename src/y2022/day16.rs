use std::{fmt::Display, collections::HashMap};

use super::super::day::Day;

pub struct Day16
{
    valves: HashMap<String, Valve>,
}

impl Day16 {
    #[allow(dead_code)]
    pub fn new() -> Day16
    {
        let input = include_str!("input16");
        //let input = include_str!("input16_example");

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
    fn answer1(&self) -> String { String::from("2265") }
    fn answer2(&self) -> String { String::from("?") }

    fn solve(&mut self) -> (String, String)
    {
        /*
        for valve in self.valves.values() {
            println!("{valve}");
        }
        */

        //println!("{}", self.valves.len());

        let initial_state = State {
            at_id: "AA".to_string(),
            time_left: 30,
            score: 0,
            score_inc: 0,
            active_ids: Vec::new(),
        };

        let ans1 = self.find_best(&initial_state, &mut HashMap::new());

        let ans2 = 0;

        //println!("{ans1}, {ans2}");
        (ans1.to_string() , ans2.to_string())
    }
}

impl Day16 {
    fn find_best(&self, from: &State, cache: &mut HashMap<State, u32>) -> u32 {
        if let Some(cached) = cache.get(from) {
            //println!("Cache Hit");
            return *cached;
        }

        //println!("{from}");

        // Check for terminal state:
        if from.time_left == 0 {
            return from.score;
        }

        let this_valve = self.valves.get(&from.at_id).unwrap();

        let mut children = vec![];

        // Switch on our valve?
        if this_valve.rate > 0 && !from.active_ids.contains(&from.at_id) {
            let score = self.find_best(&from.activate(&from.at_id, this_valve.rate), cache);
            cache.insert(from.clone(), score);
            return score;
            //children.push(from.activate(&from.at_id, this_valve.rate));
        }

        for destination in &this_valve.tunnel_ids {
            children.push(from.travel_to(destination));
        }

        let score = children.iter().map(|state| self.find_best(state, cache)).max().unwrap();
        cache.insert(from.clone(), score);
        score
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    at_id: String,
    time_left: u32,
    score: u32,
    score_inc: u32,
    active_ids: Vec<String>,
}

impl State {
    fn activate(&self, to_activate: &str, value: u32) -> State {
        let mut active_ids = self.active_ids.clone();
        active_ids.push(to_activate.to_string());

        State {
            at_id: self.at_id.clone(),
            time_left: self.time_left - 1,
            score: self.score + self.score_inc,
            score_inc: self.score_inc + value,
            active_ids,
        }
    }

    fn travel_to(&self, destination: &str) -> State {
        State {
            at_id: destination.to_string(),
            time_left: self.time_left - 1,
            score: self.score + self.score_inc,
            score_inc: self.score_inc,
            active_ids: self.active_ids.clone(),
        }
    }
}

#[derive(Debug)]
struct Valve {
    id: String,
    rate: u32,
    tunnel_ids: Vec<String>,
}

impl Valve {
    fn from_str(input: &str) -> Valve {
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

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {} left - score {} - activated: {}", self.at_id, self.time_left, self.score,
            self.active_ids.iter().fold("".to_string(), |acc, x| acc + x + " ")
        )
    }
}