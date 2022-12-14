use std::fmt::Display;

use super::super::day::Day;

pub struct Day13
{
    packets: Vec<Packet>,
}

impl Day13 {
    pub fn new() -> Day13
    {
        let input = include_str!("input13");
        //let input = include_str!("input13_example");

        let packets = input.trim().split('\n')
            .filter(|line| !line.is_empty())
            .map(Packet::from_str)
            // packet constructor wraps everything in an extra layer of list, just unwrap, it was giving me a headache to fix
            .map(|p| {
                match p {
                    Packet::Number(_) => panic!("Constructor behaviour changed!"),
                    Packet::List(inner) => inner.into_iter().next().expect("Constructor behaviour changed!")
                }
            })
            .collect::<Vec<_>>();

        Day13 { packets }
    }
}

impl Day for Day13 {
    fn day_name(&self) -> String { String::from("13") }
    fn answer1(&self) -> String { String::from("5938") }
    fn answer2(&self) -> String { String::from("29025") }

    fn solve(&mut self) -> (String, String)
    {

        let mut comparison = 1;
        let mut score = 0;
        for i in (0..self.packets.len()).step_by(2) {
            let left = &self.packets[i];
            let right = &self.packets[i + 1];
            // We expect top level to always be decisive...
            if Packet::are_ordered(left, right).unwrap() {
                score += comparison;
            }
            comparison += 1;
        }
        let ans1 = score;

        // Part 2 asks us to order the entire list to find the index of [[2]] and [[6]]
        // That's equivalent to counting how many items [[2]] and [[6]] greater than, and that's easier
        let target_a = Packet::List(vec![Packet::List(vec![Packet::Number(2)])]);
        let target_b = Packet::List(vec![Packet::List(vec![Packet::Number(6)])]);

        let mut less_than_a = 1; // This system is one-indexed so adjust for that
        let mut less_than_b = 2; // For b, we also count one more because 'a' is before it
        for packet in &self.packets {
            if Packet::are_ordered(packet, &target_a).unwrap() { less_than_a += 1 }
            if Packet::are_ordered(packet, &target_b).unwrap() { less_than_b += 1 }
        }
        let ans2 = less_than_a * less_than_b;

        //println!("{ans1}, {ans2}");
        (ans1.to_string() , ans2.to_string())
    }
}

impl Day13 {
}

#[derive(Clone)]
enum Packet {
    Number(usize),
    List(Vec<Packet>),
}

impl Packet {
    fn from_str(mut input: &str) -> Packet {
        //println!("newing '{input}'"); 
        let mut contents = vec![];

        // Handle empty list
        if input.is_empty() {
            return Packet::List(contents);
        }
        
        //let mut input = &input[1..input.len()-1];

        loop {
            //println!("  Input: {input}");
            if input == "]" {
                break;
            } else if input.starts_with('[') {
                // Handle list
                let chars = input[1..].chars().collect::<Vec<_>>();
                let end_index = Self::find_list_end(&chars);
                let substr = &chars[0..end_index].iter().collect::<String>();
                //println!("Identified sub-list: {substr}");
                let list = Packet::from_str(substr);
                contents.push(list);

                // skip to the next item, but be careful of the end of the entire thing...
                // + 3 skips the ],
                if end_index + 3 < input.len() {
                    input = &input[end_index+3..];
                } else {
                    break; // ?
                }
            } else {
                // Handle number
                if let Some((number, remainder)) = input.split_once(',') {
                    contents.push(Packet::Number(number.parse().expect("!")));
                    input = remainder
                } else {
                    // This is the last number in a list
                    contents.push(Packet::Number(input.parse().expect("!")));
                    break;
                }
            }
        }
        Packet::List(contents)
    }

    fn find_list_end(input: &[char]) -> usize {
        let mut ignore = 0;
        for (i, c) in input.iter().enumerate() {
            match c {
                '[' => ignore += 1,
                ']' => if ignore == 0 { return i; } else { ignore -= 1; }
                _ => (),
            }
        }
        panic!("List didn't end :0");
    }

    fn are_ordered(left: &Packet, right: &Packet) -> Option<bool> {
        //println!("compare : {left} vs {right}");

        match (left, right) {
            (Packet::Number(l), Packet::Number(r)) => {
                if l == r { return None; }
                Some(l < r)
            },
            (Packet::List(l), Packet::List(r)) => {
                let mut i = 0;
                loop {
                    if i == l.len() {
                        // Left ran out, if there's anything left in R, its correct
                        if i == r.len() { return None; }
                        return Some(true);
                    }
                    if i == r.len() {
                        // Right ran out, we know there's something in L, so its incorrect
                        return Some(false);
                    }
                    if let Some(validity) = Packet::are_ordered(&l[i], &r[i]) {
                        // Propagate decision up?
                        return Some(validity);
                    }
                    // We didn't get a decision, move on to the next item
                    i += 1;
                }
            },
            (Packet::List(_), Packet::Number(x)) => { Packet::are_ordered(left, &Packet::List(vec![Packet::Number(*x)])) }
            (Packet::Number(x), Packet::List(_)) => { Packet::are_ordered(&Packet::List(vec![Packet::Number(*x)]), right) }
        }

    }
}

impl Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Packet::Number(x) => write!(f, "{x}"),
            Packet::List(list) => {
                write!(f, "[").expect("!");
                for i in 0..list.len() { 
                    write!(f, "{}", list[i]).expect("!");
                    if i != list.len() - 1 { write!(f, ",").expect("!");}
                };
                write!(f, "]")
            }
        }
    }
}