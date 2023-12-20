use std::{collections::HashMap, fmt::Display};

use super::super::day::Day;

pub struct Day19
{
    workflows: HashMap<String, Workflow>,
    parts: Vec<Part>,
}

impl Day19 {
    pub fn new() -> Day19
    {
        let input = "px{a<2006:qkq,m>2090:A,rfg}\npv{a>1716:R,A}\nlnx{m>1548:A,A}\nrfg{s<537:gd,x>2440:R,A}\nqs{s>3448:A,lnx}\nqkq{x<1416:A,crn}\ncrn{x>2662:A,R}\nin{s<1351:px,qqz}\nqqz{s>2770:qs,m<1801:hdj,R}\ngd{a>3333:R,R}\nhdj{m>838:A,pv}\n\n{x=787,m=2655,a=1222,s=2876}\n{x=1679,m=44,a=2067,s=496}\n{x=2036,m=264,a=79,s=2244}\n{x=2461,m=1339,a=466,s=291}\n{x=2127,m=1623,a=2188,s=1013}";
        //let input = include_str!("../../input/y2023/19");

        let (workflows_s, parts_s) = input.split_once("\n\n").unwrap();

        let workflows = workflows_s.lines().map(Workflow::from_str).map(|w| (w.id.to_string(), w)).collect();
        let parts = parts_s.lines().map(Part::from_str).collect();

        Day19 { workflows, parts }
    }

    pub fn consolidate_terminal_rules(&mut self) -> bool {
        if self.workflows.len() == 1 { return false; }
        let mut made_changes = false;

        for terminal_workflow in self.workflows.clone().values().filter(|wf| wf.rules.iter().all(|r| r.destination.eq("R") || r.destination.eq("A"))) {
            // For each rule that points to this workflow... Replace that rule with a rule combining in each of the rules of this workflow
            self.workflows.remove(&terminal_workflow.id);
            for workflow in &mut self.workflows {
                for (i, rule) in &mut workflow.1.rules.clone().iter().enumerate() {
                    if rule.destination == terminal_workflow.id {
                        made_changes = true;
                        workflow.1.rules.remove(i);
                        let mut insertion_point = i;
                        for terminal_rule in &terminal_workflow.rules {
                            let combined_rule = rule.and(&terminal_rule);
                            workflow.1.rules.insert(insertion_point, combined_rule);
                            insertion_point += 1;
                        }
                    }
                }
            }
        }

        made_changes
    }

    pub fn consolidate_all_rules(&mut self) {
        while self.consolidate_terminal_rules() {}
    }
}

impl Day for Day19 {
    fn day_name(&self) -> String { String::from("19") }
    fn answer1(&self) -> String { String::from("333263") }
    fn answer2(&self) -> String { String::from("??") }

    fn part1(&mut self) -> String {
        self.consolidate_all_rules();
        self.workflows.get_mut("in").unwrap().consolidate_one_rule();
        self.parts.iter().filter(|p| is_accepted(p, &self.workflows)).map(|p| p.rating()).sum::<usize>().to_string()
    }

    fn part2(&mut self) -> String {
        self.workflows.values().map(|wf| wf.total_acceptance()).sum::<usize>().to_string()
    }
}

fn is_accepted(part: &Part, workflows: &HashMap<String, Workflow>) -> bool {
    let mut current_location = "in".to_string();
    loop {
        if current_location.eq("A") { 
            //println!("A");
            return true; 
        }
        if current_location.eq("R") { 
            //println!("R");
            return false; 
        }
        //print!("{current_location} -> ");

        let workflow = workflows.get(&current_location).unwrap();
        current_location = workflow.get_destination(&part);
    }
}

#[derive(Debug, Clone, Copy)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

#[derive(Debug, Clone)]
struct Workflow {
    id: String,
    rules: Vec<Rule>
}

#[derive(Debug, Clone)]
struct Rule {
    // quantity: RuleQuantity,
    // criteria: RuleCriteria,
    min_x: usize,
    max_x: usize,

    min_m: usize,
    max_m: usize,

    min_a: usize,
    max_a: usize,

    min_s: usize,
    max_s: usize,

    destination: String,
}

impl Workflow {
    fn from_str(input: &str) -> Workflow {
        let (id, rest) = input.split_once('{').unwrap();
        let rules = rest[..rest.len()-1].split(',').map(Rule::from_str).collect();

        Workflow {
            id: id.to_string(),
            rules,
        }
    }

    fn get_destination(&self, part: &Part) -> String {
        for rule in &self.rules {
            if rule.is_satisfied_by(part) {
                return rule.destination.clone();
            }
        }
        panic!("{part:?} Didn't satisfy any rules!\n{self:?}");
    }

    fn consolidate_rules(&mut self) {
        while self.consolidate_one_rule(){}
    }

    fn consolidate_one_rule(&mut self) -> bool {
        // Only consolidate consecutive rules going to the same place...
        println!("Consolidating one rule on {self}");
        for (i, window) in self.rules.clone().windows(2).enumerate() {
            if window[0].destination == window[1].destination {
                let new_rule = window[0].or(&window[1]);
                println!("{} OR {} = {}", window[0], window[1], new_rule);
                self.rules.remove(i);
                self.rules.remove(i + 1);
                self.rules.insert(i, new_rule);
                return true;
            }
        }
        false
    }

    fn total_acceptance(&self) -> usize {
        self.rules.iter().map(|rule| rule.acceptance()).sum()
    }
}

impl Rule {
    fn from_str(input: &str) -> Rule {
        let mut min_x = 0;
        let mut max_x = 4001;
        let mut min_m = 0;
        let mut max_m = 4001;
        let mut min_a = 0;
        let mut max_a = 4001;
        let mut min_s = 0;
        let mut max_s = 4001;

        if input.contains(':') {
            let (left, right) = input.split_once(':').unwrap();
            let mut chars = left.chars();

            let quantity = chars.next().unwrap();
            let criteria = chars.next().unwrap();

            let value = chars.collect::<String>().parse().unwrap();

            match (quantity, criteria) {
                ('x', '<') => max_x = value,
                ('x', '>') => min_x = value,
                ('m', '<') => max_m = value,
                ('m', '>') => min_m = value,
                ('a', '<') => max_a = value,
                ('a', '>') => min_a = value,
                ('s', '<') => max_s = value,
                ('s', '>') => min_s = value,
                _ => panic!("Invalid Rule"),
            };

            let destination = right.to_string();

            Rule { min_x, max_x, min_m, max_m, min_a, max_a, min_s, max_s, destination }
        } else {
            Rule { min_x, max_x, min_m, max_m, min_a, max_a, min_s, max_s, destination: input.to_string() }
        }
    }

    fn is_satisfied_by(&self, part: &Part) -> bool {
        self.min_x < part.x && part.x < self.max_x
        && self.min_m < part.m && part.m < self.max_m
        && self.min_a < part.a && part.a < self.max_a
        && self.min_s < part.s && part.s < self.max_s
    }

    fn and(&self, other: &Rule) -> Rule {
        let min_x = self.min_x.max(other.min_x);
        let max_x = self.max_x.min(other.max_x);

        let min_m = self.min_m.max(other.min_m);
        let max_m = self.max_m.min(other.max_m);

        let min_a = self.min_a.max(other.min_a);
        let max_a = self.max_a.min(other.max_a);

        let min_s = self.min_s.max(other.min_s);
        let max_s = self.max_s.min(other.max_s);
        Rule { min_x, max_x, min_m, max_m, min_a, max_a, min_s, max_s, destination: other.destination.to_string() }
    }

    fn or(&self, other: &Rule) -> Rule {
        let min_x = self.min_x.min(other.min_x);
        let max_x = self.max_x.max(other.max_x);

        let min_m = self.min_m.min(other.min_m);
        let max_m = self.max_m.max(other.max_m);

        let min_a = self.min_a.min(other.min_a);
        let max_a = self.max_a.max(other.max_a);

        let min_s = self.min_s.min(other.min_s);
        let max_s = self.max_s.max(other.max_s);
        Rule { min_x, max_x, min_m, max_m, min_a, max_a, min_s, max_s, destination: other.destination.to_string() }
    }

    fn acceptance(&self) -> usize {
        if self.destination.ne("A") { return 0; }
        let accepted_x = self.max_x - self.min_x - 1;
        let accepted_m = self.max_m - self.min_m - 1;
        let accepted_a = self.max_a - self.min_a - 1;
        let accepted_s = self.max_s - self.min_s - 1;
        let total = accepted_x * accepted_m * accepted_a * accepted_s;
        println!("{self} accepts {accepted_x} * {accepted_m} * {accepted_a} * {accepted_s}");
        total
    }
}

impl Part {
    fn from_str(input: &str) -> Part {
        let (x, rest) = input[3..(input.len() - 1)].split_once(',').unwrap();
        let (m, rest) = rest[2..].split_once(',').unwrap();
        let (a, rest) = rest[2..].split_once(',').unwrap();
        let s = &rest[2..];
        Part {
            x: x.parse().unwrap(),
            m: m.parse().unwrap(),
            a: a.parse().unwrap(),
            s: s.parse().unwrap(),
        }
    }

    fn rating(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

impl Display for Workflow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.id).unwrap();
        for rule in &self.rules {
            writeln!(f, "{rule}");
        }
        Ok(())
    }
}

impl Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:4}<x<{:4}, {:4}<m<{:4}, {:4}<a<{:4}, {:4}<s<{:4} => {}", 
            self.min_x, self.max_x, self.min_m, self.max_m, self.min_a, self.max_a, self.min_s, self.max_s,
            self.destination)
    }
}