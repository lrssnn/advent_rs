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

        //let input = "nop +0\nacc +1\njmp +4\nacc +3\njmp -3\nacc -99\nacc +1\njmp -4\nacc +6";

        let lines = input.trim().split('\n').map(|s| s.trim());
        
        let instructions = lines.map(Instruction::from_string).collect();
        
        Day8 { instructions, iptr: 0, acc: 0 }
    }
}

impl Day for Day8 {
    fn day_name(&self) -> String { String::from("08") }
    fn answer1(&self) -> String { String::from("1134") }
    fn answer2(&self) -> String { String::from("1205") }

    fn solve(&mut self) -> (String, String) {
        let (part1, _) = self.find_loop_value(None);
        self.reset();
        let part2 = self.find_terminating_program();
        
        (part1.to_string(), part2.to_string())
    }
}

impl Day8 {
    fn find_loop_value(&mut self, reversed_index: Option<usize>) -> (isize, bool) {
        loop {
            if self.iptr == self.instructions.len() {
                return (self.acc, true);
            }

            if self.instructions[self.iptr].seen {
                return (self.acc, false);
            }

            self.tick(reversed_index)
        }
    }

    fn find_terminating_program(&mut self) -> isize {
        for i in 0..self.instructions.len() -1 {
            let (v, terminated) = self.find_loop_value(Some(i));
            if terminated {
                return v;
            }
            self.reset();
        }
        panic!("I'm not even supposed to be here");
    }

    fn tick(&mut self, reversed_index: Option<usize>) {
        let mut inst = &mut self.instructions[self.iptr];
        inst.seen = true;

        if self.iptr == reversed_index.unwrap_or(usize::MAX) {
            match inst.op {
                Op::Jmp(_) => { self.iptr += 1},
                Op::Acc(v) => { self.acc += v; self.iptr += 1},
                Op::Nop(v) => { self.iptr = (self.iptr as isize + v) as usize },
            }
        } else {
            match inst.op {
                Op::Nop(_) => { self.iptr += 1},
                Op::Acc(v) => { self.acc += v; self.iptr += 1},
                Op::Jmp(v) => { self.iptr = (self.iptr as isize + v) as usize },
            }
        }
    }

    fn reset(&mut self) {
        for mut op in &mut self.instructions {
            op.seen = false;
        }
        self.iptr = 0;
        self.acc = 0;
    }
}

#[derive(Copy, Clone)]
struct Instruction {
    op: Op,
    seen: bool,
}

impl Instruction {
    fn from_string(input: &str) -> Instruction {
        Instruction { op: Op::from_string(input), seen: false }
    }
}

#[derive(Copy, Clone)]
enum Op {
    Nop(isize),
    Acc(isize),
    Jmp(isize),
}

impl Op {
    fn from_string(input: &str) -> Op {
        let mut parts = input.split(' ');

        match parts.next().expect("No Parts") {
            "nop" => Self::Nop(parts.next().expect("Missing second part").parse::<isize>().expect("Parse Error")),
            "acc" => Self::Acc(parts.next().expect("Missing second part").parse::<isize>().expect("Parse Error")),
            "jmp" => Self::Jmp(parts.next().expect("Missing second part").parse::<isize>().expect("Parse Error")),
            _ => panic!("invalid opcode"),
        }
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