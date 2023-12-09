use super::super::day::Day;

pub struct Day09
{
    dimensions: Vec<Vec<isize>>,
}

impl Day09 {
    pub fn new() -> Day09
    {
        //let input = "0 3 6 9 12 15\n1 3 6 10 15 21\n10 13 16 21 30 45";
        let input = include_str!("../../input/y2023/09");

        let dimensions = input.lines().map(|l| l.split_whitespace().map(|n| n.parse().unwrap()).collect()).collect();

        Day09 { dimensions }
    }
}

impl Day for Day09 {
    fn day_name(&self) -> String { String::from("09") }
    fn answer1(&self) -> String { String::from("2101499000") }
    fn answer2(&self) -> String { String::from("1089") }

    fn part1(&mut self) -> String {
        self.dimensions.iter()
            .map(|d| get_next(d))
            .sum::<isize>()
            .to_string()
    }

    fn part2(&mut self) -> String {
        self.dimensions.iter()
            .map(|d| get_prev(d))
            .sum::<isize>()
            .to_string()
    }
}

fn get_next(nums: &[isize]) -> isize {
    if nums.iter().all(|n| n.eq(&0)) {
        return 0;
    }

    nums[nums.len() - 1] + get_next(&get_diffs(nums))
}

fn get_prev(nums: &[isize]) -> isize {
    if nums.iter().all(|n| n.eq(&0)) {
        return 0;
    }

    nums[0] - get_prev(&get_diffs(nums))
}

fn get_diffs(nums: &[isize]) -> Vec<isize> {
    nums.windows(2).map(|w| w[1] - w[0]).collect()
}