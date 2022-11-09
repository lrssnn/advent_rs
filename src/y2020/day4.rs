use std::{fs, fmt::Display, collections::HashMap};
use regex::Regex;
use super::super::day::Day;

pub struct Day4
{
    passports: Vec<Passport>,
}

impl Day4 {
    pub fn new() -> Day4
    {
        let input = fs::read_to_string("src/y2020/input4")
            .expect("File Read Error");

        let passports: Vec<Passport> = input.split("\n\n").map(Passport::from_string).collect();

        //for p in passports.iter() { println!("---\n{}\n---", p) };
        //println!("---\n{}\n---", passports[0]);
        Day4 { passports }
    }
}

impl Day for Day4 {
    fn day_name(&self) -> String { String::from("04") }
    fn answer1(&self) -> String { String::from("222") }
    fn answer2(&self) -> String { String::from("140") }

    fn solve(&self) -> (String, String) {
        let complete_passports = self.passports.iter().filter(|p| p.is_complete());
        // This is insane, must be a borrow safe way to map both but count inbetween...
        let mut completes = 0;
        let mut valids = 0;
        for passport in complete_passports {
            completes += 1;
            if passport.is_valid() { valids += 1 }
        }
        let part1 = completes;
        let part2 = valids;
        (part1.to_string(), part2.to_string())
    }
}

struct Passport {
    fields: HashMap<String, String>,
    hair_re: Regex,
    pid_re: Regex,
}

impl Passport {
    fn from_string(input: &str) -> Passport {
        let mut fields = HashMap::new();
        // This string may have multiple lines
        for line in input.lines() {
            // Split on spaces to get pairs, then split each pair on the colon to get the key and value
            for kvp in line.split(' ') {
                let parts: Vec<&str> = kvp.split(':').collect();
                fields.insert(
                    parts[0].to_string(), 
                    parts[1].to_string());
            }
        }

        let hair_re = Regex::new("^#[a-f|0-9]{6}$").expect("Regex Error");
        let pid_re = Regex::new("^[0-9]{9}$").expect("Regex error...");

        Passport {fields, hair_re, pid_re}
    }

    fn is_complete(&self) -> bool {
        self.fields.contains_key("byr")
        && self.fields.contains_key("iyr")
        && self.fields.contains_key("eyr")
        && self.fields.contains_key("hgt")
        && self.fields.contains_key("hcl")
        && self.fields.contains_key("ecl")
        && self.fields.contains_key("pid")
    }

    fn is_valid(&self) -> bool {
        self.birth_year_valid()
        && self.issue_year_valid()
        && self.exp_year_valid()
        && self.height_valid()
        && self.hair_valid()
        && self.eye_valid()
        && self.pid_valid()

    }

    fn birth_year_valid(&self) -> bool {
        //byr (Birth Year) - four digits; at least 1920 and at most 2002.
        if let Ok(byr) = self.fields["byr"].parse::<usize>() {
            (1920..=2002).contains(&byr)
        } else {
            false
        }
    }

    fn issue_year_valid(&self) -> bool {
        //iyr (Issue Year) - four digits; at least 2010 and at most 2020.
        if let Ok(iyr) = self.fields["iyr"].parse::<usize>() {
            (2010..=2020).contains(&iyr)
        } else {
            false
        }
    }

    fn exp_year_valid(&self) -> bool {
        //eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
        if let Ok(eyr) = self.fields["eyr"].parse::<usize>() {
            (2020..=2030).contains(&eyr)
        } else {
            false
        }
    }

    fn height_valid(&self) -> bool {
        //hgt (Height) - a number followed by either cm or in:
        //    If cm, the number must be at least 150 and at most 193.
        //    If in, the number must be at least 59 and at most 76.
        let hgt = &self.fields["hgt"];
        let cm_mode = hgt.ends_with("cm");
        if let Ok(hgt_i) = hgt[0..hgt.len()-2].parse::<usize>() {
            if cm_mode {
                (150..=193).contains(&hgt_i)
            } else {
                (59..=76).contains(&hgt_i)
            }
        } else {
            false
        }
    }

    fn hair_valid(&self) -> bool {
        //hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
        let hcl = &self.fields["hcl"];
        // TODO This is probably way slower than validating character by character
        self.hair_re.is_match(hcl)
    }

    fn eye_valid(&self) -> bool {
        // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
        let ecl = &self.fields["ecl"];
        ecl.eq("amb") || ecl.eq("blu") || ecl.eq("brn") 
        || ecl.eq("gry") ||  ecl.eq("grn") ||  ecl.eq("hzl") ||  ecl.eq("oth")
    }

    fn pid_valid(&self) -> bool {
        // pid (Passport ID) - a nine-digit number, including leading zeroes.
        let pid = &self.fields["pid"];
        // TODO This is probably way slower than validating character by character
        self.pid_re.is_match(pid)
    }
}

impl Display for Passport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // This feels slow, but Display is only for debugging
        write!(
            f, 
            "{}", 
            &self.fields.iter()
                .map(|kvp| format!("{}->{}", kvp.0, kvp.1))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}