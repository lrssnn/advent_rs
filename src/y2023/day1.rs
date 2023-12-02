use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use super::super::day::Day;
pub struct Day1
{
    lines: Vec<String>,
}

impl Day1 {
    pub fn new() -> Day1
    {
        let input = include_str!("../../input/y2023/01");
        //let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet\n";
        //let input = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";

        let lines = input.lines().map(|l| l.to_string()).collect::<Vec<_>>();

        Day1 { lines }
    }
}

impl Day for Day1 {
    fn day_name(&self) -> String { String::from("01") }
    fn answer1(&self) -> String { String::from("55971") }
    fn answer2(&self) -> String { String::from("54719") }

    fn part1(&mut self) -> String {
        self.lines = self.lines.par_iter()
            .map(trim_between_digits).collect::<Vec<_>>();

        self.lines.par_iter()
            .map(|l| process_line(l, false))
            .sum::<u16>().to_string()
    }

    fn part2(&mut self) -> String {
        self.lines.par_iter()
            .map(|l| process_line(l, true))
            .sum::<u16>().to_string()
    }
}

fn trim_between_digits(line: &String) -> String {
    let first_digit = line.find(|c: char| c.is_ascii_digit()).unwrap();
    let mut last_digit = line.len() - 1;
    let chars = line.chars().collect::<Vec<_>>();
    while !chars[last_digit].is_ascii_digit() {
        last_digit -= 1;
    }

    // Remove the text between these two digits
    line[0..=first_digit].to_string() + &line[last_digit..line.len()]
}

fn process_line(line: &str, with_words: bool) -> u16 {
    let digits = into_digits(line, with_words);
    let first = digits[0];
    let last = digits[digits.len() - 1];
    (10 * first) + last
}

fn into_digits(l: &str, with_words: bool) -> Vec<u16> {
    let mut result = vec![];

    let digits = vec![
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ];

    for start in 0..l.len() {
        for (word, dig) in &digits {
            let working_string = &l[start..];
            if working_string.starts_with(*dig) || (with_words && working_string.starts_with(word)) {
                result.push((*dig as u16) - 48); // ASCII Zero
            }
        }
    }

    result
}
