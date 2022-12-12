use std::fmt::Display;

use super::super::day::Day;

pub struct Day11
{
    monkeys: Vec<Monkey>,
}

impl Day11 {
    pub fn new() -> Day11
    {
        let input = include_str!("input11");
        let input = include_str!("input11_example");

        let monkeys = input.trim().split("\n\n")
            .map(Monkey::from_str).collect();

        Day11 { monkeys }
    }
}

impl Day for Day11 {
    fn day_name(&self) -> String { String::from("11") }
    fn answer1(&self) -> String { String::from("?") }
    fn answer2(&self) -> String { String::from("?") }

    fn solve(&mut self) -> (String, String)
    {
        for monkey in &self.monkeys {
            println!("{monkey}");
        }
        let ans1 = self.find_active_score();
        let ans2 = 0;

        println!("{ans1}, {ans2}");
        (ans1.to_string() , ans2.to_string())
    }
}

impl Day11 {
    fn find_active_score(&mut self) -> usize {
        Monkey::take_turn(0, &mut self.monkeys);
        
        0
    }
}

struct Monkey {
    id: usize,
    items: Vec<usize>,
    operation: Operation,
    test_divisor: usize,
    true_target: usize,
    false_target: usize,
    comparisons: usize,
}

enum Operation {
    Mul(usize),
    Add(usize),
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
    
    fn take_turn(active_monkey_id: usize, monkeys: &mut Vec<Monkey>) {
        let active_monkey = &monkeys[active_monkey_id];

        for item in &active_monkey.items {
            active_monkey.comparisons += 1;
            println!("Processing {item}");
            // Inspecting the item causes stress to rise
            let mut new_stress = active_monkey.operation.apply(*item);            
            println!("Stress Inflated to {new_stress}");
            // Stress decays
            new_stress /= 3;
            println!("Stress decayed to {new_stress}");
            // Test worry level and distrbute
            if new_stress % active_monkey.test_divisor == 0 {
                println!("was divisible by {}, sending to monkey {}", active_monkey.test_divisor, active_monkey.true_target);
                let target_items = &mut monkeys[active_monkey.true_target].items;
                target_items.insert(new_stress, target_items.len());
            } else {
                println!("was NOT divisible by {}, sending to monkey {}", active_monkey.test_divisor, active_monkey.false_target);
                let target_items = &mut monkeys[active_monkey.false_target].items;
                target_items.insert(new_stress, target_items.len());
            }
        }
        
        monkeys[active_monkey_id].items = vec![];

    }
}

impl Operation {
    fn from_str(input: &str) -> Operation {
        let parts = input.trim().split(' ').collect::<Vec<_>>();
        match parts[4] {
            "*" => if parts[5].eq("old") {
                    Operation::Square
                } else {
                    Operation::Mul(parts[5].parse::<usize>().expect("Invalid Operand"))
                },
            "+" => Operation::Add(parts[5].parse::<usize>().expect("Invalid Operand")),
            _ => panic!("Invalid Operation"),
        }
    }
    
    fn apply(&self, op: usize) -> usize {
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
        write!(f, "  {}", self.items.iter().map(usize::to_string).collect::<Vec<_>>().join(", "))
    }
}