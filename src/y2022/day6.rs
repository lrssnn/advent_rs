use super::super::day::Day;

mod tests {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn packet_marker_1() {
        let input: Vec<char> = "mjqjpqmgbljsphdztnvjfqwrcgsmlb".chars().collect();
        let result = Day6::find_packet_marker(&input);
        assert_eq!(7, result); 
    }

    #[test]
    fn packet_marker_2() {
        let input: Vec<char> = "bvwbjplbgvbhsrlpgdmjqwftvncz".chars().collect();
        let result = Day6::find_packet_marker(&input);
        assert_eq!(5, result); 
    }

    #[test]
    fn packet_marker_3() {
        let input: Vec<char> = "nppdvjthqldpwncqszvftbrmjlhg".chars().collect();
        let result = Day6::find_packet_marker(&input);
        assert_eq!(6, result); 
    }

    #[test]
    fn packet_marker_4() {
        let input: Vec<char> = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".chars().collect();
        let result = Day6::find_packet_marker(&input);
        assert_eq!(10, result); 
    }

    #[test]
    fn packet_marker_5() {
        let input: Vec<char> = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".chars().collect();
        let result = Day6::find_packet_marker(&input);
        assert_eq!(11, result); 
    }

    #[test]
    fn pt1() {
        let mut day = Day6::new();
        let result = day.part1();
        assert_eq!("1566", result); 
    }

    #[test]
    fn message_marker_1() {
        let input: Vec<char> = "mjqjpqmgbljsphdztnvjfqwrcgsmlb".chars().collect();
        let result = Day6::find_message_marker(&input);
        assert_eq!(19, result); 
    }

    #[test]
    fn message_marker_2() {
        let input: Vec<char> = "bvwbjplbgvbhsrlpgdmjqwftvncz".chars().collect();
        let result = Day6::find_message_marker(&input);
        assert_eq!(23, result); 
    }

    #[test]
    fn message_marker_3() {
        let input: Vec<char> = "nppdvjthqldpwncqszvftbrmjlhg".chars().collect();
        let result = Day6::find_message_marker(&input);
        assert_eq!(23, result); 
    }

    #[test]
    fn message_marker_4() {
        let input: Vec<char> = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".chars().collect();
        let result = Day6::find_message_marker(&input);
        assert_eq!(29, result); 
    }

    #[test]
    fn message_marker_5() {
        let input: Vec<char> = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".chars().collect();
        let result = Day6::find_message_marker(&input);
        assert_eq!(26, result); 
    }

    #[test]
    fn pt2() {
        let mut day = Day6::new();
        let result = day.part2();
        assert_eq!("2265", result); 
    }
}

pub struct Day6
{
    characters: Vec<char>,
}

impl Day6 {
    pub fn new() -> Day6 {
        Self::new_with_input(include_str!("input6"))
        //Self::new_with_input("mjqjpqmgbljsphdztnvjfqwrcgsmlb")
    }

    pub fn new_with_input(input: &str) -> Day6
    {
        Day6 { characters: input.chars().collect() }
    }
}

impl Day for Day6 {
    fn day_name(&self) -> String { String::from("06") }
    fn answer1(&self) -> String { String::from("1566") }
    fn answer2(&self) -> String { String::from("2265") }

    fn part1(&mut self) -> String {
        Self::find_packet_marker(&self.characters).to_string()
    }

    fn part2(&mut self) -> String {
        Self::find_message_marker(&self.characters).to_string()
    }
}

impl Day6 {
    pub fn find_packet_marker(data: &[char]) -> usize {
        Self::find_duplicate_window(data, 4)
    }

    pub fn find_message_marker(data: &[char]) -> usize {
        Self::find_duplicate_window(data, 14)
    }

    fn find_duplicate_window(data: &[char], window_size: usize) -> usize {
        data.windows(window_size).enumerate()
        .find(|(_index, window)| Self::contains_no_duplicates(window))
        .unwrap().0 + window_size
    }

    fn contains_no_duplicates(data: &[char]) -> bool {
        // Is this stupid?
        (0..data.len()).all(|i| ((i + 1)..(data.len())).all(|j| data[i] != data[j]))
    }
}