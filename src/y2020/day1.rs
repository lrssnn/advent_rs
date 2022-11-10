use std::fs;
use super::super::day::Day;

pub struct Day1
{
    nums: Vec<isize>,
}

impl Day1 {
    pub fn new() -> Day1
    {
        let nums = fs::read_to_string("src/y2020/input1")
            .expect("File Read Error")
            .trim()
            .split('\n')
            .map(|line| line.parse::<isize>().expect("Parse Error..."))
            .collect();
        Day1 {nums}
    }
}

impl Day for Day1 {
    fn day_name(&self) -> String { String::from("01") }
    fn answer1(&self) -> String { String::from("788739") }
    fn answer2(&self) -> String { String::from("178724430") }

    fn solve(&mut self) -> (String, String)
    {
        let length = self.nums.len();
        // Part 1 
        let mut ans1 = String::new();
        'outer: for i in 0..length {
            for j in i..length {
                let a = &self.nums[i];
                let b = &self.nums[j];
                if a + b == 2020 {
                    ans1 = (a * b).to_string();
                    break 'outer;
                }
            }
        }

        // Part 2 
        let mut ans2 = String::new();
        'outer2: for i in 0..length {
            for j in i..length {
                let a = &self.nums[i];
                let b = &self.nums[j];
                let partial = a + b;
                if partial >= 2020 {
                    continue;
                }

                for k in j..length {
                    let c = &self.nums[k];
                    if partial + c == 2020 {
                        ans2 = (a * b * c).to_string();
                        break 'outer2;
                    }
                }
            }
        }

        (ans1, ans2)
    }
}