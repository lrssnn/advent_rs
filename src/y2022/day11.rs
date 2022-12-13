use std::fmt::Display;
use std::collections::HashMap;

use super::super::day::Day;

pub struct Day11
{
    monkeys: HashMap<usize, Monkey>,
}

impl Day11 {
    pub fn new() -> Day11
    {
        let input = include_str!("input11");
        //let input = include_str!("input11_example");

        let monkeys = input.trim().split("\n\n")
            .map(Monkey::from_str).map(|m| (m.id, m)).collect();

        Day11 { monkeys }
    }
}

impl Day for Day11 {
    fn day_name(&self) -> String { String::from("11") }
    fn answer1(&self) -> String { String::from("76728") }
    fn answer2(&self) -> String { String::from("?") }

    fn solve(&mut self) -> (String, String)
    {
        //for monkey in self.monkeys.values() { println!("{monkey}"); }
        let mut copy = self.monkeys.clone();

        let ans1 = Self::find_active_score(&mut self.monkeys, 20, true);
        let ans2 = Self::find_active_score(&mut copy, 10000, false);

        println!("{ans1}, {ans2}");
        (ans1.to_string() , ans2.to_string())
    }
}

impl Day11 {
    fn find_active_score(monkeys: &mut HashMap<usize, Monkey>, rounds: usize, decay_worry: bool) -> usize {
        for _round in 0..rounds {
            for active_index in 0..monkeys.len() {
                // Theres no way this (clone -> act -> insert) is the right way to do this
                let mut active_monkey = monkeys.get_mut(&active_index).expect("Invalid Index").clone();
                let mut true_target = monkeys.get_mut(&active_monkey.true_target).expect("Invalid Index").clone();
                let mut false_target = monkeys.get_mut(&active_monkey.false_target).expect("Invalid Index").clone();

                active_monkey.take_turn(&mut true_target, &mut false_target, decay_worry);

                monkeys.insert(active_monkey.true_target, true_target);
                monkeys.insert(active_monkey.false_target, false_target);
                monkeys.insert(active_index, active_monkey);
            }
        }

        let mut comparisons = monkeys.values().map(|m| m.comparisons).collect::<Vec<_>>();
        comparisons.sort();
        let len = comparisons.len();
        comparisons[len -2] * comparisons[len -1]
    }
}

#[derive(Clone)]
struct Monkey {
    id: usize,
    items: Vec<f64>,
    operation: Operation,
    test_divisor: usize,
    true_target: usize,
    false_target: usize,
    comparisons: usize,
}

#[derive(Copy, Clone)]
enum Operation {
    Mul(f64),
    Add(f64),
    Square,
}

impl Monkey {
    fn from_str(input: &str) -> Monkey {
        let lines = input.split('\n').collect::<Vec<_>>();
        // Monkey ids always single digit
        let id = lines[0].chars().collect::<Vec<_>>()[7]
            .to_string().parse::<usize>().expect("Invalid monkey id");
        
        let items = lines[1].split(':').collect::<Vec<_>>()[1]
            .trim().split(", ").map(|e| e.parse().expect("Invalid item..."))
            .collect::<Vec<_>>();
        
        let operation = Operation::from_str(lines[2]);
        
        let test_divisor = lines[3].split(' ').last().unwrap().parse::<usize>().expect("Invalid Test Divisor");
        let true_target = lines[4].split(' ').last().unwrap().parse::<usize>().expect("Invalid true target");
        let false_target = lines[5].split(' ').last().unwrap().parse::<usize>().expect("Invalid false target");
        
        Monkey {
            id, items, operation, test_divisor, true_target, false_target, comparisons: 0,
        }
    }
    
    fn take_turn(&mut self, true_target: &mut Monkey, false_target: &mut Monkey, decay_worry: bool) {
        for item in &self.items {
            self.comparisons += 1;
            //println!("Processing {item}");

            // Inspecting the item causes stress to rise
            let mut new_stress = self.operation.apply(*item);            
            //println!("Stress Inflated to {new_stress}");

            // Stress decays
            if decay_worry {new_stress = (new_stress/3.0).floor(); }
            //println!("Stress decayed to {new_stress}");

            // Test worry level and distrbute
            if new_stress % (self.test_divisor as f64) == 0.0 {
                //println!("was divisible by {}, sending to monkey {}", self.test_divisor, true_target.id);
                let target_index = true_target.items.len();
                true_target.items.insert(target_index, new_stress);
            } else {
                //println!("was NOT divisible by {}, sending to monkey {}", self.test_divisor, false_target.id);
                let target_index = false_target.items.len();
                false_target.items.insert(target_index, new_stress);
            }
        }
        
        self.items = vec![];

    }
}

impl Operation {
    fn from_str(input: &str) -> Operation {
        let parts = input.trim().split(' ').collect::<Vec<_>>();
        match parts[4] {
            "*" => if parts[5].eq("old") {
                    Operation::Square
                } else {
                    Operation::Mul(parts[5].parse::<f64>().expect("Invalid Operand"))
                },
            "+" => Operation::Add(parts[5].parse::<f64>().expect("Invalid Operand")),
            _ => panic!("Invalid Operation"),
        }
    }
    
    fn apply(&self, op: f64) -> f64 {
        match self {
            Operation::Mul(x) => op * x,
            Operation::Add(x) => op + x,
            Operation::Square => op * op,
        }
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Operation: new = old {}", match self {
            Operation::Add(x) => format!("+ {x}"),
            Operation::Mul(x) => format!("* {x}"),
            Operation::Square => "* old".to_string(),
        })
    }
}

impl Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Monkey {} | Op: {} | Test: Divisible By {} | True: {} False {}",
            self.id, self.operation, self.test_divisor, self.true_target, self.false_target).expect("!");
        write!(f, "  {}", self.items.iter().map(f64::to_string).collect::<Vec<_>>().join(", "))
    }
}