use std::fmt::Display;

use super::super::day::Day;

pub struct Day15
{
    steps: Vec<Step>,
}

#[allow(dead_code)]
impl Day15 {
    pub fn new() -> Day15
    {
        //let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let input = include_str!("../../input/y2023/15");

        let steps = input.trim().split(',').map(Step::from_str).collect();

        Day15 { steps }
    }
}

impl Day for Day15 {
    fn day_name(&self) -> String { String::from("15") }
    fn answer1(&self) -> String { String::from("505459") }
    fn answer2(&self) -> String { String::from("228508") }

    fn part1(&mut self) -> String {
        self.steps.iter().map(|step| step.hash_code).sum::<usize>().to_string()
    }

    fn part2(&mut self) -> String {
        const EMPTY_VEC: Vec<Lens> = Vec::new();
        let mut boxes = [EMPTY_VEC; 256].to_vec();

        for step in &self.steps {
            match step.operation {
                Operation::Remove => remove_lens(&step.id, &mut boxes[step.label_hash]),
                Operation::Insert(focal_length) => add_lens(&step.id, focal_length, &mut boxes[step.label_hash]),
            }
        }

        boxes.iter().enumerate().flat_map(|(i, bx)| 
            bx.iter().enumerate().map(move |(slot, lens)| 
                (i + 1) * (slot + 1) * lens.focal_length as usize
            )
        ).sum::<usize>()
        .to_string()
    }
}


fn add_lens(target: &String, focal_length: u8, bx: &mut Vec<Lens>) {
    for lens in bx.iter_mut() {
        if lens.label.eq(target) {
            lens.focal_length = focal_length;
            return;
        }
    }
    bx.push(Lens::new(target.to_string(), focal_length));
}

fn remove_lens(target: &String, bx: &mut Vec<Lens>) {
    for (i, lens) in bx.iter().enumerate() {
        if lens.label.eq(target) {
            bx.remove(i);
            return;
        }
    }
}

fn _print_boxes(boxes: &[Vec<Lens>]) {
    for (i, b) in boxes.iter().enumerate() {
        if !b.is_empty() {
            print!("Box {i}: ");
            for lens in b {
                print!("{lens} ");
            }
            println!();
        }
    }
}
enum Operation {
    Remove,
    Insert(u8),
}

#[derive(Clone)]
struct Lens {
    label: String,
    focal_length: u8,
}

impl Lens {
    fn new(label: String, focal_length: u8) -> Self {
        Self { label, focal_length }
    }
}

struct Step {
    id: String,
    operation: Operation,
    hash_code: usize,
    label_hash: usize,
}

impl Step {
    fn from_str(input: &str) -> Self {
        let hash_code = Step::hash_code(input);
        let id;
        let operation;
        if let Some(_id) = input.strip_suffix('-') {
            id = _id.to_string();
            operation = Operation::Remove;
        } else {
            let (_id, focal_length) = input.split_once('=').unwrap();
            id = _id.to_string();
            operation = Operation::Insert(focal_length.parse::<u8>().unwrap());
        }

        let label_hash = Step::hash_code(&id);

        Self { hash_code, id, operation, label_hash }
    }

    fn hash_code(input: &str) -> usize {
        input.chars().fold(0, |hash, e| {
            ((hash + (e as u8) as usize) * 17) % 256
        })
    }
}

impl Display for Step {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.id, self.operation)
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Operation::Remove => "-".to_string(),
            Operation::Insert(focal_length) => "=".to_string() + &focal_length.to_string(),
        })
    }
}

impl Display for Lens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{} {}]", self.label, self.focal_length)
    }
}