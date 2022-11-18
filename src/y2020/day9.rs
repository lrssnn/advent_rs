use std::fs;
use super::super::day::Day;

const N: usize = 1000; // 20 for example
const WINDOW: usize = 25; // 5 for example

pub struct Day9 {
    nums: [usize; N],
}

impl Day9 {
    pub fn new() -> Day9 {
        let input = fs::read_to_string("src/y2020/input9").expect("File Read Error");

        //let input = "35\n20\n15\n25\n47\n40\n62\n55\n65\n95\n102\n117\n150\n182\n127\n219\n299\n277\n309\n576";

        let lines = input.trim().split('\n').map(|s| s.trim());

        let mut nums = [0; N];

        for (i, line) in lines.enumerate() {
            nums[i] = line.parse::<usize>().expect("Parse Error...");
        }
        
        Day9 { nums }
    }
}

impl Day for Day9 {
    fn day_name(&self) -> String { String::from("09") }
    fn answer1(&self) -> String { String::from("1639024365") }
    fn answer2(&self) -> String { String::from("219202240") }

    fn solve(&mut self) -> (String, String) {
        let part1 = self.find_invalid();
        let part2 = self.find_range_weakness(part1);
        
        (part1.to_string(), part2.to_string())
    }
}

impl Day9 {
    fn find_invalid(&self) -> usize {
        'inspection: for inspecting in WINDOW..N {
            //println!("Inspecting {}({})", inspecting, self.nums[inspecting]);
            for i in (inspecting - WINDOW)..inspecting {
                for j in (i + 1)..inspecting {
                    //println!("  {}({}) + {}({}) = {}", i, self.nums[i], j, self.nums[j], self.nums[i] + self.nums[j]);
                    if self.nums[i] + self.nums[j] == self.nums[inspecting] { continue 'inspection; }
                }
            }
            return self.nums[inspecting];
        }
        panic!("Didn't win");
    }

    fn find_range_weakness(&self, target_sum: usize) -> usize {
        'outer: for start in 0..N {
            let mut sum = 0;
            for end in start..N {
                sum += self.nums[end];
                if sum > target_sum {
                    continue 'outer;
                }
                if sum == target_sum {
                    return self.find_weakness(start, end);
                }
            }
        }
        panic!("Didn't win");
    }

    fn find_weakness(&self, start: usize, end: usize) -> usize {
        let mut smallest = usize::MAX;
        let mut largest = usize::MIN;
        for i in start..end {
            if self.nums[i] < smallest { smallest = self.nums[i]; }
            if self.nums[i] > largest { largest = self.nums[i]; }
        }
        smallest + largest
    }
}