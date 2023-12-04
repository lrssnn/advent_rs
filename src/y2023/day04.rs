use std::{fmt::Display, collections::HashMap};

use super::super::day::Day;

pub struct Day04
{
    cards: Vec<Card>,
}

impl Day04 {
    pub fn new() -> Day04
    {
        //let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\nCard 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\nCard 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\nCard 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\nCard 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\nCard 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let input = include_str!("../../input/y2023/04");

        let cards = input.lines().map(Card::from_str).collect();

        Day04 { cards }
    }
}

impl Day for Day04 {
    fn day_name(&self) -> String { String::from("04") }
    fn answer1(&self) -> String { String::from("18653") }
    fn answer2(&self) -> String { String::from("5921508") }

    fn part1(&mut self) -> String {
        self.cards.iter()
            .map(Card::score)
            .sum::<usize>()
            .to_string()
    }

    fn part2(&mut self) -> String {
        let mut cache = HashMap::new();
        self.cards.iter()
            .map(|c| c.evaluate(&self.cards, &mut cache))
            .sum::<usize>()
            .to_string()
    }
}

#[derive(Clone)]
struct Card {
    id: usize,
    winners: Vec<usize>,
    numbers: Vec<usize>,
}

impl Card {
    fn from_str(input: &str) -> Self {
        let (id, rest) = input.split_once(": ").unwrap();

        let id = id.split_whitespace().last().unwrap();

        let (winners, numbers) = rest.split_once(" | ").unwrap();

        let id = id.parse::<usize>().unwrap() - 1;

        let winners = winners.split_whitespace().map(|n| n.parse::<usize>().unwrap()).collect();
        let numbers = numbers.split_whitespace().map(|n| n.parse::<usize>().unwrap()).collect();
        
        Self { id, winners, numbers }
    }

    fn evaluate(&self, cards: &Vec<Card>, cache: &mut HashMap<usize, usize>) -> usize {
        if let Some(cached) = cache.get(&self.id) {
            return *cached;
        }

        let matches = self.matches() as usize;

        let mut score = 1;
        for i in (self.id + 1)..(self.id + matches + 1) {
            score += cards[i].evaluate(cards, cache);
        }

        cache.insert(self.id, score);
        score
    }

    fn matches(&self) -> u32 {
        self.numbers.iter().filter(|n| self.winners.contains(n)).count() as u32
    }

    fn score(&self) -> usize {
        let matches = self.matches();
        if matches == 0 {
            0
        } else {
            2usize.pow(matches - 1)
        }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Card {}: {} | {}",
            self.id,
            numbers_to_list(&self.winners),
            numbers_to_list(&self.numbers)
        )
    }
}

fn numbers_to_list(input: &[usize]) -> String {
    input.iter().fold(String::new(), |acc, n| acc + " " + &n.to_string())
}