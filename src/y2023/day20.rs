use std::{collections::{HashMap, VecDeque}, fmt::Display};

use super::super::day::Day;

pub struct Day20
{
    modules: HashMap<String, Module>,
}

impl Day20 {
    pub fn new() -> Day20
    {
        //let input = "broadcaster -> a, b, c\n%a -> b\n%b -> c\n%c -> inv\n&inv -> a";
        //let input = "broadcaster -> a\n%a -> inv, con\n&inv -> b\n%b -> con\n&con -> output";
        let input = include_str!("../../input/y2023/20");

        let modules = input.lines().map(Module::from_str).map(|m| (m.id.to_string(), m)).collect();

        Day20 { modules }
    }

    fn prime_conjunction_memories(&mut self) {
        let cloned = self.modules.clone();
        for module in self.modules.values_mut().filter(|m| matches!(m.mode, ModuleType::Conjunction(_))) {
            let memory = cloned.values().filter(|m| m.outputs.contains(&module.id)).map(|m| (m.id.clone(), false)).collect();
            module.mode = ModuleType::Conjunction(memory);
        }
    }

    fn push_button_x(&mut self, x: usize) -> (usize, usize) {
        let mut lows = 0;
        let mut highs = 0;
        for _ in 0..x {
            let (l, h, _) = self.push_button();
            lows += l;
            highs += h;
        }
        (lows, highs)
    }

    fn push_button_to_rx(&mut self) -> usize {
        let mut x = 0;
        loop {
            x += 1;
            let (_, _, sent_rx) = self.push_button();
            if sent_rx {
                return x;
            }
        }
    }

    fn push_button(&mut self) -> (usize, usize, bool) {
        let mut lows = 1;
        let mut highs = 0;
        let mut message_queue = VecDeque::new();
        let mut sent_rx = false;
        message_queue.push_back(Message { sender: "button".to_string(), target: "broadcaster".to_string(), pulse: false });
        while let Some(message) = message_queue.pop_front() {
            if let Some(target) = self.modules.get_mut(&message.target) {
                for m in target.handle_pulse(message.sender, message.pulse) {
                    if m.pulse {
                        highs += 1;
                    } else {
                        lows += 1;
                        if m.target.eq("rx") {
                            sent_rx = true;
                        }
                    }

                    message_queue.push_back(m);
                }
            }
        }
        (lows, highs, sent_rx)
    }
}

impl Day for Day20 {
    fn day_name(&self) -> String { String::from("20") }
    fn answer1(&self) -> String { String::from("731517480") }
    fn answer2(&self) -> String { String::from("??") }

    fn part1(&mut self) -> String {
        self.prime_conjunction_memories();
        return String::new();

        let (lows, highs) = self.push_button_x(1000);
        println!("{lows}, {highs}");
        (lows * highs).to_string()
    }

    fn part2(&mut self) -> String {
        self.push_button_to_rx().to_string()
    }
}

#[derive(Clone)]
enum ModuleType {
    Broadcast,
    FlipFlop(bool),
    Conjunction(HashMap<String, bool>),
}

#[derive(Clone)]
struct Module {
    id: String,
    mode: ModuleType,
    outputs: Vec<String>
}

#[derive(Debug)]
struct Message {
    sender: String,
    target: String,
    pulse: bool,
}

impl Module {
    fn from_str(input: &str) -> Module {
        let (left, right) = input.split_once(" -> ").unwrap();
        let mut left = left.chars();
        let (id, mode) = match left.next().unwrap() {
            'b' => ("broadcaster".to_string(), ModuleType::Broadcast),
            '%' => (left.collect::<String>(), ModuleType::FlipFlop(false)),
            '&' => (left.collect::<String>(), ModuleType::Conjunction(HashMap::new())),
            _ => panic!("Unknown Module Type"),
        };

        let outputs = right.split(", ").map(|s| s.to_string()).collect();

        Module { id, mode, outputs, }
    }

    fn handle_pulse(&mut self, sender: String, high: bool) -> Vec<Message> {
        let pulses = match &self.mode {
            ModuleType::Broadcast => self.handle_pulse_broadcast(high),
            ModuleType::FlipFlop(state) => self.handle_pulse_flip_flop(*state, high),
            ModuleType::Conjunction(memory) => self.handle_pulse_conjunction(sender, &mut memory.clone(), high),
        };

        pulses
    }

    fn handle_pulse_broadcast(&self, high: bool) -> Vec<Message> {
        self.messages(high)
    }

    fn handle_pulse_flip_flop(&mut self, state: bool, high: bool) -> Vec<Message> {
        if !high {
            let state = !state;
            self.mode = ModuleType::FlipFlop(state);
            self.messages(state)
        } else {
            vec![]
        }
    }

    fn handle_pulse_conjunction(&mut self, sender: String, memory: &mut HashMap<String, bool>, high: bool) -> Vec<Message> {
        memory.insert(sender, high);
        let pulse = !memory.values().all(|&b| b);
        self.mode = ModuleType::Conjunction(memory.clone());
        self.messages(pulse)
    }

    fn messages(&self, pulse: bool) -> Vec<Message> {
        self.outputs.iter().map(|target| 
            Message { 
                sender: self.id.to_string(),
                target: target.to_string(),
                pulse,
            }
        ).collect()
    }

}

impl Display for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{} -> {}", self.mode, self.id, self.outputs.iter().fold(String::new(), |acc, e| acc + &e + ", "))
    }
}

impl Display for ModuleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ModuleType::Broadcast => "B",
            ModuleType::FlipFlop(_) => "%",
            ModuleType::Conjunction(_) => "&",
        })
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} -> {}", self.sender, if self.pulse { "High" } else { "Low" }, self.target)
    }
}
