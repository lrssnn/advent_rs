use std::fmt::Display;

use super::super::day::Day;

pub struct Day06
{
    races: Vec<Race>,
    mega_race: Race,
}

impl Day06 {
    pub fn new() -> Day06
    {
        //let input = "Time:      7  15   30\nDistance:  9  40  200";
        let input = include_str!("../../input/y2023/06");

        Day06 { races: Race::from_str(input), mega_race: Race::from_str_single(input) }
    }
}

impl Day for Day06 {
    fn day_name(&self) -> String { String::from("06") }
    fn answer1(&self) -> String { String::from("1413720") }
    fn answer2(&self) -> String { String::from("30565288") }

    fn part1(&mut self) -> String {
        self.races.iter().map(Race::count_winning_options).product::<usize>().to_string()
    }

    fn part2(&mut self) -> String {
        self.mega_race.count_winning_options().to_string()
    }
}

struct Race {
    time: usize,
    distance: usize,
}

fn nums_from_str(input: &str) -> Vec<usize> {
    input.split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect::<Vec<_>>()
}

impl Race {
    fn from_str(input: &str) -> Vec<Race> {
        let (time_line, dist_line) = input.split_once('\n').unwrap();

        let times = nums_from_str(&time_line[5..]);
        let distances = nums_from_str(&dist_line[9..]);

        times.iter().zip(distances).map(|(&time, distance)| Race { time, distance }).collect()
    }

    fn from_str_single(input: &str) -> Race {
        let (time_line, dist_line) = input.split_once('\n').unwrap();
        let time = time_line[5..].split_whitespace().collect::<String>().parse::<usize>().unwrap();
        let distance = dist_line[9..].split_whitespace().collect::<String>().parse::<usize>().unwrap();

        Race { time, distance }
    }

    fn count_winning_options(&self) -> usize {
        // distance = charge * (time - charge)
        // solving that for charge (quadratic formula)
        // 0.5 * (time +- sqrt(time^2 - 4*distance))
        let sqrt_term = ((self.time.pow(2) - 4*self.distance) as f32).sqrt();
        let mut lower = (self.time as f32 - sqrt_term) / 2.0;
        let mut upper = (self.time as f32 + sqrt_term) / 2.0;

        // Exactly matching the distance is not considered enough, so this ensures that exact matches are rounded up
        // (ceil 20.0 = 20, we want 21)
        lower = lower.floor() + 1.0;

        // The same logic applies here, but we would subtract one from this then add it back to compensate for the 
        // The counting being inclusive of both ends
        upper = upper.ceil();

        (upper - lower) as usize
    }
}

impl Display for Race {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Time: {}, Dist: {}", self.time, self.distance)
    }
}
