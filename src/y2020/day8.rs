use std::{fs, fmt::Display};
use super::super::day::Day;

pub struct Day8
{
    instructions: Vec<Instruction>,
    iptr: usize,
    acc: isize,
}

impl Day8 {
    pub fn new() -> Day8
    {
        let input = fs::read_to_string("src/y2020/input8")
            .expect("File Read Error");

        let input = "nop +0\nacc +1\njmp +4\nacc +3\njmp -3\nacc -99\nacc +1\njmp -4\nacc +6";

        let lines = input.trim().split('\n').map(|s| s.trim());
        
        let instructions = lines.map(Instruction::from_string).collect();
        
        Day8 { instructions, iptr: 0, acc: 0 }
    }
}

impl Day for Day8 {
    fn day_name(&self) -> String { String::from("08") }
    fn answer1(&self) -> String { String::from("1134") }
    fn answer2(&self) -> String { String::from("?") }

    fn solve(&mut self) -> (String, String) {
        let part1 = self.find_loop_value();
        let part2 = 0;
        
        println!("{}, {}", part1, part2);
        (part1.to_string(), part2.to_string())
    }
}

impl Day8 {
    fn find_loop_value(&mut self) -> isize {
        loop {
            if self.instructions[self.iptr].seen {
                return self.acc;
            }

            self.tick()
        }
    }

    fn tick(&mut self) {
        let mut inst = &mut self.instructions[self.iptr];
        println!("Processing {}", inst);
        inst.seen = true;

        match inst.op {
            Op::Nop(_) => { self.iptr += 1},
            Op::Acc(v) => { self.acc += v; self.iptr += 1},
            Op::Jmp(v) => { self.iptr = (self.iptr as isize + v) as usize },
        }
    }
}

struct Instruction {
    op: Op,
    seen: bool,
}

impl Instruction {
    fn from_string(input: &str) -> Instruction {
        let result = Instruction { op: Op::from_string(input), seen: false };
        println!("{}", result);
        result
    }
}

enum Op {
    Nop(isize),
    Acc(isize),
    Jmp(isize),
}

impl Op {
    fn from_string(input: &str) -> Op {
        let mut parts = input.split(' ');

        let result = match parts.next().expect("No Parts") {
            "nop" => Self::Nop(parts.next().expect("Missing second part").parse::<isize>().expect("Parse Error")),
            "acc" => Self::Acc(parts.next().expect("Missing second part").parse::<isize>().expect("Parse Error")),
            "jmp" => Self::Jmp(parts.next().expect("Missing second part").parse::<isize>().expect("Parse Error")),
            _ => panic!("invalid opcode"),
        };

        result
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!( f, "{} ({})", self.op, self.seen)
    }
}
impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!( f, "{}", match self {
            Self::Nop(v) => "nop".to_string() + &v.to_string(),
            Self::Acc(v) => "acc ".to_string() + &v.to_string(),
            Self::Jmp(v) => "jmp ".to_string() + &v.to_string()
            })
    }
}