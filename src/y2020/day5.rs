use std::{fs, fmt::Display};
use super::super::day::Day;

pub struct Day5
{
    passes: Vec<BoardingPass>,
}

impl Day5 {
    pub fn new() -> Day5
    {
        let input = fs::read_to_string("src/y2020/input5")
            .expect("File Read Error");
        
        let passes: Vec<BoardingPass> = input.trim().split('\n')
            .map(BoardingPass::from_string)
            .collect();

        Day5 { passes }
    }
}

impl Day for Day5 {
    fn day_name(&self) -> String { String::from("05") }
    fn answer1(&self) -> String { String::from("892") }
    fn answer2(&self) -> String { String::from("625") }

    fn solve(&mut self) -> (String, String) {
        let max_id = self.passes.iter_mut().map(|p| p.get_id()).max().expect("No max?");
        let part1 = max_id;
        
        assert_eq!(892, max_id); // Just to draw attention to this hardcoded size
        let mut found = [false; 893];
        for p in self.passes.iter_mut() {
            found[p.get_id()] = true;
        }
        
        let mut part2 = 0;
        let not_found = found.iter().enumerate().filter(|pair| !(*pair.1));
        for n in not_found {
            if n.0 == 0 { continue; }
            if found[n.0 -1] && found[n.0 + 1] {
                part2 = n.0;        
                break;
            } 
        }
        (part1.to_string(), part2.to_string())
    }
}

struct BoardingPass {
    row_indicators: [bool; 7],
    col_indicators: [bool; 3],
    id: Option<usize>,
}

impl BoardingPass {
    fn from_string(input: &str) -> BoardingPass {
        let mut row_indicators = [false; 7];
        let mut col_indicators = [false; 3];
        
        let mut chars = input.chars();

        for row_indicator in &mut row_indicators {
            let c = chars.next().expect("Line too short...");
            if c.eq(&'B') { *row_indicator = true; }
        }

        for col_indicator in &mut col_indicators {
            let c = chars.next().expect("Line too short...");
            if c.eq(&'R') { *col_indicator = true; }
        }
        
        assert_eq!(0, chars.count());

        BoardingPass {row_indicators, col_indicators, id: None}
    }
    
    fn get_row(&self) -> usize {
        let mut index = 0;
        if self.row_indicators[0] { index += 64; };
        if self.row_indicators[1] { index += 32; };
        if self.row_indicators[2] { index += 16; };
        if self.row_indicators[3] { index += 8; };
        if self.row_indicators[4] { index += 4; };
        if self.row_indicators[5] { index += 2; };
        if self.row_indicators[6] { index += 1; };
        index
    }

    fn get_col(&self) -> usize {
        let mut index = 0;
        if self.col_indicators[0] { index += 4; };
        if self.col_indicators[1] { index += 2; };
        if self.col_indicators[2] { index += 1; };
        index
    }
    
    fn get_id(&mut self) -> usize {
        if let Some(cached) = self.id {
            return cached;
        }
        let row = self.get_row();
        let col = self.get_col();
        let id = (row * 8) + col;
        self.id = Some(id);
        id
    }

    // Just the above without a caching step
    fn get_id_safe(&self) -> usize {
        if let Some(cached) = self.id {
            return cached;
        }
        let row = self.get_row();
        let col = self.get_col();
        (row * 8) + col
    }
}

impl Display for BoardingPass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // This feels slow, but Display is only for debugging
        write!(
            f, 
            "{} {}: ({}, {}) -> {}", 
            self.row_indicators.iter()
                .map(|b| if *b {"B"} else {"F"})
                .collect::<Vec<&str>>()
                .join(""),
            self.col_indicators.iter()
                .map(|b| if *b {"R"} else {"L"})
                .collect::<Vec<&str>>()
                .join(""),
            self.get_row(),
            self.get_col(),
            self.get_id_safe()
        )
    }
}