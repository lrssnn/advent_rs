use std::fs;
use super::super::day::Day;

pub struct Day13 {
    earliest_time: usize,
    buses: Vec<Option<usize>>,
}

impl Day13 {
    pub fn new() -> Day13 {
        let input = fs::read_to_string("src/y2020/input13").expect("File Read Error");
    
        //let input = "939\n7,13,x,x,59,x,31,19";

        let mut lines = input.trim().split('\n').map(|s| s.trim());

        let earliest_time = lines.next().expect("").parse::<usize>().expect("");
        let buses = lines.next().expect("").split(',').map(|b| {
            if b == "x" { return None }
            Some(b.parse::<usize>().expect(""))
        }).collect();

        Day13 { earliest_time, buses }
    }

    fn find_part_1(&self) -> usize {
        let mut time = self.earliest_time;
        let mut bus: Option<usize> = None;
        while bus.is_none() {
            time += 1;
            bus = self.buses.iter().find_map(|b_opt| {
                if let Some(b) = b_opt {
                    if time % b == 0 { 
                        Some(*b)
                    } else { 
                        None
                    }
                } else {
                    None
                }
            });
        }
        let wait_time = time - self.earliest_time;
        wait_time * bus.unwrap()
    }

    fn find_part_2(&self) -> usize {

        let buses = self.buses.iter().filter_map(|b| {
            b.as_ref().copied()
        }).collect();

        let offsets = self.buses.iter().enumerate().filter_map(|(i, b)| {
            b.as_ref().map(|bus| *bus as isize - i as isize)
        }).collect();
        chinese_remainder(buses, offsets)
    }
}

// This and mul_inv ported from https://0xdf.gitlab.io/adventofcode2020/13
// Terrible naming but I don't understand enough to fix it lol. Crazy fast so idc
fn chinese_remainder(n: Vec<usize>, a: Vec<isize>) -> usize {
    let mut sum: isize = 0;
    let prod: usize = n.iter().product();
    for (n_i, a_i) in n.iter().zip(a.iter()) {
        let p = prod / n_i;
        sum += a_i * mul_inv(p as isize, *n_i as isize) * p as isize;
    }
    (sum % prod as isize) as usize
}

fn mul_inv(mut a: isize, mut b: isize) -> isize {
    if b == 1 { return 1; }
    let b0 = b;
    let (mut x0, mut x1) = (0, 1);
    while a > 1 {
        let q = a / b;
        (a, b) = (b, a % b);
        (x0, x1) = (x1 - q * x0, x0);
    }
    if x1 < 0 { x1 += b0; }
    x1
}

impl Day for Day13 {
    fn day_name(&self) -> String { String::from("13") }
    fn answer1(&self) -> String { String::from("2845") }
    fn answer2(&self) -> String { String::from("487905974205117") }

    fn solve(&mut self) -> (String, String) {
        let part1 = self.find_part_1();
        let part2 = self.find_part_2();

        //println!("{:?}", (part1.to_string(), part2.to_string()));
        (part1.to_string(), part2.to_string())
    }
}
