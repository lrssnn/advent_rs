use super::super::day::Day;
extern crate test;

mod tests {
    #![allow(unused_imports)]
    use super::*;
    use test::Bencher;

    #[test]
    fn packet_marker_1() {
        let input: Vec<char> = "mjqjpqmgbljsphdztnvjfqwrcgsmlb".chars().collect();
        let result = find_packet_marker(&input);
        assert_eq!(7, result); 
    }

    #[test]
    fn packet_marker_2() {
        let input: Vec<char> = "bvwbjplbgvbhsrlpgdmjqwftvncz".chars().collect();
        let result = find_packet_marker(&input);
        assert_eq!(5, result); 
    }

    #[test]
    fn packet_marker_3() {
        let input: Vec<char> = "nppdvjthqldpwncqszvftbrmjlhg".chars().collect();
        let result = find_packet_marker(&input);
        assert_eq!(6, result); 
    }

    #[test]
    fn packet_marker_4() {
        let input: Vec<char> = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".chars().collect();
        let result = find_packet_marker(&input);
        assert_eq!(10, result); 
    }

    #[test]
    fn packet_marker_5() {
        let input: Vec<char> = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".chars().collect();
        let result = find_packet_marker(&input);
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
        let result = find_message_marker(&input);
        assert_eq!(19, result); 
    }

    #[test]
    fn message_marker_2() {
        let input: Vec<char> = "bvwbjplbgvbhsrlpgdmjqwftvncz".chars().collect();
        let result = find_message_marker(&input);
        assert_eq!(23, result); 
    }

    #[test]
    fn message_marker_3() {
        let input: Vec<char> = "nppdvjthqldpwncqszvftbrmjlhg".chars().collect();
        let result = find_message_marker(&input);
        assert_eq!(23, result); 
    }

    #[test]
    fn message_marker_4() {
        let input: Vec<char> = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".chars().collect();
        let result = find_message_marker(&input);
        assert_eq!(29, result); 
    }

    #[test]
    fn message_marker_5() {
        let input: Vec<char> = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".chars().collect();
        let result = find_message_marker(&input);
        assert_eq!(26, result); 
    }

    #[test]
    fn pt2() {
        let mut day = Day6::new();
        let result = day.part2();
        assert_eq!("2265", result); 
    }

    #[bench]
    fn pt1_bench(b: &mut Bencher) {
        let input: Vec<char> = include_str!("input6").chars().collect();
        b.iter(|| find_packet_marker(&input));
    }

    #[bench]
    fn pt2_bench(b: &mut Bencher) {
        let input: Vec<char> = include_str!("input6").chars().collect();
        b.iter(|| find_message_marker(&input));
    }

}

pub struct Day6
{
    characters: Vec<char>,
}

impl Day6 {
    pub fn new() -> Day6 {
        Self::new_with_input(include_str!("input6"))
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
        find_packet_marker(&self.characters).to_string()
    }

    fn part2(&mut self) -> String {
        find_message_marker(&self.characters).to_string()
    }
}

fn find_packet_marker(data: &[char]) -> usize {
    find_duplicate_window(data, 4)
}

fn find_message_marker(data: &[char]) -> usize {
    find_duplicate_window(data, 14)
}

fn find_duplicate_window(data: &[char], window_size: usize) -> usize {
    for i in 0..(data.len() - window_size) {
        if contains_no_duplicates(&data[i..(i + window_size + 1)]) {
            return i + window_size;
        }
    }
    panic!("Failed to find an answer");
}

fn contains_no_duplicates(data: &[char]) -> bool {
    for i in 0..(data.len() - 1) {
        for j in (i + 1)..(data.len() - 1) {
            if data[i] == data[j] {
                return false;
            }
        }
    }
    true
}