use std::{fmt::Display, collections::{HashMap, HashSet}, iter, hash::{Hash, Hasher}};

use super::super::day::Day;

pub struct Day16
{
    valves: HashMap<String, Valve>,
}

impl Day16 {
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
    fn answer2(&self) -> String { String::from("2811") }

    // SLOW! 5558ms for non-example input (release)
    // 22ms for example
    fn part1(&mut self) -> String {
        self.consolidate_zero_rate_valves();

        let initial_state = State {
            me_at: "AA".to_string(),
            elephant_at: "AA".to_string(),
            me_travel: 0,
            elephant_travel: 30, // This means the elephant isn't going to do anything

            time_left: 29,
            score: 0,
            active_ids: HashSet::new(),
        };
        self.find_best(&initial_state, &mut HashMap::new(), 0).to_string()
    }

    fn part2(&mut self) -> String {
        let initial_state = State {
            me_at: "AA".to_string(),
            elephant_at: "AA".to_string(),
            me_travel: 0,
            elephant_travel: 0,

            //time_left: 25,
            time_left: 25,
            score: 0,
            active_ids: HashSet::new(),
        };
        let score = self.find_best(&initial_state, &mut HashMap::new(), 0);
        if score >= 2815 { println!("SCORE TOO HIGH!");}
        score.to_string()
    }
}

impl Day16 {
    fn find_best(&self, from: &State, cache: &mut HashMap<State, u32>, mut best: u32) -> u32 {
        if let Some(cached) = cache.get(from) {
            return *cached;
        }

        let me_valve = self.valves.get(&from.me_at).unwrap();
        let el_valve = self.valves.get(&from.elephant_at).unwrap();

        if from.time_left == 0 {
            cache.insert(from.clone(), from.score);
            return from.score;
        }

        let me_choices = Self::get_choices(me_valve, &from.active_ids, from.time_left, from.me_travel);
        let el_choices = Self::get_choices(el_valve, &from.active_ids, from.time_left, from.elephant_travel);
        
        let children = if me_choices.is_empty() {
            println!("In Here");
            el_choices.iter().filter_map(|el_choice| from.apply_choices(Choice::DoNothing, el_choice.clone())).collect::<Vec<_>>()
        } else if el_choices.is_empty() {
            println!("In There");
            me_choices.iter().filter_map(|me_choice| from.apply_choices(me_choice.clone(), Choice::DoNothing)).collect::<Vec<_>>()
        } else {
            me_choices.iter()
            .flat_map(|me_choice| el_choices.iter().map(move |el_choice| (me_choice, el_choice))) // Yields all choice combinations
            .filter_map(|(me_choice, el_choice)| {
                from.apply_choices(me_choice.clone(), el_choice.clone())
            }) // Turn those choices into a state
            .collect::<Vec<_>>()
        };

        // println!("My choices...");
        // for c in &me_choices { println!("  {c:?}");}

        // println!("Elephant choices...");
        // for c in &el_choices { println!("  {c:?}");}

        //  println!("Combinations: ");
        //  for child in &children {
        //      println!("  {child:?}");
        //  }

        if children.is_empty() {
            cache.insert(from.clone(), from.score);
            //println!("{score}, {} scores evaluated...", cache.len());
            return from.score;
        } else {
            // Filter out states that couldn't possibly increase our current best
            let mut score = 0;
            for state in &children {
                if state.potential_score(&self.valves) > best {
                    score = u32::max(score, self.find_best(state, cache, best));
                    best = u32::max(best, score);
                }
            } 
            cache.insert(from.clone(), score);
            score
        }
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

    fn get_choices(current_valve: &Valve, active_ids: &HashSet<String>, time_left: u32, travel_time: u32) -> Vec<Choice> {
        if travel_time > 0 {
            return vec![Choice::Travel];
        } 

        // Switch on a valve?
        let mut choices = vec![];
        if current_valve.rate > 0 && !active_ids.contains(&current_valve.id) {
            choices.push(Choice::Activate(current_valve.id.clone(), current_valve.rate));
        }

        // Move?
        for destination in &current_valve.paths {
            if destination.1 < time_left {
                choices.push(Choice::Move(destination.clone()));
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
    me_at: String,
    elephant_at: String,
    me_travel: u32,
    elephant_travel: u32,

    time_left: u32,
    score: u32,
    active_ids: HashSet<String>,
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.me_at.hash(state);
        self.elephant_at.hash(state);
        self.me_travel.hash(state);
        self.elephant_travel.hash(state);
        self.time_left.hash(state);
        let mut vec = self.active_ids.iter().collect::<Vec<&String>>();
        vec.sort();
        vec.hash(state);
    }
}

#[derive(Clone, Debug)]
#[derive(PartialEq)]
enum Choice {
    Activate(String, u32),
    Move(Path),
    Travel,
    DoNothing,
}

type Path = (String, u32);

impl State {
    fn apply_choices(&self, me_choice: Choice, elephant_choice: Choice) -> Option<State> {
        // Do not turn on the same valve at the same time
        if let Choice::Activate(ref v, _) = me_choice {
            if let Choice::Activate(ref v2, _) = elephant_choice {
                if v == v2 {
                    // println!("Preventing double opening {v} - {v2}");
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
                    score: self.score + (rate * self.time_left),
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
                    score: self.score + (rate * self.time_left),
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

    fn potential_score(&self, valves: &HashMap<String, Valve>) -> u32 {
        // We can put a cap on the potential score by imagining that we can turn on the remaining valves in the shortest possible time
        let mut score = self.score;
        let mut sim_time_left = self.time_left;
        let mut valves: Vec<_> = valves.values().filter(|v| !self.active_ids.contains(&v.id)).collect();
        // for v in &valves { println!("{v}")}
        valves.sort_by(|a, b| b.rate.cmp(&a.rate));
        // println!("--");
        // for v in &valves { println!("{v}")}
        // println!("--");
        // println!("  ");
        for v in valves {
            score += v.rate * sim_time_left;
            sim_time_left -= 1;
            if sim_time_left == 0 {
                return score;
            }
        }
        score

        // let mut score = self.score;
        // for valve in valves.values().filter(|v| !self.active_ids.contains(&v.id)) {
        //     score += valve.rate * self.time_left;
        // }
        // score
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