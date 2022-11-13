use std::collections::{HashSet};
use std::{fs, fmt::Display};
use super::super::day::Day;

pub struct Day7
{
    rules: Vec<Rule>,
}

impl Day7 {
    pub fn new() -> Day7
    {
        /* 
        */
        let input = fs::read_to_string("src/y2020/input7")
            .expect("File Read Error");

        //let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.\n dark orange bags contain 3 bright white bags, 4 muted yellow bags.\n bright white bags contain 1 shiny gold bag.\n muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\n shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\n dark olive bags contain 3 faded blue bags, 4 dotted black bags.\n vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\n faded blue bags contain no other bags.\n dotted black bags contain no other bags.";
        let lines = input.trim().split('\n').map(|s| s.trim());
        
        let rules = lines.flat_map(Rule::from_string).collect();
        
        Day7 { rules }
    }
}

impl Day for Day7 {
    fn day_name(&self) -> String { String::from("07") }
    fn answer1(&self) -> String { String::from("?") }
    fn answer2(&self) -> String { String::from("?") }

    fn solve(&mut self) -> (String, String) {
        let part1 = self.find_parents("shiny gold").len();
        let part2 = 0;
        
        println!("{:?}", part1);
        println!("{:?}", part2);
        (part1.to_string(), part2.to_string())
    }
}

impl Day7 {
    fn find_parents(&self, target: &str) -> HashSet<String> {
        let parents = self.rules
            .iter()
            .filter(|r| r.contains(target));
        let mut result = HashSet::new();
        for p in parents {
            result.insert(p.outer.to_string());
            let ancestors = self.find_parents(&p.outer);
            result.extend(ancestors);
        }
        result
    }
}

struct Rule {
    outer: String,
    inner: Option<String>,
    quantity: usize,
}

impl Rule {
    fn from_string(input: &str) -> Vec<Rule> {
        let mut words = input.split(' ');
        let outer: String = words.next().expect("Error").to_string() 
            + " " + words.next().expect("Error");

        let _bags = words.next();
        let _contain = words.next();

        let mut rules = vec![];

        println!("{}", input);
        // The rest of the iterator is any number of Quantity Desc Desc bag/s,
        while let Some(quantity_s) = words.next() {
            if quantity_s.eq("no") {
                let rule = Rule {outer: outer.trim().to_string(), inner: None, quantity: 0};
                let _other = words.next();
                let _bags = words.next();
                println!("    {}", rule);
                rules.push(rule);
            } else {
                let quantity = quantity_s.parse::<usize>().expect("Parse error");
                let inner: String = words.next().expect("Error").to_string() 
                    + " " + words.next().expect("Error");
                let _bag = words.next();
                let rule = Rule {outer: outer.trim().to_string(), inner: Some(inner.trim().to_string()), quantity};
                println!("    {}", rule);
                rules.push(rule);
            }
        }

        rules
    }

    fn contains(&self, target: &str) -> bool {
        if let Some(inner) = &self.inner {
            inner.eq(target)
        } else {
            false
        }
    }

}

impl Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // This feels slow, but Display is only for debugging
        write!(
            f, 
            "{} -> {} {:?}",
            self.outer,
            self.quantity,
            self.inner
        )
    }
}