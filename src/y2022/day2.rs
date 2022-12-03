use std::{fs, fmt::Display};
use super::super::day::Day;

pub struct Day2
{
    turns: Vec<Turn>,
}

impl Day2 {
    pub fn new() -> Day2
    {
        let input = fs::read_to_string("src/y2022/input2").expect("File Read Error");
        //let input = "A Y\nB X\nC Z";

        let turns = input.trim().split('\n')
            .map(Turn::from_str)
            .collect();
        Day2 {turns}
    }
}

impl Day for Day2 {
    fn day_name(&self) -> String { String::from("02") }
    fn answer1(&self) -> String { String::from("11767") }
    fn answer2(&self) -> String { String::from("13886") }

    fn solve(&mut self) -> (String, String)
    {
        let ans1: usize = self.turns.iter().map(|t| t.score_1()).sum();
        let ans2: usize = self.turns.iter().map(|t| t.score_2()).sum();

        //println!("{}, {}", ans1, ans2);
        (ans1.to_string(), ans2.to_string())
    }
}

struct Turn {
    opponent: Choice,
    // Part 1
    mine: Choice,
    // Part 2
    desired_result: Result,
}

impl Turn {
    fn from_str(input: &str) -> Turn {
        //println!("'{}'", input);
        let mut parts = input.split(' ');
        let opp_s = parts.next().expect("Malformed Line");
        let min_s = parts.next().expect("Malformed Line");

        Turn {
            opponent: Choice::from_str(opp_s),
            mine: Choice::from_str(min_s),
            desired_result: Result::from_str(min_s),
        }
    }

    fn score_1(&self) -> usize {
        self.result().score() + self.mine.score()
    }

    fn score_2(&self) -> usize {
        self.desired_result.score() + self.desired_play().score()
    }

    fn result(&self) -> Result {
        match (&self.mine, &self.opponent) {
            (Choice::Paper, Choice::Paper) => Result::Draw,
            (Choice::Paper, Choice::Scissors) => Result::Lose,
            (Choice::Paper, Choice::Rock) => Result::Win,
            (Choice::Scissors, Choice::Paper) => Result::Win,
            (Choice::Scissors, Choice::Scissors) => Result::Draw,
            (Choice::Scissors, Choice::Rock) => Result::Lose,
            (Choice::Rock, Choice::Paper) => Result::Lose,
            (Choice::Rock, Choice::Scissors) => Result::Win,
            (Choice::Rock, Choice::Rock) => Result::Draw,
        }
    }

    fn desired_play(&self) -> Choice {
        match (&self.opponent, &self.desired_result) {
            (Choice::Paper, Result::Lose) => Choice::Rock,
            (Choice::Paper, Result::Draw) => Choice::Paper,
            (Choice::Paper, Result::Win) => Choice::Scissors,
            (Choice::Scissors, Result::Lose) => Choice::Paper,
            (Choice::Scissors, Result::Draw) => Choice::Scissors,
            (Choice::Scissors, Result::Win) => Choice::Rock,
            (Choice::Rock, Result::Lose) => Choice::Scissors,
            (Choice::Rock, Result::Draw) => Choice::Rock,
            (Choice::Rock, Result::Win) => Choice::Paper,
        }
    }
}



enum Choice { Paper, Scissors, Rock, }
enum Result { Win, Lose, Draw }

impl Result {
    fn from_str(input: &str) -> Result {
        match input {
            "X" => Result::Lose,
            "Y" => Result::Draw,
            "Z" => Result::Win,
            _ => panic!("Invalid Result Input..."),
        }
    }

    fn score(&self) -> usize {
        match self {
            Result::Win => 6,
            Result::Draw => 3,
            Result::Lose => 0,
        }
    }
}

impl Choice {
    fn from_str(input: &str) -> Choice {
        match input {
            "A" | "X" => Choice::Rock,
            "B" | "Y" => Choice::Paper,
            "C" | "Z" => Choice::Scissors,
            _ => panic!("Invalid Choice Input..."),
        }
    }

    fn score(&self) -> usize {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }
}

impl Display for Turn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", 
            match self.opponent {
                Choice::Rock => "A",
                Choice::Paper => "B",
                Choice::Scissors => "C",
            },
            match self.mine {
                Choice::Rock => "X",
                Choice::Paper => "Y",
                Choice::Scissors => "Z",
            })
    }
}