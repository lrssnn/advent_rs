pub trait Day
{
    fn day_name(&self) -> String;
    fn answer1(&self) -> String;
    fn answer2(&self) -> String;

    fn part1(&mut self) -> String;
    fn part2(&mut self) -> String;

    fn validate1(&self, candidate: &String) -> char {
        if candidate.eq(&self.answer1()) {'✓'} else {'x'} 
    }

    fn validate2(&self, candidate: &String) -> char {
        if candidate.eq(&self.answer2()) {'✓'} else {'x'} 
    }
}