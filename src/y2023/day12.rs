use std::{fmt::Display, collections::HashMap};

//use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use super::super::day::Day;

pub struct Day12
{
    rows: Vec<Row>,
    unknown_groups_cache: HashMap<usize, Vec<Vec<(SpringType, usize)>>>,
}

impl Day12 {
    pub fn new() -> Day12
    {
        let input = "???.### 1,1,3\n.??..??...?##. 1,1,3\n?#?#?#?#?#?#?#? 1,3,1,6\n????.#...#... 4,1,1\n????.######..#####. 1,6,5\n?###???????? 3,2,1";
        //let input = include_str!("../../input/y2023/12");

        let rows = input.lines().map(Row::from_str).collect();
            
        Day12 { rows, unknown_groups_cache: HashMap::new() }
    }

    fn truncate_rows(&mut self) {
        // self.rows = self.rows.iter().map(Row::truncated_row).collect()
    }
}

impl Day for Day12 {
    fn day_name(&self) -> String { String::from("12") }
    fn answer1(&self) -> String { String::from("7032") }
    fn answer2(&self) -> String { String::from("??") }

    fn part1(&mut self) -> String {
        // println!();
        self.truncate_rows();
        for row in &self.rows {
            println!("{row}");
            println!("{row}: {}", row.resolve_unknowns(&mut self.unknown_groups_cache).iter().filter(|c| row.is_satisfied_by(c)).count());
            println!("Yields: ");
            for s in row.resolve_unknowns(&mut self.unknown_groups_cache).iter() {
                println!("  {}: {}", _springs_to_string(s), row.is_satisfied_by(s));
            }
            let mut buf = String::new();
            std::io::stdin().read_line(&mut buf).unwrap();
        }

        self.rows.iter()
            .map(|row| row.resolve_unknowns(&mut self.unknown_groups_cache).iter()
                .filter(|c| row.is_satisfied_by(c))
                .count())
            .sum::<usize>()
            .to_string()
        //String::new()
    }

    fn part2(&mut self) -> String {
        // self.rows.par_iter()
        //     .map(|row| row.unfolded().resolve_unknowns().iter()
        //         .filter(|c| row.is_satisfied_by(c))
        //         .count())
        //     .sum::<usize>()
        //     .to_string()
        String::new()
    }
}

fn _group_to_string(input: &(SpringType, usize)) -> String {
    let mut r = String::new();
    for _ in 0..input.1 {
        r += &input.0.to_string();
    }
    r
}

fn _springs_to_string(input: &[SpringType]) -> String {
    let mut r = String::new();
    for spring in input {
        r += &spring.to_string();
    }
    r
}

fn _groups_to_string(input: &Vec<(SpringType, usize)>) -> String {
    let mut r = String::new();
    for group in input {
        r += &_group_to_string(group);
    }
    r
}

fn _vec_of_group_vec_to_string(input: &Vec<Vec<(SpringType, usize)>>) -> String {
    let mut r = String::new();
    for group in input {
        r += &("->".to_owned() + &_groups_to_string(group) + "\n");
    }
    r
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum SpringType {
    Operational,
    Broken,
    Unknown,
}

impl SpringType {
    fn from_char(input: char) -> SpringType {
        match input {
            '.' => SpringType::Operational,
            '#' => SpringType::Broken,
            '?' => SpringType::Unknown,
            _ => panic!("Unknown Group Type"),
        }
    }
}

#[derive(Clone)]
struct Row {
    #[allow(dead_code)]
    springs: Vec<SpringType>,
    groups: Vec<(SpringType, usize)>,
    bad_groups: Vec<usize>,
}

impl Row {
    fn from_str(input: &str) -> Row {
        let (springs, rest) = input.split_once(' ').unwrap();
        let springs = springs.chars().map(SpringType::from_char).collect();
        let groups = Row::groups_from_springs(&springs);
        let bad_groups = rest.split(',').map(|s| s.parse::<usize>().unwrap()).collect();
        Row { 
            springs, 
            groups, 
            bad_groups 
        }
    }

    // fn truncated_row(&self) -> Self {
    //     // Remove any groups that are fulfilled by the knowns in the group
    //     println!("truncating {self}");
    //     let mut groups = vec![];
    //     let mut bad_groups = vec![];
    //     // Front
    //     let mut first_unknown_group = 0;
    //     let mut first_not_dropped_group_index = 0;
    //     while self.groups[first_unknown_group].0 != SpringType::Unknown {
    //         if self.groups[first_unknown_group].0 == SpringType::Broken && self.groups[first_unknown_group + 1].0 != SpringType::Unknown {
    //             println!("Dropping group {}", self.bad_groups[first_not_dropped_group_index]);
    //             first_not_dropped_group_index += 1;
    //         }
    //     }

    //     groups = self.groups[first_not_dropped_group_index..].to_vec();


    //     let springs = Self::springs_from_groups(&groups);
    //     Row {
    //         springs,
    //         groups,
    //         bad_groups,
    //     }
    // }

    fn _springs_from_groups(groups: &Vec<(SpringType, usize)>) -> Vec<SpringType> {
        let mut r = vec![];
        for group in groups {
            for _ in 0..group.1 {
                r.push(group.0);
            }
        }
        r
    }

    fn groups_from_springs(springs: &Vec<SpringType>) -> Vec<(SpringType, usize)> {
        let mut groups = vec![];

        let mut current_group = springs[0];
        let mut group_size = 0;
        for &spring in springs {
            if spring == current_group {
                group_size += 1;
            } else {
                groups.push((current_group, group_size));
                current_group = spring;
                group_size = 1;
            }
        }
        groups.push((current_group, group_size));
        groups
    }

    fn _unfolded(&self) -> Row {
        let mut unfolded_springs = vec![];
        let mut unfolded_bad_groups = vec![];
        for _ in 0..5 {
            unfolded_springs.append(&mut self.springs.clone());
            unfolded_springs.push(SpringType::Unknown);
            unfolded_bad_groups.append(&mut self.bad_groups.clone());
        }

        Row {
            springs: unfolded_springs,
            bad_groups: unfolded_bad_groups,
            groups: self.groups.clone(),
        }
    }

    fn resolve_unknowns(&self, _cache: &mut HashMap<usize, Vec<Vec<(SpringType, usize)>>>) -> Vec<Vec<SpringType>> {
        // let mut result = vec![self.groups.clone()];

        // println!("Resolve Unknowns of {}", groups_to_string(&self.groups));
        // for (group_index, group) in self.groups.iter().enumerate() {
        //     if group.0 != SpringType::Unknown { continue; }
        //     let unknown_group_perms = Self::unknown_group_permutations(group.1, cache);
        //     println!("unknown group {} becomes \n{}", group_to_string(&group), vec_of_group_vec_to_string(&unknown_group_perms));
        //     result = result.iter().flat_map(|springs| {
        //         // Remove the unknown group
        //         let mut base = springs.clone();
        //         base.remove(group_index);
        //         unknown_group_perms.iter().map(move |perm| {
        //             println!("-> Substituting {} into {} at index {}", groups_to_string(perm), groups_to_string(&base), group_index);
        //             let mut result = base.clone();
        //             for (i, grp) in perm.iter().enumerate() {
        //                 result.insert(group_index + i, *grp);
        //             }
        //             result
        //         })                
        //     }).collect();

        //     println!("After that substitution:");
        //     for r in &result {
        //         println!("  {}", groups_to_string(&r));
        //     }
        //     println!("------");
        // }

        let mut result = vec![self.springs.clone()];
        
        for (i, &spring) in self.springs.iter().enumerate() {
            if spring == SpringType::Unknown {
                result = result.iter().flat_map(|vec| {
                    let mut good = vec.clone();
                    let mut bad = vec.clone();
                    good[i] = SpringType::Operational; 
                    bad[i] = SpringType::Broken;
                    vec![good, bad]
                })
                .filter(|candidate| self.can_be_satisfied_by(candidate))
                .collect();
            }
        }

        result
    }

    fn _unknown_group_permutations(length: usize, cache: &mut HashMap<usize, Vec<Vec<(SpringType, usize)>>>) -> Vec<Vec<(SpringType, usize)>> {
        if cache.contains_key(&length) {
            return cache.get(&length).unwrap().clone()
        }

        if length == 1 {
            return vec![vec![(SpringType::Operational, 1)], vec![(SpringType::Broken, 1)]];
        }

        let rest_perms = Self::_unknown_group_permutations(length - 1, cache);

        let resolve_with = |spring_type, perms: &Vec<(SpringType, usize)>| {
            // This is hideous
            let mut result = perms.clone();
            let last_i = result.len() - 1;
            let last = result[last_i];
            if last.0 == spring_type {
                result[last_i] = (spring_type, last.1 + 1);
            } else {
                result.push((spring_type, 1));
            }
            result
        };

        let mut good: Vec<_> = rest_perms.iter().map(|p| resolve_with(SpringType::Operational, p)).collect();
        let mut bad: Vec<_> = rest_perms.iter().map(|p| resolve_with(SpringType::Broken, p)).collect();

        good.append(&mut bad);
        cache.insert(length, good.clone());
        good
    }

    fn can_be_satisfied_by(&self, candidate: &Vec<SpringType>) -> bool {
        let groups = Row::groups_from_springs(candidate);
        let bad_groups = groups.iter().filter(|g| g.0 != SpringType::Operational).collect::<Vec<_>>();

        for i in 0..bad_groups.len() {
            if bad_groups[i].0 == SpringType::Unknown {
                // Give up on checking this for now
                return true;
            }
            if i >= self.bad_groups.len() {
                // ???
                return true;
            }
            if bad_groups[i].1 != self.bad_groups[i] {
                println!("Disqualifying {} as candidate for {self}", _springs_to_string(candidate));
                return false;
            }
        }
        //println!("Happy");
        true
    }

    fn is_satisfied_by(&self, candidate: &Vec<SpringType>) -> bool {
        let groups = Row::groups_from_springs(candidate);
        let bad_groups = groups.iter().filter(|g| g.0 == SpringType::Broken).collect::<Vec<_>>();

        if bad_groups.len() != self.bad_groups.len() {
            return false;
        }

        for i in 0..bad_groups.len() {
            if bad_groups[i].1 != self.bad_groups[i] {
                return false;
            }
        }
        true

        // let mut bad_group_index = 0;
        
        // for &group in &groups {
        //     if group.0 == SpringType::Broken && group.1 == self.bad_groups[bad_group_index] {
        //         bad_group_index += 1;
        //         if bad_group_index == self.bad_groups.len() {
        //             return true;
        //         }
        //     }
        // }

        // return false;
    }
}

impl Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for group in &self.groups {
            for _ in 0..group.1 {
                write!(f, "{}", group.0).unwrap();
            }
        }
        write!(f, " ").unwrap();

        for group in &self.bad_groups {
            write!(f, "{group},").unwrap();
        }
        write!(f, "")
    }
}

impl Display for SpringType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            SpringType::Operational => ".",
            SpringType::Broken => "#",
            SpringType::Unknown => "?",
        })
    }
}