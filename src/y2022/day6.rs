use super::super::day::Day;

pub struct Day6
{
    characters: Vec<char>,
}

impl Day6 {
    pub fn new() -> Day6
    {
        let input = include_str!("input6");
        //let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

        Day6 { characters: input.chars().collect() }
    }
}

impl Day for Day6 {
    fn day_name(&self) -> String { String::from("06") }
    fn answer1(&self) -> String { String::from("1566") }
    fn answer2(&self) -> String { String::from("2265") }

    fn part1(&mut self) -> String {
        self.find_start_marker().to_string()
    }

    fn part2(&mut self) -> String {
        self.find_message_marker().to_string()
    }
}

impl Day6 {
    fn find_start_marker(&self) -> usize {
        for i in 4..self.characters.len() {
            if 
                // I should be sent to prison
                self.characters[i - 4] != self.characters[i - 3]
                && self.characters[i - 4] != self.characters[i - 2]
                && self.characters[i - 4] != self.characters[i - 1]
                && self.characters[i - 4] != self.characters[i]
                && self.characters[i - 3] != self.characters[i - 2]
                && self.characters[i - 3] != self.characters[i - 1]
                && self.characters[i - 3] != self.characters[i]
                && self.characters[i - 2] != self.characters[i - 1]
                && self.characters[i - 2] != self.characters[i]
                && self.characters[i - 1] != self.characters[i] {
                    return i;
                }
        }

        panic!("Failed to find an answer!");
    }

    fn find_message_marker(&self) -> usize {
        for i in 14..self.characters.len() {
            if 
                // I should be sent to prison
                self.characters[i - 14] != self.characters[i - 13]
                && self.characters[i - 14] != self.characters[i - 12]
                && self.characters[i - 14] != self.characters[i - 11]
                && self.characters[i - 14] != self.characters[i - 10]
                && self.characters[i - 14] != self.characters[i - 9]
                && self.characters[i - 14] != self.characters[i - 8]
                && self.characters[i - 14] != self.characters[i - 7]
                && self.characters[i - 14] != self.characters[i - 6]
                && self.characters[i - 14] != self.characters[i - 5]
                && self.characters[i - 14] != self.characters[i - 4]
                && self.characters[i - 14] != self.characters[i - 3]
                && self.characters[i - 14] != self.characters[i - 2]
                && self.characters[i - 14] != self.characters[i - 1]
                && self.characters[i - 14] != self.characters[i]
                && self.characters[i - 13] != self.characters[i - 12]
                && self.characters[i - 13] != self.characters[i - 11]
                && self.characters[i - 13] != self.characters[i - 10]
                && self.characters[i - 13] != self.characters[i - 9]
                && self.characters[i - 13] != self.characters[i - 8]
                && self.characters[i - 13] != self.characters[i - 7]
                && self.characters[i - 13] != self.characters[i - 6]
                && self.characters[i - 13] != self.characters[i - 5]
                && self.characters[i - 13] != self.characters[i - 4]
                && self.characters[i - 13] != self.characters[i - 3]
                && self.characters[i - 13] != self.characters[i - 2]
                && self.characters[i - 13] != self.characters[i - 1]
                && self.characters[i - 13] != self.characters[i]
                && self.characters[i - 12] != self.characters[i - 11]
                && self.characters[i - 12] != self.characters[i - 10]
                && self.characters[i - 12] != self.characters[i - 9]
                && self.characters[i - 12] != self.characters[i - 8]
                && self.characters[i - 12] != self.characters[i - 7]
                && self.characters[i - 12] != self.characters[i - 6]
                && self.characters[i - 12] != self.characters[i - 5]
                && self.characters[i - 12] != self.characters[i - 4]
                && self.characters[i - 12] != self.characters[i - 3]
                && self.characters[i - 12] != self.characters[i - 2]
                && self.characters[i - 12] != self.characters[i - 1]
                && self.characters[i - 12] != self.characters[i]
                && self.characters[i - 11] != self.characters[i - 10]
                && self.characters[i - 11] != self.characters[i - 9]
                && self.characters[i - 11] != self.characters[i - 8]
                && self.characters[i - 11] != self.characters[i - 7]
                && self.characters[i - 11] != self.characters[i - 6]
                && self.characters[i - 11] != self.characters[i - 5]
                && self.characters[i - 11] != self.characters[i - 4]
                && self.characters[i - 11] != self.characters[i - 3]
                && self.characters[i - 11] != self.characters[i - 2]
                && self.characters[i - 11] != self.characters[i - 1]
                && self.characters[i - 11] != self.characters[i]
                && self.characters[i - 10] != self.characters[i - 9]
                && self.characters[i - 10] != self.characters[i - 8]
                && self.characters[i - 10] != self.characters[i - 7]
                && self.characters[i - 10] != self.characters[i - 6]
                && self.characters[i - 10] != self.characters[i - 5]
                && self.characters[i - 10] != self.characters[i - 4]
                && self.characters[i - 10] != self.characters[i - 3]
                && self.characters[i - 10] != self.characters[i - 2]
                && self.characters[i - 10] != self.characters[i - 1]
                && self.characters[i - 10] != self.characters[i]
                && self.characters[i - 9] != self.characters[i - 8]
                && self.characters[i - 9] != self.characters[i - 7]
                && self.characters[i - 9] != self.characters[i - 6]
                && self.characters[i - 9] != self.characters[i - 5]
                && self.characters[i - 9] != self.characters[i - 4]
                && self.characters[i - 9] != self.characters[i - 3]
                && self.characters[i - 9] != self.characters[i - 2]
                && self.characters[i - 9] != self.characters[i - 1]
                && self.characters[i - 9] != self.characters[i]
                && self.characters[i - 8] != self.characters[i - 7]
                && self.characters[i - 8] != self.characters[i - 6]
                && self.characters[i - 8] != self.characters[i - 5]
                && self.characters[i - 8] != self.characters[i - 4]
                && self.characters[i - 8] != self.characters[i - 3]
                && self.characters[i - 8] != self.characters[i - 2]
                && self.characters[i - 8] != self.characters[i - 1]
                && self.characters[i - 8] != self.characters[i]
                && self.characters[i - 7] != self.characters[i - 6]
                && self.characters[i - 7] != self.characters[i - 5]
                && self.characters[i - 7] != self.characters[i - 4]
                && self.characters[i - 7] != self.characters[i - 3]
                && self.characters[i - 7] != self.characters[i - 2]
                && self.characters[i - 7] != self.characters[i - 1]
                && self.characters[i - 7] != self.characters[i]
                && self.characters[i - 6] != self.characters[i - 5]
                && self.characters[i - 6] != self.characters[i - 4]
                && self.characters[i - 6] != self.characters[i - 3]
                && self.characters[i - 6] != self.characters[i - 2]
                && self.characters[i - 6] != self.characters[i - 1]
                && self.characters[i - 6] != self.characters[i]
                && self.characters[i - 5] != self.characters[i - 4]
                && self.characters[i - 5] != self.characters[i - 3]
                && self.characters[i - 5] != self.characters[i - 2]
                && self.characters[i - 5] != self.characters[i - 1]
                && self.characters[i - 5] != self.characters[i]
                && self.characters[i - 4] != self.characters[i - 3]
                && self.characters[i - 4] != self.characters[i - 2]
                && self.characters[i - 4] != self.characters[i - 1]
                && self.characters[i - 4] != self.characters[i]
                && self.characters[i - 3] != self.characters[i - 2]
                && self.characters[i - 3] != self.characters[i - 1]
                && self.characters[i - 3] != self.characters[i]
                && self.characters[i - 2] != self.characters[i - 1]
                && self.characters[i - 2] != self.characters[i]
                && self.characters[i - 1] != self.characters[i] {
                    return i;
                }
        }

        panic!("Failed to find an answer!");
    }
}
