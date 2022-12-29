#![feature(iter_next_chunk)]

mod y2022;
mod day;

use std::env;
use std::time::Instant;
use std::time::Duration;

use y2022::day1::*;
use y2022::day2::*;
use y2022::day3::*;
use y2022::day4::*;
use y2022::day5::*;
use y2022::day6::*;
use y2022::day7::*;
use y2022::day8::*;
use y2022::day9::*;
use y2022::day10::*;
use y2022::day11::*;
use y2022::day12::*;
use y2022::day13::*;
use y2022::day14::*;
use y2022::day15::*;
use y2022::day16::*;
use y2022::day17::*;
use y2022::day18::*;

use crate::day::Day as DayTrait;

fn main() {
    let args: Vec<String> = env::args().collect();
    for arg in &args {println!("'{arg}' {}", arg == "--full")};
    if args.iter().any(|arg| arg == "--full") {
        test(true);
    } else {
        test(false);
    }
}

fn test(full: bool)
{
    let construction_start = Instant::now();
    let days: Vec<Box<dyn DayTrait>> = if full {
        vec! [
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
        Box::new(Day13::new()),
        Box::new(Day14::new()),
        Box::new(Day15::new()),
        Box::new(Day16::new()),
        Box::new(Day17::new()),
        Box::new(Day18::new()),
    ]
    } else {
        vec![
            Box::new(Day18::new()),
        ]
    };

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
