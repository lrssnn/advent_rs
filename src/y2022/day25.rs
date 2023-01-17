use std::fmt::Display;

use super::super::day::Day;

pub struct Day25
{
    numbers: Vec<Ib5>,
}

impl Day25 {
    pub fn new() -> Day25
    {
        let input = include_str!("input25");
        //let input = include_str!("input25_example");

        let numbers = input.lines().map(|l| Ib5::from_str(l)).collect::<Vec<_>>();

        Day25 { numbers }
    }
}

impl Day for Day25 {
    fn day_name(&self) -> String { String::from("25") }
    fn answer1(&self) -> String { String::from("?") }
    fn answer2(&self) -> String { String::from("?") }

    fn part1(&mut self) -> String {
        let sum = self.numbers.iter().map(|n| n.value()).sum::<i64>();
        Ib5::from_int(sum as i64).to_string()
    }

    fn part2(&mut self) -> String {
        "unsolved".to_string()
    }
}

struct Ib5 {
    digits: Vec<Digit>,
}

impl Ib5 {
    fn from_str(input: &str) -> Ib5 {
        let digits = input.chars().map(|c| Digit::from_char(c)).collect::<Vec<_>>();
        Ib5 { digits, }
    }

    fn from_int(input: i64) -> Ib5 {
        // First figure out how many digits we are going to need...
        let digits = Ib5::digits_needed(input);
        let mut result = vec![];
        
        // now start at 5^digits, and start taking chunks out of the original total.
        // We may go negative, so beware!
        let mut remaining = input as i64;
        for digit in 0..digits {
            let place_value = 5_i64.pow((digits - digit - 1) as u32);
            // max_remaining is the biggest number we can represent with the digits after this one
            let max_remaining = Ib5::max_value(digits-digit-1) as i64;

            // How much do we need to remove from this value, to make 'remaining' acheivable?
            let this_digit = remaining - max_remaining;

            println!("{remaining} remaining. Looking at digit {digit} ({}), place_value: {place_value}, max_remaining: {max_remaining}, this: {this_digit}", digits - digit);
            // Figure out which digit to assign;
            if this_digit > place_value { 
                result.push(Digit::Two); 
                remaining -= 2*place_value;
            }
            else if this_digit > 0 { 
                result.push(Digit::One); 
                remaining -= place_value;
            }
            else if this_digit > -1 * place_value { 
                result.push(Digit::Zero); 
            }
            else if this_digit > -2 * place_value {
                result.push(Digit::Minus);
                remaining += place_value;
            }
            else { 
                result.push(Digit::DoubleMinus); 
                remaining += 2*place_value;
            }
        }

        Ib5{ digits: result }
    }

    fn value(&self) -> i64 {
        let places = self.digits.len() as i64 - 1;
        self.digits.iter().enumerate()
            .map(|(place, d)| d.value_placed(places-place as i64))
            .sum()
    }

    fn digits_needed(input: i64) -> i64 {
        let mut digits = 1;
        let mut maximum = 2; // 2x5^0

        // This should probably be a lookup table with these limits, we'll see
        while maximum < input {
            // Add 1 digit, which contributes 2x5^n where n is the number of digits
            // to the total representable size
            maximum += 2 * 5_i64.pow(digits);
            digits += 1;
        }

        digits as i64
    }

    fn max_value(digit: i64) -> i64 {
        // Geometric series
        (2*(1 - 5_i64.pow(digit as u32))/-4) as i64
    }
}

enum Digit {
    Two, One, Zero, Minus, DoubleMinus,
}

impl Digit {
    fn from_char(c: char) -> Digit {
        match c  {
            '2' => Digit::Two,
            '1' => Digit::One,
            '0' => Digit::Zero,
            '-' => Digit::Minus,
            '=' => Digit::DoubleMinus,
            _ => panic!("Unknown digit type")
        }
    }

    fn value(&self) -> i8 {
        match self {
            Digit::Two => 2,
            Digit::One => 1,
            Digit::Zero => 0,
            Digit::Minus => -1,
            Digit::DoubleMinus => -2,
        }
    }

    fn value_placed(&self, place: i64) -> i64 {
        self.value() as i64 * 5_i64.pow(place as u32)
    }
}

impl Display for Digit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Digit::Two => '2',
            Digit::One => '1',
            Digit::Zero => '0',
            Digit::Minus => '-',
            Digit::DoubleMinus => '=',
        })
    }
}

impl Display for Ib5 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for d in &self.digits {
            write!(f, "{d}").unwrap();
        }
        Ok(())
    }
}