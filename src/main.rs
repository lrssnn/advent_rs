#![feature(iter_next_chunk)]

mod y2022;
mod day;

use std::env;
use std::io::Write;
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
use y2022::day19::*;
use y2022::day20::*;

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
        Box::new(Day19::new()),
        Box::new(Day20::new()),
    ]
    } else {
        vec![
            Box::new(Day20::new()),
        ]
    };

    let construction_time = construction_start.elapsed();

    println!("Total construction time {}ms", construction_time.as_millis());

    println!("+-----+---------------+---------------+");
    println!("| Day |       1       |       2       |");
    println!("+-----+---------------+---------------+");
    let mut total_millis = Duration::from_secs(0);
    for mut day in days {
        print!("|  {} |", day.day_name());
        std::io::stdout().flush().unwrap();

        let start_1 = Instant::now();
        let ans1 = day.part1();
        let solve_time_1 = start_1.elapsed();
        total_millis += solve_time_1;
        let valid_1 = day.validate1(&ans1);
        print!(" {:10.3}: {} |", (solve_time_1.as_micros() as f32 / 1000.0), valid_1);
        std::io::stdout().flush().unwrap();

        let start_2 = Instant::now();
        let ans2 = day.part2();
        let solve_time_2 = start_2.elapsed();
        total_millis += solve_time_2;
        let valid_2 = day.validate2(&ans2);
        print!(" {:10.3}: {} |", (solve_time_2.as_micros() as f32 / 1000.0), valid_2);
        std::io::stdout().flush().unwrap();

        //if !full {
            print!("     1: {ans1}, 2: {ans2}");
        //}

        println!();
    }
    println!("+-----+---------------+---------------+");
    println!("  Total solve time {:.2} ms ({:.2} s)", (total_millis.as_micros() as f32 / 1000.0), (total_millis.as_millis() as f32 / 1000.0));
}
