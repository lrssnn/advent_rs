use std::{fmt::Display, collections::HashMap, iter};

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

    fn part1(&mut self) -> String {
        let initial_state = State {
            at_id: "AA".to_string(),
            time_left: 30,
            score: 0,
            active_ids: Vec::new(),
        };

        self.consolidate_zero_rate_valves();

        self.find_best(&initial_state, &mut HashMap::new()).to_string()
    }

    fn part2(&mut self) -> String {
        /*
        let initial_state = State2 {
            me_at: "AA".to_string(),
            elephant_at: "AA".to_string(),
            me_travel: 0,
            elephant_travel: 0,

            time_left: 26,
            score: 0,
            active_ids: Vec::new(),
        };
        */
        "unsolved".to_string()
    }
}

impl Day16 {
    fn find_best(&self, from: &State, cache: &mut HashMap<State, u32>) -> u32 {
        if let Some(cached) = cache.get(from) {
            //println!("Cache Hit");
            return *cached;
        }

        let this_valve = self.valves.get(&from.at_id).unwrap();

        /*
        println!("{from}");
        println!("This valve: {this_valve}");
        */

        // Check for terminal state:
        let min_time = this_valve.paths.iter().map(|path| path.1).min().unwrap();
        if from.time_left < min_time {
            // if we have time, we should turn on this valve. Hacky
            if from.time_left > 0 && !from.active_ids.contains(&from.at_id) {
                return from.score + (this_valve.rate * from.time_left);
            }
            return from.score;
        }

        let mut children = vec![];

        // Switch on our valve?
        if this_valve.rate > 0 && !from.active_ids.contains(&from.at_id) {
            let score = self.find_best(&from.activate(&from.at_id, this_valve.rate), cache);
            cache.insert(from.clone(), score);
            return score;
        }

        for destination in &this_valve.paths {
            if destination.1 <= from.time_left {
                children.push(from.travel_to(destination));
            }
        }

        let score = children.iter().map(|state| self.find_best(state, cache)).max().unwrap();
        cache.insert(from.clone(), score);
        score
    }

    #[allow(dead_code)]
    fn find_best_2(&self) -> u32 {
        // TODO Don't think we can just clone above. need to think about this some more...
        0
    }

    fn consolidate_zero_rate_valves(&mut self) {
        let mut next_key = self.valves.values().find(|v| v.rate == 0 && v.id.ne("AA")).unwrap().id.to_string();
        while let Some(dead_valve) = self.valves.remove(&next_key) {
            // For each place this valve can go, replace that one's path to here with paths to this one's other places

            for (other_id, dead_other) in &dead_valve.paths {
                let other = self.valves.get_mut(other_id).unwrap();

                for (dest_id, dead_dest) in &dead_valve.paths {
                    if dest_id.ne(&other.id) {
                        other.paths.push((dest_id.to_string(), dead_other + dead_dest));
                    }
                }

                let remove_index = other.paths.iter().position(|e| e.0.eq(&dead_valve.id)).unwrap();
                other.paths.remove(remove_index);
            }

            if let Some(next_valve) = self.valves.values().find(|v| v.rate == 0 && v.id.ne("AA")) {
                next_key = next_valve.id.to_string();
            } else {
                next_key = "INVALID - WONT FIND IN WHILE LET ABOVE".to_string();
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    at_id: String,
    time_left: u32,
    score: u32,
    active_ids: Vec<String>,
}

impl State {
    fn activate(&self, to_activate: &str, value: u32) -> State {
        let mut active_ids = self.active_ids.clone();
        active_ids.push(to_activate.to_string());

        State {
            at_id: self.at_id.clone(),
            time_left: self.time_left - 1,
            score: self.score + (value * (self.time_left - 1)),
            active_ids,
        }
    }

    fn travel_to(&self, destination: &(String, u32)) -> State {
        State {
            at_id: destination.0.to_string(),
            time_left: self.time_left - destination.1,
            score: self.score,
            active_ids: self.active_ids.clone(),
        }
    }
}

// TODO TODO TODO Please do not do this - Please make this support part 1 as well once we have part 2
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State2 {
    me_at: String,
    elephant_at: String,
    me_travel: u8,
    elephant_travel: u8,

    time_left: u32,
    score: u32,
    active_ids: Vec<String>,
}

impl State2 {
    // TODO, I'm thinking we should have something like this
    // fn apply_choices(&self, me_choice: Choice, elephant_choice: Choice) -> State2 {
        // let result = self.apply_me_choice(me_choice);
        // result = result.apply_elephant_choice(elephant_choice);
        // result = result.apply_time_step()
        // result
    // }
    // where Choice is an enum of Activate(some_valve) or Move(some_path), and apply_time_step reduces the time
    // left and applies travel logic
    // Then the search can separately enumerate the options 'me' has and the elephant has, and call this with every
    // combination
    // Neat! but I can't look at this any more right now
    #[allow(dead_code)]
    fn activate(&self, to_activate_1: &str, to_activate_2: &str, value_1: u32, value_2: u32) -> State2 {
        // if someone is traveling, update their travel time
        let me_travel = self.me_travel.saturating_sub(1);
        let elephant_travel = self.elephant_travel.saturating_sub(1);

        let mut active_ids = self.active_ids.clone();

        active_ids.push(to_activate_1.to_string());

        // Let's make the second activation optional
        if !to_activate_2.is_empty() {
            active_ids.push(to_activate_2.to_string());
        }

        State2 {
            me_at: self.me_at.clone(),
            elephant_at: self.elephant_at.clone(),
            me_travel,
            elephant_travel,
            time_left: self.time_left - 1,
            score: self.score + (value_1 * (self.time_left - 1) + (value_2 * self.time_left - 1)),
            active_ids,
        }
    }

    #[allow(dead_code)]
    fn travel_to(&self, me_path: &(String, u32), elephant_path: &(String, u32)) -> State2 {
        let me_at = if !me_path.0.is_empty() { me_path.0.clone() } else { self.me_at.clone() };
        let elephant_at = if !elephant_path.0.is_empty() { elephant_path.0.clone() } else { self.elephant_at.clone() };

        // This is weird - Its the travel time left AFTER we spend the one time step implicit in the state transition
        let me_travel = me_path.1.saturating_sub(1) as u8;
        let elephant_travel = elephant_path.1.saturating_sub(1) as u8;
        State2 {
            me_at,
            elephant_at,
            me_travel,
            elephant_travel,
            time_left: self.time_left - 1,
            score: self.score,
            active_ids: self.active_ids.clone(),
        }
    }
}

#[derive(Debug, Clone)]
struct Valve {
    id: String,
    rate: u32,
    paths: Vec<(String, u32)>,
}

impl Valve {
    fn from_str(input: &str) -> Valve {
        let mut parts = input.split("valves ").collect::<Vec<_>>();
        if parts.len() == 1 {
            // single valve
            parts = input.split("valve ").collect::<Vec<_>>();
        }
        let paths = parts[1].split(", ").map(|s| s.to_string()).zip(iter::repeat(1)).collect::<Vec<_>>();
        let parts = parts[0].split(' ').collect::<Vec<_>>();
        let id = parts[1].to_string();
        let rate_s = parts[4];
        let rate = rate_s[5..rate_s.len()-1].parse::<u32>().unwrap();
        Valve { id, rate, paths }
    }
}

impl Display for Valve {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02} ({}) -> {}", self.id, self.rate,
            self.paths.iter().fold("".to_string(), |acc, x| acc + &x.1.to_string() + ": " + &x.0 + ",")
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