use super::super::day::Day;

mod tests {
    use super::*;

    #[test]
    fn pt1_example_should_work() {
        let input = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";
        let mut day = Day1::new(input);
        let result = day.part1();
        assert_eq!("24000", result); 
    }

    #[test]
    fn pt2_example_should_work() {
        let input = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";
        let mut day = Day1::new(input);
        day.part1();
        let result = day.part2();
        assert_eq!("45000", result); 
    }

    #[test]
    fn pt1_should_work() {
        let input = include_str!("input1");
        let mut day = Day1::new(input);
        let result = day.part1();
        assert_eq!(day.answer1(), result);
    }

    #[test]
    fn pt2_should_work() {
        let input = include_str!("input1");
        let mut day = Day1::new(input);
        day.part1();
        let result = day.part2();
        assert_eq!(day.answer2(), result);
    }
}

pub struct Day1
{
    sums: Vec<usize>,
}

impl Day1 {
    pub fn new(input: &str) -> Day1
    {
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

    fn part1(&mut self) -> String
    {
        self.sums.sort();
        let len = self.sums.len();
        let max = self.sums[len-1];

        max.to_string()
    }
    
    fn part2(&mut self) -> String
    {
        let top_three: usize = self.sums[self.sums.len()-3..].iter().sum();
        top_three.to_string()
    }
}