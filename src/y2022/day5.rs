use super::super::day::Day;

pub struct Day5
{
    // Store a separate state for each part, ugly but *Shrug*
    ship_pt1: Vec<Stack>,
    ship_pt2: Vec<Stack>,
    steps: Vec<Step>,
}

impl Day5 {
    pub fn new() -> Day5
    {
        let input = include_str!("input5");
        //let input = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2";

        let mut parts = input.split("\n\n");

        let ship_lines = parts.next().expect("Malformed Input");
        let step_lines = parts.next().expect("Malformed Input").trim().split('\n');

        let ship = Stack::stacks_from_str(ship_lines);
        let steps = step_lines.map(Step::from_str).collect();
        Day5 { ship_pt1: ship.clone(), ship_pt2: ship, steps }
    }
}

impl Day for Day5 {
    fn day_name(&self) -> String { String::from("05") }
    fn answer1(&self) -> String { String::from("FWNSHLDNZ") }
    fn answer2(&self) -> String { String::from("RNRGDNFQG") }

    fn solve(&mut self) -> (String, String)
    {
        for step in &self.steps {
            apply_step(&mut self.ship_pt1, step);
            apply_step_chunked(&mut self.ship_pt2, step);
        }
        let ans1 = get_top_string(&self.ship_pt1);
        let ans2 = get_top_string(&self.ship_pt2);

        //println!("{}, {}", ans1, ans2);
        (ans1, ans2)
    }
}

fn apply_step(ship: &mut [Stack], step: &Step) {
    for _ in 0..step.repeats {
        // Shifting from 1 based labels to 0 based vec
        let item = ship[step.from-1].data.pop().expect("Shouldn't take from empty stack");
        ship[step.to-1].data.push(item);
    }
}

fn apply_step_chunked(ship: &mut [Stack], step: &Step) {
    // Shifting from 1 based labels to 0 based vec
    let mut tmp = vec![];
    for _ in 0..step.repeats {
        // Shifting from 1 based labels to 0 based vec
        let item = ship[step.from-1].data.pop().expect("Shouldn't take from empty stack");
        tmp.push(item);
    }
    tmp.reverse();
    ship[step.to-1].data.append(&mut tmp)
}

fn get_top_string(ship: &[Stack]) -> String {
    ship.iter().map(|s| s.data.last().expect("Should not be empty")).collect()
}

#[derive(Clone)]
struct Stack {
    data: Vec<char>,
}

impl Stack {
    fn stacks_from_str(input: &str) -> Vec<Stack> {
        let lines: Vec<Vec<char>> = input.split('\n').map(|line| line.chars().collect()).collect();
        let mut stacks = vec![];

        let mut col = 1;

        while col < lines[0].len() {
            let mut s = vec![];
            for row in (0..lines.len() - 1).rev() { // We walk up the stack so the top element will be first to pop()
                let c = lines[row][col];
                if c != ' ' { s.push(c) }
            }
            stacks.push(Stack { data: s });
            col += 4; // skip ] [
        }

        stacks
    }

}

struct Step {
    from: usize,
    to: usize,
    repeats: usize,
}

impl Step {
    fn from_str(input: &str) -> Step {
        // move REPEATS from FROM to TO
        //  0      1      2   3    4  5
        let parts = input.split(' ').collect::<Vec<_>>();
        Step {
            repeats: parts[1].parse::<usize>().expect("Invalid input"),
            from: parts[3].parse::<usize>().expect("Invalid input"),
            to: parts[5].parse::<usize>().expect("Invalid input"),
        }
    }
}