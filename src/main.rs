#![feature(iter_next_chunk)]

mod y2022;
mod day;

use std::env;
use std::io::Write;
use std::time::Instant;
use std::time::Duration;

// This is disgusting
#[allow(unused_imports)]
use y2022::{day1::*, day2::*, day3::*, day4::*, day5::*, day6::*, day7::*, day8::*, day9::*, day10::*, day11::*, day12::*, day13::*, day14::*, 
    day15::*, day16::*, day17::*, day18::*, day19::*, day20::*, day21::*, day22::*, day23::*, day24::*, day25::*};

use crate::day::Day as DayTrait;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.iter().any(|arg| arg == "--full") {
        test(true, 1);
    } else {
        test(false, 1);
    }
}

fn test(full: bool, runs: u16) {
    let construction_start = Instant::now();
    let days: Vec<Box<dyn DayTrait>> = if full {
        vec! [
        //Box::new(Day1::new()),
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
        //Box::new(Day16::new()),
        Box::new(Day17::new()),
        Box::new(Day18::new()),
        Box::new(Day19::new()),
        Box::new(Day20::new()),
        Box::new(Day21::new()),
        Box::new(Day22::new()),
        Box::new(Day23::new()),
        Box::new(Day24::new()),
        Box::new(Day25::new()),
    ]
    } else {
        vec![
            Box::new(Day17::new()),
        ]
    };

    let construction_time = construction_start.elapsed();

    println!("Total construction time {}ms", construction_time.as_millis());

    println!("+-----+---------------+---------------+");
    println!("| Day |       1       |       2       |");
    println!("+-----+---------------+---------------+");
    let mut total_millis = Duration::from_secs(0);
    for mut day in days {
        for _run in 0..runs {
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
    }
    println!("+-----+---------------+---------------+");
    println!("  Total solve time {:.2} ms ({:.2} s)", (total_millis.as_micros() as f32 / 1000.0), (total_millis.as_millis() as f32 / 1000.0));
}
