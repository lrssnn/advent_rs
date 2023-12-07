use core::cmp::Ordering;
use std::{fmt::Display, collections::HashMap};

use super::super::day::Day;

const JOKER: u8 = 1;

pub struct Day07
{
    hands: Vec<Hand>,
}

impl Day07 {
    pub fn new() -> Day07
    {
        //let input = "32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483";
        let input = include_str!("../../input/y2023/07");

        let hands = input.lines().map(Hand::from_str).collect();

        Day07 { hands }
    }
}

impl Day for Day07 {
    fn day_name(&self) -> String { String::from("07") }
    fn answer1(&self) -> String { String::from("247961593") }
    fn answer2(&self) -> String { String::from("248750699") }

    fn part1(&mut self) -> String {
        self.hands.sort();
        self.hands.iter().enumerate().map(|(rank, hand)| hand.get_winnings(rank + 1)).sum::<usize>().to_string()
    }

    fn part2(&mut self) -> String {
        for hand in self.hands.iter_mut() {
            // Convert Jack value Joker value (11 -> 1)
            for i in 0..5 {
                if hand.cards[i] == 11 {
                    hand.cards[i] = 1;
                }
            }
            // Convert frequencies as well
            if let Some(j) = hand.frequencies.remove(&11) {
                hand.frequencies.insert(JOKER, j);
            }
        }

        self.hands.sort();

        let answer = self.hands.iter().enumerate().map(|(rank, hand)| hand.get_winnings(rank + 1)).sum::<usize>();
        answer.to_string()
    }
}

#[derive(PartialEq, Eq)]
struct Hand {
    cards: [u8; 5],
    bid: usize,
    frequencies: HashMap<u8, u8>,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        println!("Ord {self} vs {other}");
        self.partial_cmp(other).unwrap()
    }
}

impl Hand {
    fn from_str(input: &str) -> Self {
        let (cards, bid) = input.split_once(' ').unwrap();
        let cards = cards.chars().map(value_from_char).collect::<Vec<_>>().try_into().unwrap();
        let bid = bid.parse::<usize>().unwrap();
        let frequencies = Hand::card_frequencies(&cards);
        Self { cards, bid, frequencies }
    }

    fn card_frequencies(cards: &[u8; 5]) -> HashMap<u8, u8> {
        let mut res = HashMap::new();
        for card in cards {
            let f = res.get(card).unwrap_or(&0u8);
            res.insert(*card, f + 1);
        }

        res
    }

    fn get_winnings(&self, rank: usize) -> usize {
        self.bid * rank
    }

    fn get_hand_type(&self) -> HandType {
        if self.has_five() { HandType::FiveOfKind }
        else if self.has_four() { HandType::FourOfKind }
        else if self.has_full_house() { HandType::FullHouse }
        else if self.has_three() { HandType::ThreeOfKind }
        else if self.has_two_pair() { HandType::TwoPair }
        else if self.has_pair() { HandType::Pair }
        else { HandType::HighCard }
    }

    fn has_five(&self) -> bool {
        self.frequencies.len() == 1 ||
        self.frequencies.len() == 2 && self.frequencies.contains_key(&JOKER)
    }

    fn has_four(&self) -> bool {
        let jokers = self.frequencies.get(&JOKER).unwrap_or(&0);
        self.frequencies.iter().any(|(&k, &v)| k != JOKER && (v + jokers) == 4)
    }

    fn has_three(&self) -> bool {
        let jokers = self.frequencies.get(&JOKER).unwrap_or(&0);
        self.frequencies.iter().any(|(&k, &v)| k != JOKER && (v + jokers) == 3)
    }

    fn has_full_house(&self) -> bool {
        // We know we checked for a four of a kind already, this is sufficient
        let has_true_full_house = self.frequencies.len() == 2;
        let has_wild_full_house = self.frequencies.len() == 3 && self.frequencies.contains_key(&JOKER);

        has_true_full_house || has_wild_full_house
    }

    fn has_two_pair(&self) -> bool {
        // NO WILDCARDS - if you could create a two pair with a wild card, you would create a 
        // 3 of a kind (or better) instead, in all cases
        self.frequencies.values().filter(|&v| *v == 2).count() == 2
    }

    fn has_pair(&self) -> bool {
        let jokers = *self.frequencies.get(&JOKER).unwrap_or(&0);
        // Again, we know we don't have anything better
        self.frequencies.values().any(|v| *v == 2) ||
        jokers != 0
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.get_hand_type().partial_cmp(&other.get_hand_type()) {
            Some(Ordering::Equal) => {}
            ord => return ord,
        }
        match self.cards[0].partial_cmp(&other.cards[0]) {
            Some(Ordering::Equal) => {}
            ord => return ord,
        }

        match self.cards[1].partial_cmp(&other.cards[1]) {
            Some(Ordering::Equal) => {}
            ord => return ord,
        }

        match self.cards[2].partial_cmp(&other.cards[2]) {
            Some(Ordering::Equal) => {}
            ord => return ord,
        }

        match self.cards[3].partial_cmp(&other.cards[3]) {
            Some(Ordering::Equal) => {}
            ord => return ord,
        }

        match self.cards[4].partial_cmp(&other.cards[4]) {
            Some(Ordering::Equal) => {}
            ord => return ord,
        }
        panic!();
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

fn value_from_char(input: char) -> u8 {
    match input {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("Unrecognised char {input}"),
    }
}

fn str_from_value(input: u8) -> String {
    match input {
        1 => '*',
        2 => '2',
        3 => '3',
        4 => '4',
        5 => '5',
        6 => '6',
        7 => '7',
        8 => '8',
        9 => '9',
        10 => 'T',
        11 => 'J',
        12 => 'Q',
        13 => 'K',
        14 => 'A',
        _ => panic!("Unrecognised char {input}"),
    }.to_string()
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.cards.iter().fold(String::new(), |acc, c| acc + &str_from_value(*c)))
    }
}
