use std::{fs, fmt::Display, collections::HashMap};
use super::super::day::Day;

pub struct Day14 {
    instructions: Vec<Instruction>,
    mask: [BitMaskItem; 36],
    memory: HashMap<usize, usize>,
}

impl Day14 {
    pub fn new() -> Day14 {
        let input = fs::read_to_string("src/y2020/input14").expect("File Read Error");
    
        //let input = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\nmem[8] = 11\nmem[7] = 101\nmem[8] = 0";
        //let input = "mask = 000000000000000000000000000000X1001X\nmem[42] = 100\nmask = 00000000000000000000000000000000X0XX\nmem[26] = 1";

        let lines = input.trim().split('\n').map(|s| s.trim());

        let instructions = lines.map(Instruction::from_string).collect();

        Day14 { instructions, mask: [BitMaskItem::PassThrough; 36], memory: HashMap::new() }
    }

    fn memory_total(&self) -> usize {
        self.memory.values().sum()
    }
}

impl Day for Day14 {
    fn day_name(&self) -> String { String::from("14") }
    fn answer1(&self) -> String { String::from("14839536808842") }
    fn answer2(&self) -> String { String::from("4215284199669") }

    fn solve(&mut self) -> (String, String) {
        for i in &self.instructions {
            match i {
                Instruction::SetMask(mask) => {
                    self.mask = *mask;
                },
                Instruction::SetMem(addr, value) => {
                    self.memory.insert(*addr, apply_mask(*value, self.mask));
                }
            }
        }
        let part1 = self.memory_total();

        self.memory = HashMap::new();

        // Part 2
        for i in &self.instructions {
            match i {
                Instruction::SetMask(mask) => {
                    self.mask = *mask;
                },
                Instruction::SetMem(addr, value) => {
                    for address in calculate_addresses(*addr, self.mask) {
                        //println!("  Address: {}", address);
                        self.memory.insert(address, *value);
                    }
                }
            }
        }
        let part2 = self.memory_total();

        //println!("{:?}", (part1.to_string(), part2.to_string()));
        (part1.to_string(), part2.to_string())
    }
}

fn calculate_addresses(address: usize, mask: [BitMaskItem; 36]) -> Vec<usize> {
    let address_bits = format!("{:036b}", address);
    //println!("Address: {}", address_bits);
    let mut addresses = vec![address_bits];
    for (i, bit) in mask.iter().enumerate() {
        match bit {
            // What the
            BitMaskItem::One => {
                addresses.iter_mut().for_each(|address| address.replace_range(i..i+1, "1"));
            },
            BitMaskItem::Zero => {}
            BitMaskItem::PassThrough => {
                addresses = addresses.into_iter().flat_map(|mut address| {
                    // For each address, make sure that address has a one in this position, and add a new one with a zero
                    let mut zeroed = address.clone();
                    zeroed.replace_range(i..i+1, "0");
                    address.replace_range(i..i+1, "1");
                    vec![address.to_string(), zeroed.to_string()]
                }).collect();
            }
        }
    }
    addresses.iter()
        .map(|string| 
            usize::from_str_radix(string, 2)
            .unwrap()
        ).collect()
}

fn apply_mask(value: usize, mask: [BitMaskItem; 36]) -> usize {
    // This is definitely optimisation bait
    let value_bits = format!("{:036b}", value);

    let result_bits: String = mask.iter().zip(value_bits.chars()).map(|(mask_b, value_b)| {
        match mask_b {
            BitMaskItem::One => '1',
            BitMaskItem::Zero => '0',
            BitMaskItem::PassThrough => value_b,
        }
    }).collect();

    usize::from_str_radix(&result_bits, 2).expect("")
}

enum Instruction {
    SetMask([BitMaskItem; 36]),
    SetMem(usize, usize),
}

impl Instruction {
    fn from_string(input: &str) -> Instruction {
        let mut sides = input.split('=');
        let left = sides.next().expect("");
        let right = sides.next().expect("").trim();
        if left.eq("mask ") {
            Instruction::SetMask(BitMaskItem::bits_from_string(right))
        } else {
            let end_bracket_index = left.find(']').expect("");
            let address = left[4..end_bracket_index].parse::<usize>().expect("");
            
            let value = right.parse::<usize>().expect(""); 
            Instruction::SetMem(address, value)
        }
    }
}

#[derive(Clone, Copy)]
enum BitMaskItem {
    PassThrough,
    One,
    Zero,
}

impl BitMaskItem {
    fn bits_from_string(input: &str) -> [BitMaskItem; 36] {
        assert_eq!(input.len(), 36);
        let mut bits = [BitMaskItem::PassThrough; 36];
        for (i, char) in input.chars().enumerate() {
            match char {
                '0' => bits[i] = BitMaskItem::Zero,
                '1' => bits[i] = BitMaskItem::One,
                _ => (),
            }
        }
        bits
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SetMask(mask) => write!(f, "mask = {}", mask.map(|b| b.to_string()).join("")),
            Self::SetMem(addr, value) => write!(f, "mem[{}] = {}", addr, value),
        }
    }
}

impl Display for BitMaskItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::One => '1',
            Self::Zero => '0',
            Self::PassThrough => 'X',
        })
    }
}
