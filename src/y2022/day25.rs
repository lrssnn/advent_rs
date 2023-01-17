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

        let numbers = input.lines().map(Ib5::from_str).collect::<Vec<_>>();

        Day25 { numbers }
    }
}

impl Day for Day25 {
    fn day_name(&self) -> String { String::from("25") }
    fn answer1(&self) -> String { String::from("122-12==0-01=00-0=02") }
    fn answer2(&self) -> String { String::from("free :)") }

    fn part1(&mut self) -> String {
        let sum = self.numbers.iter().map(|n| n.value()).sum::<i64>();
        Ib5::from_int(sum).to_string()
    }

    fn part2(&mut self) -> String {
        "free :)".to_string()
    }
}

struct Ib5 {
    digits: Vec<Digit>,
}

impl Ib5 {
    fn from_str(input: &str) -> Ib5 {
        let digits = input.chars().map(Digit::from_char).collect::<Vec<_>>();
        Ib5 { digits, }
    }

    fn from_int(input: i64) -> Ib5 {
        // First figure out how many digits we are going to need...
        let digits = Ib5::digits_needed(input);
        let mut result = vec![];
        
        // now start at 5^digits, and start taking chunks out of the original total.
        // We may go negative, so beware!
        let mut remaining = input;
        for digit in 0..digits {
            let place_value = 5_i64.pow(digits - digit - 1);
            // max_remaining is the biggest number we can represent with the digits after this one
            let max_remaining = Ib5::max_value(digits-digit-1);

            // How much do we need to remove from this value, to make 'remaining' acheivable?
            let this_digit = remaining - max_remaining;

            // Figure out which digit to assign;
            if this_digit > place_value { 
                result.push(Digit::Two); 
                remaining -= 2*place_value;
            }
            else if this_digit > 0 { 
                result.push(Digit::One); 
                remaining -= place_value;
            }
            else if this_digit > -place_value { 
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
        let places = self.digits.len() - 1;
        self.digits.iter().enumerate()
            .map(|(place, d)| d.value_placed((places-place) as u32))
            .sum()
    }

    fn digits_needed(input: i64) -> u32 {
        let mut digits = 1;
        let mut maximum = 2; // 2x5^0

        // This should probably be a lookup table with these limits, we'll see
        while maximum < input {
            // Add 1 digit, which contributes 2x5^n where n is the number of digits
            // to the total representable size
            maximum += 2 * 5_i64.pow(digits);
            digits += 1;
        }

        digits
    }

    fn max_value(digit: u32) -> i64 {
        // Geometric series
        2*(1 - 5_i64.pow(digit))/-4
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

    fn value_placed(&self, place: u32) -> i64 {
        self.value() as i64 * 5_i64.pow(place)
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