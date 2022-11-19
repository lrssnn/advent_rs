use std::{fs, collections::HashMap};
use super::super::day::Day;

//const N: usize = 11 + 2;
const N: usize = 93 + 2;

pub struct Day10 {
    nums: [usize; N],
    ways_to_get_cache: HashMap<usize, usize>,
    cache_hits: usize,
    cache_misses: usize,
}

impl Day10 {
    pub fn new() -> Day10 {
        let input = fs::read_to_string("src/y2020/input10").expect("File Read Error");

        //let input = "16\n10\n15\n5\n1\n11\n7\n19\n6\n12\n4";

        let lines = input.trim().split('\n').map(|s| s.trim());
        
        let mut nums = [0; N];
        let mut max = 0;

        for (i, line) in lines.enumerate() {
            // i + 1 keeps zero in index 0 to represent the seat
            nums[i + 1] = line.parse::<usize>().expect("Parse Error...");
            if nums[i + 1] > max { max = nums[i + 1]; }
        }
        nums[N - 1] = max + 3;

        nums.sort();
        
        Day10 { nums, ways_to_get_cache: HashMap::new(), cache_hits: 0, cache_misses: 0 }
    }

    fn calculate_ratio(&self) -> usize {
        let mut ones = 0;
        let mut threes = 0;

        for i in 1..N {
            let difference = self.nums[i] - self.nums[i-1];
            //println!("{}({}) - {}({}) = {}", i, self.nums[i], i-1, self.nums[i-1], difference);
            if difference == 1 { ones += 1; }
            if difference == 3 { threes += 1; }
        }
        ones * threes
    }

    fn calculate_permutations(&mut self) -> usize {
        self.ways_to_get_to(N-1)
    }

    fn ways_to_get_to(&mut self, target_i: usize) -> usize {
        if self.ways_to_get_cache.contains_key(&target_i) {
            self.cache_hits += 1;
            return *self.ways_to_get_cache.get(&target_i).expect("Contains Key Error");
        }

        let target = self.nums[target_i];
        //println!("ways_to_get_to {}({})...", target_i, target);
        if target == 0 { return 1; }

        // we need to look back in the array until finding one too far away
        let mut total = 0;
        let mut i = target_i - 1;
        loop {
            let candidate = self.nums[i];
            if target - candidate > 3 { break; }
            //println!("  {} can get to {}", candidate, target);
            total += self.ways_to_get_to(i);
            
            if i == 0 { break; }
            else { i -= 1; }

        }

        //println!(" {} ways_to_get_to {}({})", total, target_i, target);
        self.cache_misses += 1;
        self.ways_to_get_cache.insert(target_i, total);
        total
    }

}

impl Day for Day10 {
    fn day_name(&self) -> String { String::from("10") }
    fn answer1(&self) -> String { String::from("1885") }
    fn answer2(&self) -> String { String::from("2024782584832") }

    fn solve(&mut self) -> (String, String) {
        let part1 = self.calculate_ratio();
        let part2 = self.calculate_permutations();
        
        (part1.to_string(), part2.to_string())
    }
}

