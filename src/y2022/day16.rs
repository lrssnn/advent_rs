use std::{fmt::Display, collections::{HashMap, HashSet}, iter, hash::{Hash, Hasher}};

use super::super::day::Day;

pub struct Day16
{
    valves: Vec<Valve>,
}

impl Day16 {
    pub fn new() -> Day16
    {
        let input = include_str!("input16");
        //let input = include_str!("input16_example");

        let mut valves = input.trim().split('\n')
            .map(ValveString::from_str)
            .collect::<Vec<_>>();

        // We rely on "AA" being first, technically this is more work than we need to do
        valves.sort_by_key(|v| v.id.to_string());

        Day16 { valves: Valve::from_valve_strings(&valves) }
    }
}

impl Day for Day16 {
    fn day_name(&self) -> String { String::from("16") }
    fn answer1(&self) -> String { String::from("2265") }
    fn answer2(&self) -> String { String::from("2811") }

    // SLOW! 5558ms for non-example input (release)
    // 22ms for example
    fn part1(&mut self) -> String {
        self.consolidate_zero_rate_valves();

        let initial_state = State {
            me_at: 0,
            elephant_at: 0,
            me_travel: 0,
            elephant_travel: 50, // This means the elephant isn't going to do anything

            time_left: 29,
            score: 0,
            active_ids: HashSet::new(),
        };
        self.find_best(&initial_state, &mut HashMap::new(), 0, true).to_string()
    }

    #[allow(unreachable_code)]
    fn part2(&mut self) -> String {
        let initial_state = State {
            me_at: 0,
            elephant_at: 0,
            me_travel: 0,
            elephant_travel: 0,

            //time_left: 25,
            time_left: 25,
            score: 0,
            active_ids: HashSet::new(),
        };
        let score = self.find_best(&initial_state, &mut HashMap::new(), 0, false);
        if score >= 2815 { println!("SCORE TOO HIGH!");}
        score.to_string()
    }
}

impl Day16 {
    fn find_best(&self, from: &State, cache: &mut HashMap<State, u16>, mut best: u16, add_one_turn: bool) -> u16 {
        if let Some(cached) = cache.get(from) {
            return *cached;
        }

        let me_valve = self.valves.iter().find(|v| v.id == from.me_at).unwrap();
        let el_valve = self.valves.iter().find(|v| v.id == from.elephant_at).unwrap();

        if from.time_left == 0 {
            cache.insert(from.clone(), from.score);
            return from.score;
        }

        let me_choices = Self::get_choices(me_valve, &from.active_ids, from.time_left, from.me_travel);
        let el_choices = Self::get_choices(el_valve, &from.active_ids, from.time_left, from.elephant_travel);
        
        // Cross product of my options and the elephant's
        let children = me_choices.iter()
            .flat_map(|me_choice| el_choices.iter().map(move |el_choice| (me_choice, el_choice))) // Yields all choice combinations
            .filter_map(|(me_choice, el_choice)| {
                from.apply_choices(me_choice.clone(), el_choice.clone())
            }) // Turn those choices into a state
            .collect::<Vec<_>>();

        if children.is_empty() {
            cache.insert(from.clone(), from.score);
            cache.insert(from.clone(), from.score);
            from.score
        } else {
            // Filter out states that couldn't possibly increase our current best
            let mut score = 0;
            for state in &children {
                if state.potential_score(&self.valves, add_one_turn) > best {
                    score = u16::max(score, self.find_best(state, cache, best, add_one_turn));
                    best = u16::max(best, score);
                }
            } 
            cache.insert(from.clone(), score);
            score
        }
    }

    fn consolidate_zero_rate_valves(&mut self) {
        let mut map = self.valves.iter().map(|v| (v.id, v.clone())).collect::<HashMap<_,_>>();

        let mut next_key = map.values().find(|v| v.rate == 0 && v.id != 0).unwrap().id;

        while let Some(dead_valve) = map.remove(&next_key) {
            // For each place this valve can go, replace that one's path to here with paths to this one's other places

            for (other_id, dead_other) in &dead_valve.paths {
                let other = map.get_mut(other_id).unwrap();

                for (dest_id, dead_dest) in &dead_valve.paths {
                    if *dest_id != other.id {
                        other.paths.push((*dest_id, dead_other + dead_dest));
                    }
                }

                let remove_index = other.paths.iter().position(|e| e.0.eq(&dead_valve.id)).unwrap();
                other.paths.remove(remove_index);
            }

            if let Some(next_valve) = map.values().find(|v| v.rate == 0 && v.id != 0) {
                next_key = next_valve.id;
            } else {
                break;
            }
        }

        let valves = map.values().cloned().collect::<Vec<_>>();
        self.valves = valves;
    }

    fn get_choices(current_valve: &Valve, active_ids: &HashSet<u8>, time_left: u8, travel_time: u8) -> Vec<Choice> {
        if travel_time > 0 {
            return vec![Choice::Travel];
        } 

        // Switch on a valve?
        let mut choices = vec![];
        if current_valve.rate > 0 && !active_ids.contains(&current_valve.id) {
            choices.push(Choice::Activate(current_valve.id, current_valve.rate));
        }

        // Move?
        for destination in &current_valve.paths {
            if destination.1 < time_left {
                choices.push(Choice::Move(*destination));
            }
        }

        if choices.is_empty() {
            choices.push(Choice::DoNothing);
        }

        choices
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    me_at: u8,
    elephant_at: u8,
    me_travel: u8,
    elephant_travel: u8,

    time_left: u8,
    score: u16,
    active_ids: HashSet<u8>,
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.me_at.hash(state);
        self.elephant_at.hash(state);
        self.me_travel.hash(state);
        self.elephant_travel.hash(state);
        self.time_left.hash(state);
        let mut vec = self.active_ids.iter().collect::<Vec<&u8>>();
        vec.sort();
        vec.hash(state);
    }
}

#[derive(Clone, Debug)]
#[derive(PartialEq)]
enum Choice {
    Activate(u8, u8),
    Move(Path),
    Travel,
    DoNothing,
}

type Path = (u8, u8);

impl State {
    fn apply_choices(&self, me_choice: Choice, elephant_choice: Choice) -> Option<State> {
        // Do not turn on the same valve at the same time
        if let Choice::Activate(ref v, _) = me_choice {
            if let Choice::Activate(ref v2, _) = elephant_choice {
                if v == v2 {
                    return None;
                }
            }
        }

        // Do not both do nothing
        if let Choice::DoNothing = me_choice {
            if let Choice::DoNothing = elephant_choice {
                return None;
            }
        }

        Some(self.apply_me_choice(me_choice)
            .apply_elephant_choice(elephant_choice)
            .apply_time_step())
    }

    fn apply_me_choice(&self, choice: Choice) -> State {
        match choice {
            Choice::Activate(valve, rate) => {
                let mut active_ids = self.active_ids.clone();
                active_ids.insert(valve);
                State {
                    active_ids,
                    score: self.score + (rate as u16 * self.time_left as u16),
                    ..self.clone()
                }
            }
            Choice::Move(path) => {
                let me_at = path.0;
                let me_travel = path.1;
                State {
                    me_at,
                    me_travel,
                    ..self.clone()
                }
            },
            Choice::Travel | Choice::DoNothing => {
                self.clone()
            }
        }
    }

    fn apply_elephant_choice(&self, choice: Choice) -> State {
        match choice {
            Choice::Activate(valve, rate) => {
                let mut active_ids = self.active_ids.clone();
                active_ids.insert(valve);
                State {
                    active_ids,
                    score: self.score + (rate as u16 * self.time_left as u16),
                    ..self.clone()
                }
            }
            Choice::Move(path) => {
                let elephant_at = path.0;
                let elephant_travel = path.1;
                State {
                    elephant_at,
                    elephant_travel,
                    ..self.clone()
                }
            },
            Choice::Travel | Choice::DoNothing => {
                self.clone()
            }
        }
    }

    fn apply_time_step(&self) -> State {
        State {
            time_left: self.time_left - 1,
            me_travel: self.me_travel.saturating_sub(1),
            elephant_travel: self.elephant_travel.saturating_sub(1),
            ..self.clone()
        }
    }

    fn potential_score(&self, valves: &[Valve], add_one_turn: bool) -> u16 {
        // We can put a cap on the potential score by imagining that we can turn on the remaining valves in the shortest possible time
        let mut score = self.score;
        // I have no idea why... but this is required for part1, and makes part 2 a LOT slower
        let mut sim_time_left = self.time_left + if add_one_turn { 1 } else { 0 };
        let mut valves: Vec<_> = valves.iter().filter(|v| !self.active_ids.contains(&v.id)).collect();

        valves.sort_by(|a, b| b.rate.cmp(&a.rate));

        for v in valves {
            score += v.rate as u16 * sim_time_left as u16;
            sim_time_left -= 1;
            if sim_time_left == 0 {
                return score;
            }
        }
        score
    }
}

#[derive(Debug, Clone)]
struct ValveString {
    id: String,
    rate: u8,
    paths: Vec<(String, u8)>,
}

#[derive(Debug, Clone)]
struct Valve {
    id: u8,
    rate: u8,
    paths: Vec<(u8, u8)>,
}

impl ValveString {
    fn from_str(input: &str) -> ValveString {
        let mut parts = input.split("valves ").collect::<Vec<_>>();
        if parts.len() == 1 {
            // single valve
            parts = input.split("valve ").collect::<Vec<_>>();
        }
        let paths = parts[1].split(", ").map(|s| s.to_string()).zip(iter::repeat(1)).collect::<Vec<_>>();
        let parts = parts[0].split(' ').collect::<Vec<_>>();
        let id = parts[1].to_string();
        let rate_s = parts[4];
        let rate = rate_s[5..rate_s.len()-1].parse::<u8>().unwrap();
        ValveString { id, rate, paths }
    }
}

impl Valve {
    fn from_valve_strings(input: &[ValveString]) -> Vec<Valve> {
        // Determine mapping from string id to int id
        assert!(input[0].id.eq("AA"));
        let string_to_u8 = input.iter()
            .enumerate()
            .map(|(i, valve)| (valve.id.to_string(), i as u8))
            .collect::<HashMap<String, u8>>();

        input.iter().map(|v| Valve::from_valve_string(v, &string_to_u8)).collect()
    }

    fn from_valve_string(input: &ValveString, id_map: &HashMap<String, u8>) -> Valve {
        let paths = input.paths.iter().map(|(id_str, cost)| (*id_map.get(id_str).unwrap(), *cost)).collect::<Vec<_>>();
        Valve {
            id: *id_map.get(&input.id).unwrap(),
            rate: input.rate,
            paths
        }
    }
}

impl Display for Valve {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02} ({}) -> {}", self.id, self.rate,
            self.paths.iter().fold("".to_string(), |acc, x| acc + &x.1.to_string() + ": " + &x.0.to_string() + ",")
        )
    }
}