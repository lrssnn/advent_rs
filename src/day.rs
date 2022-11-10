pub trait Day
{
    fn day_name(&self) -> String;
    fn answer1(&self) -> String;
    fn answer2(&self) -> String;

    fn solve(&mut self) -> (String, String);

    fn validate(&self, candidates: (String, String)) -> (char, char) {
        (if candidates.0 == self.answer1() {'✓'} else {'x'}, 
         if candidates.1 == self.answer2() {'✓'} else {'x'})
    }

    /*
    fn get_lines(path: &'static str) -> Vec<&str> {
        fs::read_to_string(path)
            .expect("File Read Error")
            .trim()
            .split('\n')
            .collect()
    } 
    */
}