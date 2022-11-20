mod y2020;
mod day;

use std::time::Instant;
use std::time::Duration;

use y2020::day1::*;
use y2020::day2::*;
use y2020::day3::*;
use y2020::day4::*;
use y2020::day5::*;
use y2020::day6::*;
use y2020::day7::*;
use y2020::day8::*;
use y2020::day9::*;
use y2020::day10::*;
use y2020::day11::*;
use y2020::day12::*;

use crate::day::Day as DayTrait;

fn main() {
    /*
    let day = new Day8();
    day.Solve();
    println!("{day.Valid1} | {day.Valid2}");
    */
    full_test();
    /*
    let day = Day1::new();
    let (ans1, ans2) = day.solve();
    println!("Answer 1 '{}' | Answer 2 '{}'", ans1, ans2);
    */
}

fn full_test()
{
    let construction_start = Instant::now();
    let days: Vec<Box<dyn DayTrait>> = vec! [
        Box::new(Day1::new()),
        Box::new(Day2::new()),
        Box::new(Day3::new()),
        Box::new(Day4::new()),
        Box::new(Day5::new()),
        Box::new(Day6::new()),
        Box::new(Day7::new()),
        Box::new(Day8::new()),
        Box::new(Day9::new()),
        Box::new(Day10::new()),
        Box::new(Day11::new()),
        Box::new(Day12::new()),
    ];

    let construction_time = construction_start.elapsed();

    println!("Total construction time {}ms", construction_time.as_millis());

    println!("+-----+---+---+-----------------+");
    println!("| Day | 1 | 2 | Solve Time (ms) |");
    println!("+-----+---+---+-----------------+");
    let mut total_millis = Duration::from_secs(0);
    for mut day in days {
        let start = Instant::now();
        let answers = day.solve();
        let solve_time = start.elapsed();
        total_millis += solve_time;
        let valids = day.validate(answers);
        println!("|  {} | {} | {} | {:15} |", day.day_name(), valids.0, valids.1, (solve_time.as_micros() as f32 / 1000.0));
    }
    println!("+-----+---+---+-----------------+");
    println!("  Total solve time {} ms", (total_millis.as_micros() as f32 / 1000.0));
}
