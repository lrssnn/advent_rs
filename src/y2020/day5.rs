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

    fn solve(&self) -> (String, String) {
        let max_id = self.passes.iter().map(|p| p.get_id()).max().expect("No max?");
        let part1 = max_id.to_string();
        
        assert_eq!(892, max_id); // Just to draw attention to this hardcoded size
        let mut found = [false; 893];
        for p in self.passes.iter() {
            found[p.get_id()] = true;
        }
        
        let mut part2 = String::new();
        let not_found = found.iter().enumerate().filter(|pair| *pair.1 == false);
        for n in not_found {
            if n.0 == 0 { continue; }
            if found[n.0 -1] && found[n.0 + 1] {
                part2 = n.0.to_string();        
                break;
            } 
        }
        (part1.to_string(), part2.to_string())
    }
}

struct BoardingPass {
    row_indicators: [bool; 7],
    col_indicators: [bool; 3],
}

impl BoardingPass {
    fn from_string(input: &str) -> BoardingPass {
        let mut row_indicators = [false; 7];
        let mut col_indicators = [false; 3];
        
        let mut chars = input.chars();

        for i in 0..7 {
            let c = chars.next().expect("Line too short...");
            if c.eq(&'B') { row_indicators[i] = true; }
        }

        for j in 0..3 {
            let c = chars.next().expect("Line too short...");
            if c.eq(&'R') { col_indicators[j] = true; }
        }
        
        assert_eq!(0, chars.count());

        let b = BoardingPass {row_indicators, col_indicators};
        //println!("{}", b);
        b
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
    
    fn get_id(&self) -> usize {
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
            self.get_id()
        )
    }
}