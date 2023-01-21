use std::{fmt::Display, default::Default};

use rayon::prelude::{ParallelIterator, IntoParallelRefIterator, IndexedParallelIterator};

use super::super::day::Day;

const MAX_STATES: usize = 750000;

pub struct Day19
{
    blueprints: Vec<Blueprint>,
}

impl Day19 {
    pub fn new() -> Day19
    {
        let input = include_str!("input19");
        //let input = include_str!("input19_example");

        let blueprints = input.trim().lines()
            .map(Blueprint::from_str).collect::<Vec<_>>();

        Day19 { blueprints }
    }
}

impl Day for Day19 {
    fn day_name(&self) -> String { String::from("19") }
    fn answer1(&self) -> String { String::from("790") }
    fn answer2(&self) -> String { String::from("7350") }

    fn part1(&mut self) -> String {
        // A smarter man would re-use the calculation of the first three states, but I simply do not want to
        self.blueprints.par_iter().map(|bp| bp.evaluate()).sum::<usize>().to_string()
    }

    fn part2(&mut self) -> String {
        self.blueprints.par_iter().take(3).map(|bp| bp.evaluate_2()).product::<usize>().to_string()
    }
}

impl Day19 {
}

struct Blueprint {
    id: usize,
    ore_bot_cost: Purchase,
    clay_bot_cost: Purchase,
    obsidian_bot_cost: Purchase,
    geode_bot_cost: Purchase,

    max_ore_bots: u8,
    max_clay_bots: u8,
    max_obsidian_bots: u8,
}

impl Blueprint {
    fn from_str(input: &str) -> Blueprint {
        let (_, rest) = input.split_once("Blueprint ").unwrap();
        let (id, rest) = rest.split_once(": Each ore robot costs ").unwrap();
        let (ore, rest) = rest.split_once(" ore. Each clay robot costs ").unwrap();
        let (clay, rest) = rest.split_once(" ore. Each obsidian robot costs ").unwrap();
        let (obsidian_ore, rest) = rest.split_once(" ore and ").unwrap();
        let (obsidian_clay, rest) = rest.split_once(" clay. Each geode robot costs ").unwrap();
        let (geode_ore, rest) = rest.split_once(" ore and ").unwrap();
        let (geode_obsidian, _) = rest.split_once(" obsidian.").unwrap();

        let ore = ore.parse::<u8>().unwrap();
        let clay = clay.parse::<u8>().unwrap();
        let obsidian_ore = obsidian_ore.parse::<u8>().unwrap();
        let obsidian_clay = obsidian_clay.parse::<u8>().unwrap();
        let geode_ore = geode_ore.parse::<u8>().unwrap();
        let geode_obsidian = geode_obsidian.parse::<u8>().unwrap();

        let ore_bot_cost = Purchase::new(PurchaseType::Ore, ore, 0, 0);
        let clay_bot_cost = Purchase::new(PurchaseType::Clay, clay, 0, 0);
        let obsidian_bot_cost = Purchase::new(PurchaseType::Obsidian, obsidian_ore, obsidian_clay, 0);
        let geode_bot_cost = Purchase::new(PurchaseType::Geode, geode_ore, 0, geode_obsidian);

        // Once we have enough of a resource to make any purchase of that resource every turn, there is no point
        // buying any more. This is used in should_buy
        // Code formatting crimes?
        let max_ore_bots = ore_bot_cost.ore_cost.max(clay_bot_cost.ore_cost.max(obsidian_bot_cost.ore_cost.max(geode_bot_cost.ore_cost)));
        let max_clay_bots = ore_bot_cost.clay_cost.max(clay_bot_cost.clay_cost.max(obsidian_bot_cost.clay_cost.max(geode_bot_cost.clay_cost)));
        let max_obsidian_bots = ore_bot_cost.obsidian_cost.max(clay_bot_cost.obsidian_cost.max(obsidian_bot_cost.obsidian_cost.max(geode_bot_cost.obsidian_cost)));

        Blueprint { id: id.parse::<usize>().unwrap(), ore_bot_cost, clay_bot_cost, obsidian_bot_cost, geode_bot_cost, max_ore_bots, max_clay_bots, max_obsidian_bots }
    }

    fn evaluate(&self) -> usize {
        self.max_geodes(24) * self.id
    }

    fn evaluate_2(&self) -> usize {
        self.max_geodes(32)
    }

    fn max_geodes(&self, steps: usize) -> usize {
        let mut states = vec![State::new()];

        for _step in 0..steps {
            states = states.iter().flat_map(|state| state.step(self)).collect();
            if states.len() > MAX_STATES * 2 {
                states.dedup();
                states.sort_by_key(|state| state.heuristic());
                states.reverse();
                states.truncate(MAX_STATES);
            }
        }

        states.iter().max_by_key(|state| state.geode).unwrap().geode as usize
    }
}

#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct State {
    ore_bots: u8,
    clay_bots: u8,
    obsidian_bots: u8,
    geode_bots: u8,

    ore: u8,
    clay: u8,
    obsidian: u8,
    geode: u8,
}

impl State {
    fn new() -> State {
        State {
            ore_bots: 1,
            ..Default::default() // All zero
        }
    }

    fn step(&self, bp: &Blueprint) -> Vec<State> {
        if self.can_buy_geode_every_turn(bp) {
            return vec![self.construct_bot(&bp.geode_bot_cost)];
        }

        let mut result = Vec::with_capacity(5);
        result.push(self.construct_bot(&Purchase::nothing()));

        if self.can_afford(&bp.geode_bot_cost) { 
            result.push(self.construct_bot(&bp.geode_bot_cost));
        }
        if self.should_buy(PurchaseType::Obsidian, bp) && self.can_afford(&bp.obsidian_bot_cost) { 
            result.push(self.construct_bot(&bp.obsidian_bot_cost));
        }
        if self.should_buy(PurchaseType::Clay, bp) && self.can_afford(&bp.clay_bot_cost) { 
            result.push(self.construct_bot(&bp.clay_bot_cost));
        }
        if self.should_buy(PurchaseType::Ore, bp) && self.can_afford(&bp.ore_bot_cost) { 
            result.push(self.construct_bot(&bp.ore_bot_cost));
        }

        result
    }

    fn can_afford(&self, cost: &Purchase) -> bool {
        self.ore >= cost.ore_cost
        && self.clay >= cost.clay_cost
        && self.obsidian >= cost.obsidian_cost
    }

    fn construct_bot(&self, purchase: &Purchase) -> State {
        State {
            ore_bots: self.ore_bots + purchase.ore_bots,
            clay_bots: self.clay_bots + purchase.clay_bots,
            obsidian_bots: self.obsidian_bots + purchase.obsidian_bots,
            geode_bots: self.geode_bots + purchase.geode_bots,

            ore: self.ore - purchase.ore_cost + self.ore_bots,
            clay: self.clay - purchase.clay_cost + self.clay_bots,
            obsidian: self.obsidian - purchase.obsidian_cost + self.obsidian_bots,
            geode: self.geode + self.geode_bots,
        }
    }

    // This heuristic clearly sucks - we can only throw away very few states and still get the right answer
    fn heuristic(&self) -> usize {
        self.ore_bots as usize + 
        self.clay_bots as usize * 2 + 
        self.obsidian_bots as usize * 4 + 
        self.geode_bots as usize * 8 + 
        self.geode as usize * 16
    }

    fn should_buy(&self, resource: PurchaseType, bp: &Blueprint) -> bool {
        match resource {
            PurchaseType::Ore => {
                self.ore_bots < bp.max_ore_bots
            },
            PurchaseType::Clay => {
                self.clay_bots < bp.max_clay_bots
            }
            PurchaseType::Obsidian => {
                self.obsidian_bots < bp.max_obsidian_bots
            }
            _ => panic!("Don't call this with {resource:?}"),
        }
    }

    fn can_buy_geode_every_turn(&self, bp: &Blueprint) -> bool {
        self.ore_bots >= bp.geode_bot_cost.ore_cost && self.obsidian_bots >= bp.geode_bot_cost.obsidian_cost
    }
}

#[derive(Debug)]
struct Purchase {
    ore_cost: u8,
    clay_cost: u8,
    obsidian_cost: u8,

    ore_bots: u8,
    clay_bots: u8,
    obsidian_bots: u8,
    geode_bots: u8,
}

impl Purchase {
    fn new(purchase: PurchaseType, ore_cost: u8, clay_cost: u8, obsidian_cost: u8) -> Purchase {
        match purchase {
            PurchaseType::Ore =>      Purchase { ore_cost, clay_cost, obsidian_cost, ore_bots: 1, clay_bots: 0, obsidian_bots: 0, geode_bots: 0 },
            PurchaseType::Clay =>     Purchase { ore_cost, clay_cost, obsidian_cost, ore_bots: 0, clay_bots: 1, obsidian_bots: 0, geode_bots: 0 },
            PurchaseType::Obsidian => Purchase { ore_cost, clay_cost, obsidian_cost, ore_bots: 0, clay_bots: 0, obsidian_bots: 1, geode_bots: 0},
            PurchaseType::Geode =>    Purchase { ore_cost, clay_cost, obsidian_cost, ore_bots: 0, clay_bots: 0, obsidian_bots: 0, geode_bots: 1},
            PurchaseType::None =>     Purchase { ore_cost, clay_cost, obsidian_cost, ore_bots: 0, clay_bots: 0, obsidian_bots: 0, geode_bots: 0 }
        }
    }

    fn nothing() -> Purchase {
        Purchase::new(PurchaseType::None, 0, 0, 0)
    }
}

#[derive(Debug)]
enum PurchaseType { Ore, Clay, Obsidian, Geode, None }

impl Display for Blueprint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ore: {}, clay: {}, obsidian: {}, geode: {}", self.ore_bot_cost, self.clay_bot_cost, self.obsidian_bot_cost, self.geode_bot_cost)
    }
}

impl Display for Purchase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ore", self.ore_cost).unwrap();
        if self.clay_cost > 0 {
            write!(f, " and {} clay", self.clay_cost).unwrap();
        }
        if self.obsidian_cost > 0 {
            write!(f, " and {} obsidian", self.obsidian_cost).unwrap();
        }
        Ok(())
    }
}