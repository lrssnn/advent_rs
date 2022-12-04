use super::super::day::Day;

pub struct Day1
{
    sums: Vec<usize>,
}

impl Day1 {
    pub fn new() -> Day1
    {
        let input = include_str!("input1");
        //let input = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";
        let sums = input.trim().split("\n\n") // split elves
            .map(|lines| lines
                .split('\n')
                .map(|line| line.trim().parse::<usize>().expect("Parse Error..."))
                .sum()
            ).collect();
        Day1 {sums}
    }
}

impl Day for Day1 {
    fn day_name(&self) -> String { String::from("01") }
    fn answer1(&self) -> String { String::from("72017") }
    fn answer2(&self) -> String { String::from("212520") }

    fn solve(&mut self) -> (String, String)
    {
        self.sums.sort();
        let len = self.sums.len();
        let max = self.sums[len-1];

        let ans1 = max;
        
        let top_three: usize = self.sums[len-3..].iter().sum();
        let ans2 = top_three;

        //println!("{}, {}", ans1, ans2);
        (ans1.to_string(), ans2.to_string())
    }
}