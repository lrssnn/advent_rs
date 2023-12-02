use std::fmt::Display;

use super::super::day::Day;

pub struct Day02
{
    games: Vec<Game>,
}

impl Day02 {
    pub fn new() -> Day02
    {
        //let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let input = include_str!("../../input/y2023/02");

        let games = input.trim().lines().map(Game::from_str).collect();

        Day02 { games }
    }
}

impl Day for Day02 {
    fn day_name(&self) -> String { String::from("02") }
    fn answer1(&self) -> String { String::from("2101") }
    fn answer2(&self) -> String { String::from("58269") }

    fn part1(&mut self) -> String {
        self.games.iter().filter(|g| g.is_possible()).map(|g| g.id).sum::<u16>().to_string()
    }

    fn part2(&mut self) -> String {
        self.games.iter().map(|g| power(g.minimal_cubes())).sum::<u16>().to_string()
    }
}

fn power((red, green, blue): (u16, u16, u16)) -> u16 {
    red * green * blue
}

#[derive(Default)]
struct Reveal {
    red: u16,
    green: u16,
    blue: u16,
}

struct Game {
    id: u16,
    reveals: Vec<Reveal>,
}

impl Game {
    fn from_str(input: &str) -> Self {
        let (id_s, rest) = input.split_once(": ").unwrap();
        let id = id_s[5..].parse::<u16>().unwrap();
        
        let reveals = rest.split("; ").map(Reveal::from_str).collect();

        Self { id, reveals }
    }

    fn is_possible(&self) -> bool {
        self.reveals.iter().all(|g| g.is_possible())
    }

    fn minimal_cubes(&self) -> (u16, u16, u16) {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        for r in &self.reveals {
            max_red = max_red.max(r.red);
            max_green = max_green.max(r.green);
            max_blue = max_blue.max(r.blue);
        }

        (max_red, max_green, max_blue)
    }
}

impl Reveal {
    fn from_str(input: &str) -> Self {
        let items = input.split(", ");
        let mut result = Self::default(); 

        for item in items {
            let (value_s, colour) = item.split_once(' ').unwrap();
            let value = value_s.parse::<u16>().unwrap();
            match colour {
                "red" => { result.red = value; },
                "green" => { result.green = value; },
                "blue" => { result.blue = value; },
                _ => panic!(),
            }
        }

        result
    }

    fn is_possible(&self) -> bool {
        self.red <= 12 && self.blue <= 14 && self.green <= 13
    }
}

impl Display for Reveal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} red, {} blue, {} green", self.red, self.blue, self.green)
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Game {}: {} (possible: {})", 
            self.id, 
            self.reveals.iter().map(|g| g.to_string() + "; ").reduce(|acc, r| acc + &r).unwrap(),
            self.is_possible()
        )
    }
}