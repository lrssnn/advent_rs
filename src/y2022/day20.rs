use super::super::day::Day;

const KEY: isize = 811589153;

pub struct Day20
{
    numbers: Vec<(usize,isize)>,
    numbers_2: Vec<(usize, isize)>,
}

impl Day20 {
    pub fn new() -> Day20
    {
        let input = include_str!("input20");
        //let input = "1\n2\n-3\n3\n-2\n0\n4";

        let numbers = input.trim().lines()
            .map(|line| line.parse().unwrap()).enumerate().collect::<Vec<_>>();

        let numbers_2 = numbers.clone();

        Day20 { numbers, numbers_2 }
    }
}

impl Day for Day20 {
    fn day_name(&self) -> String { String::from("20") }
    fn answer1(&self) -> String { String::from("2622") }
    fn answer2(&self) -> String { String::from("1538773034088") }

    fn part1(&mut self) -> String {
        self.get_coords(1).to_string()
    }

    fn part2(&mut self) -> String {
        // apply the key now. This is gross
        self.numbers = self.numbers_2.iter().map(|item| (item.0, item.1 * KEY)).collect();
        self.get_coords(10).to_string()
    }
}

impl Day20 {
    fn get_coords(&mut self, rounds: usize) -> isize {
        for _round in 0..rounds {
            self.mix();
        }

        let mixed = &self.numbers;
        let wrap = mixed.len();
        let zero_index = mixed.iter().position(|&i| i.1 == 0).unwrap();

        mixed[(zero_index + 1000) % wrap].1 +
        mixed[(zero_index + 2000) % wrap].1 +
        mixed[(zero_index + 3000) % wrap].1
    }

    fn mix(&mut self) {
        for original_index in 0..self.numbers.len() {
            let source = self.numbers.iter().position(|x| x.0 == original_index).unwrap();
            let num = self.numbers[source].1;
            let mut target = source as isize + num;
            // Stolen :)
            target = target.rem_euclid(self.numbers.len() as isize - 1);

            let val = self.numbers.remove(source);
            self.numbers.insert(target as usize, val);
        }
    }
}