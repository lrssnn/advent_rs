use std::fmt::Display;

use super::super::day::Day;

pub struct Day21
{
    monkeys: Vec<Monkey>,
}

impl Day21 {
    pub fn new() -> Day21
    {
        let input = include_str!("input21");
        let input = include_str!("input21_example");

        let monkeys = input.trim().lines()
            .map(Monkey::from_str).collect::<Vec<_>>();

        Day21 { monkeys }
    }
}

impl Day for Day21 {
    fn day_name(&self) -> String { String::from("21") }
    fn answer1(&self) -> String { String::from("?") }
    fn answer2(&self) -> String { String::from("?") }

    fn part1(&mut self) -> String {
        for monkey in &self.monkeys {
            println!("{monkey}");
        }
        "unknown".to_string()
    }

    fn part2(&mut self) -> String {
        "unknown".to_string()
    }
}

impl Day21 {
}

struct Monkey {
    id: String,
    job: Job,
    value: Option<i32>,
}

enum Job {
    Number(i32),
    Operation(OperationType, String, String),
}

enum OperationType {
    Add,
    Sub,
    Mul,
    Div,
}

impl Monkey {
    fn from_str(input: &str) -> Monkey {
        let (id, job) = input.split_once(": ").unwrap();
        let id = id.to_string();
        let job = Job::from_str(job);
        Monkey { id, job, value: None }
    }
 }

impl Job {
    fn from_str(input: &str) -> Job {
        if !input.contains(' ') {
            Job::Number(input.parse().unwrap())
        } else {
            let parts = input.split(' ').collect::<Vec<_>>();
            let left = parts[0].to_string();
            let op = OperationType::from_str(parts[1]);
            let right = parts[2].to_string();
            Job::Operation(op, left, right)
        }
    }
}

impl OperationType {
    fn from_str(input: &str) -> OperationType {
        match input {
            "+" => OperationType::Add,
            "-" => OperationType::Sub,
            "*" => OperationType::Mul,
            "/" => OperationType::Div,
            _ => panic!("Invalid operation type"),            
        }
    }
}

impl Display for OperationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            OperationType::Add => "+",
            OperationType::Sub => "-",
            OperationType::Mul => "*",
            OperationType::Div => "/",
        })
    }
}

impl Display for Job {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Job::Number(x) => write!(f, "{x}"),
            Job::Operation(op, l, r) => write!(f, "{l} {op} {r}"),
        }
    }
}

impl Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {} ({:?})", self.id, self.job, self.value)
    }
}