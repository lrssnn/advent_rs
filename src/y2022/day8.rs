use super::super::day::Day;

//const WIDTH: usize = 5;
const WIDTH: usize = 99;

pub struct Day8
{
    trees: [[usize; WIDTH]; WIDTH],
}

impl Day8 {
    pub fn new() -> Day8
    {
        let input = include_str!("input8");
        //let input = "30373\n25512\n65332\n33549\n35390";

        let trees = input.lines().map(|line| 
            line.chars().map(|char| 
                char.to_digit(10).expect("Parse Error") as usize
            ).collect::<Vec<_>>().try_into().unwrap()
        ).collect::<Vec<_>>().try_into().unwrap();

        Day8 { trees }
    }
}

impl Day for Day8 {
    fn day_name(&self) -> String { String::from("08") }
    fn answer1(&self) -> String { String::from("1825") }
    fn answer2(&self) -> String { String::from("235200") }

    fn solve(&mut self) -> (String, String)
    {
        let (num_visible, best_score) = self.evaluate();
        let ans1 = num_visible;
        let ans2 = best_score;

        println!("{ans1}, {ans2}");
        (ans1.to_string() , ans2.to_string())
    }
}

impl Day8 {
    // (num_visible, best_scenic)
    fn evaluate(&self) -> (usize, usize) {
        let mut num_visible = 0;
        let mut best_scenic = 0;
        for y in 0..WIDTH {
            for x in 0..WIDTH {
                let (score, visible) = self.score(x, y);
                if visible { num_visible += 1; }
                if score > best_scenic { best_scenic = score; }
            }
        }
        (num_visible, best_scenic)
    }

    fn score(&self, x: usize, y: usize) -> (usize, bool) {
        let (left_score, left_visible) = self.scenic_score_left(x, y);
        let (right_score, right_visible) = self.scenic_score_right(x, y);
        let (up_score, up_visible) = self.scenic_score_up(x, y);
        let (down_score, down_visible) = self.scenic_score_down(x, y);

        (left_score * right_score * up_score * down_score,
         left_visible || right_visible || up_visible || down_visible)
    }

    fn scenic_score_left(&self, x: usize, y: usize) -> (usize, bool) {
        if x == 0 { return (0, true); }
        let us = self.trees[y][x];
        let mut x = x - 1;
        let mut score = 0;
        loop {
            score += 1;
            if self.trees[y][x] >= us { return (score, false) }
            if x == 0 { break; } else {x -= 1;}
        }
        (score, true)
    }

    fn scenic_score_right(&self, x: usize, y: usize) -> (usize, bool) {
        if x == WIDTH - 1 { return (0, true); }
        let us = self.trees[y][x];
        let mut x = x + 1;
        let mut score = 0;
        loop {
            score += 1;
            if self.trees[y][x] >= us { return (score, false); }
            if x == WIDTH - 1 { break; } else {x += 1;}
        }
        (score, true)
    }

    fn scenic_score_up(&self, x: usize, y: usize) -> (usize, bool) {
        if y == 0 { return (0, true); }
        let us = self.trees[y][x];
        let mut y = y - 1;
        let mut score = 0;
        loop {
            score += 1;
            if self.trees[y][x] >= us { return (score, false); }
            if y == 0 { break; } else {y -= 1;}
        }
        (score, true)
    }

    fn scenic_score_down(&self, x: usize, y: usize) -> (usize, bool) {
        if y == WIDTH - 1 { return (0, true); }
        let us = self.trees[y][x];
        let mut y = y + 1;
        let mut score = 0;
        loop {
            score += 1;
            if self.trees[y][x] >= us { return (score, false); }
            if y == WIDTH - 1 { break; } else {y += 1;}
        }
        (score, true)
    }
}
