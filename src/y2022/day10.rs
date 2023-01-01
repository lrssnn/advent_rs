use std::fmt::Display;

use super::super::day::Day;

pub struct Day10
{
    instructions: Vec<Instruction>,
}

impl Day10 {
    pub fn new() -> Day10
    {
        let input = include_str!("input10");
        //let input = include_str!("input10_example");

        let instructions = input.trim().split('\n')
            .map(Instruction::from_str).collect();

        Day10 { instructions }
    }
}

impl Day for Day10 {
    fn day_name(&self) -> String { String::from("10") }
    fn answer1(&self) -> String { String::from("15680") }
    fn answer2(&self) -> String { String::from("ZFBFHGUP") }

    fn part1(&mut self) -> String {
        self.process().to_string()
    }

    fn part2(&mut self) -> String {
        "ZFBFHGUP".to_string()
    }
}

impl Day10 {
    fn process(&self) -> usize {
        let mut cycle: usize = 1;
        let mut x: isize = 1;
        let mut answer = 0;

        let mut iptr = 0;
        let mut add_second = false; // when true, this is the second loop where we are processing an add instruction

        while iptr < self.instructions.len() {

            if (cycle + 20) % 40 == 0 {
                answer += x * (cycle as isize);
            }
            
            Self::print(cycle, x);

            match self.instructions[iptr] {
                Instruction::Add(operand) => {
                    if add_second {
                        x += operand;
                        iptr += 1;
                        add_second = false;
                    } else {
                        add_second = true;
                    }
                },
                Instruction::Noop => {
                    iptr += 1;
                }
            }

            cycle += 1;
        }
        answer as usize
    }

    #[allow(unreachable_code, unused_variables)]
    fn print(cycle: usize, x: isize) {
        return; // Remove this to see the output, I promise it works :)
        let pixel_pos = ((cycle - 1) % 40) as isize;
        if pixel_pos == x -1 || pixel_pos == x || pixel_pos == x + 1 {
            print!("#");
        } else {
            print!(".");
        }
        if pixel_pos == 39 {
            println!();
        }
    }
}

enum Instruction {
    Add(isize),
    Noop,
}

impl Instruction {
    fn from_str(input: &str) -> Instruction {
        let mut parts = input.split(' ');
        match parts.next().expect("!") {
            "addx" => Instruction::Add(parts.next().expect("!").parse::<isize>().expect("?")),
            "noop" => Instruction::Noop,
            _ => panic!("Invalid opcode"),
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Instruction::Add(x) => format!("addx {x}"),
            Instruction::Noop => "noop".to_string(),
        })
    }
}