use std::{fs, fmt::Display, collections::HashMap, ops::Range};
use super::super::day::Day;

pub struct Day16 {
    rules: Vec<Rule>,
    mine: Ticket,
    nearby: Vec<Ticket>,
}

impl Day16 {
    pub fn new() -> Day16 {
        let input = fs::read_to_string("src/y2020/input16").expect("File Read Error");
    
        //let input = "class: 1-3 or 5-7\nrow: 6-11 or 33-44\nseat: 13-40 or 45-50\n\nyour ticket:\n7,1,14\n\nnearby tickets:\n7,3,47\n40,4,50\n55,2,20\n38,6,12";
        //let input = "class: 0-1 or 4-19\nrow: 0-5 or 8-19\nseat: 0-13 or 16-19\n\nyour ticket:\n11,12,13\n\nnearby tickets:\n3,9,18\n15,1,5\n5,14,9";

        let mut lines = input.trim().split('\n').map(|s| s.trim());

        let mut rules = vec![];

        let mut current_line = lines.next().expect("Ran out of lines");
        while !current_line.is_empty() {
            rules.push(Rule::from_string(current_line));
            current_line = lines.next().expect("Ran out of lines");
        }

        
        // discard "your ticket: "
        let _ = lines.next().expect("Ran out of lines");

        current_line = lines.next().expect("Ran out of lines");
        let mine = Ticket::from_string(current_line);

        let mut nearby = vec![];

        // discard empty line and "nearby tickets: "
        let _ = lines.next().expect("Ran out of lines");
        let _ = lines.next().expect("Ran out of lines");
        while let Some(current_line) = lines.next() {
            nearby.push(Ticket::from_string(current_line));
        }

        Day16 { rules, mine, nearby }
    }

    fn populate_field_indicies(&mut self, tickets: &Vec<Ticket>) {
        // Maybe we can naively assign one field at a time
        while self.rules.iter().any(|r| r.field_index.is_none()) {
            println!("Another Loop");
            'index_loop: for i in 0..tickets.first().expect("Must be a field").values.len() {
                //println!("\nlooking at field index {}", i);
                'rule_loop: for mut rule in &mut self.rules {
                    if rule.field_index.is_some() { continue 'rule_loop; }
                    println!("  looking at rule {}", rule);
                    for ticket in tickets {
                        let value = ticket.values[i];
                        //println!("    v: {}", value);
                        if !rule.range_a.contains(&value) && !rule.range_b.contains(&value) {
                            //println!("    for ticket {}, {} violated rule", ticket, value);
                            continue 'rule_loop;
                        }
                        //println!("    v: {} satisfied", value);
                    }
                    rule.field_index = Some(i);
                    //println!("{}", rule);
                    continue 'index_loop;
                }
            }
        }
    }
}

impl Day for Day16 {
    fn day_name(&self) -> String { String::from("16") }
    fn answer1(&self) -> String { String::from("25961") }
    fn answer2(&self) -> String { String::from("?") }

    fn solve(&mut self) -> (String, String) {
        let mut part1 = 0;
        let mut valids = vec![];
        for ticket in &self.nearby {
            let invalids = ticket.invalid_for_all(&self.rules);
            if invalids.len() > 0 {
                for invalid in invalids {
                    part1 += invalid;
                }
            } else {
                valids.push(ticket.clone());
            }
        }

        //for ticket in &valids { println!("{}", ticket)}

        self.populate_field_indicies(&valids);

        for rule in &self.rules { println!("{}", rule)}

        let departure_rule_indices = self.rules.iter().filter_map(|r| {
            if r.name.starts_with("departure") {
                println!("{}", r);
                Some(r.field_index.expect("Didn't find index for rule"))
            } else {
                None
            }
        });

        let my_departure_values = departure_rule_indices.map(|i| self.mine.values[i]);

        let part2: usize = my_departure_values.product();

        println!("{:?}", (part1.to_string(), part2.to_string()));
        (part1.to_string(), part2.to_string())
    }
}

#[derive(Clone)]
struct Rule {
    name: String,
    range_a: Range<usize>,
    range_b: Range<usize>,
    field_index: Option<usize>,
}

impl Rule {
    fn from_string(input: &str) -> Rule {
        let mut parts = input.split(": ");
        let name = parts.next().expect("Malformed Line").to_string();
        let mut ranges = parts.next().expect("Malformed Line").split(" or ");

        let mut range_a_strs = ranges.next().expect("Malformed Line").split("-");
        let range_a_min = range_a_strs.next().expect("Malformed Line").parse::<usize>().expect("Parse Error");
        let range_a_max = range_a_strs.next().expect("Malformed Line").parse::<usize>().expect("Parse Error");
        let range_a = range_a_min..(range_a_max + 1);

        let mut range_b_strs = ranges.next().expect("Malformed Line").split("-");
        let range_b_min = range_b_strs.next().expect("Malformed Line").parse::<usize>().expect("Parse Error");
        let range_b_max = range_b_strs.next().expect("Malformed Line").parse::<usize>().expect("Parse Error");
        let range_b = range_b_min..(range_b_max + 1);

        Rule { name, range_a, range_b, field_index: None }
    }
}

impl Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {:?} or {:?}. Index: {}", self.name, self.range_a, self.range_b, if let Some(i) = self.field_index { i.to_string()} else { "Unknown".to_string()})
    }
}

#[derive(Clone)]
struct Ticket {
    values: Vec<usize>,
}

impl Ticket {
    fn from_string(input: &str) -> Ticket {
        let values = input.split(',').map(|s| s.parse::<usize>().expect("Parse Error")).collect();
        Ticket { values }
    }

    fn invalid_for_all(&self, rules: &Vec<Rule>) -> Vec<usize> {
        let mut res = vec![];
        'value_loop: for v in &self.values {
            for rule in rules {
                if rule.range_a.contains(v) || rule.range_b.contains(v) {
                    //println!("{} Satisfied {}", v, rule);
                    continue 'value_loop;
                }
            }
            res.push(*v);
        }

        res
    }
}

impl Display for Ticket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.values.iter()
            .map(|v| v.to_string())
            .reduce(|acc, v| acc + ","  + &v).unwrap()
        )
    }
}