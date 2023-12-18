#[allow(unused_imports)]
use std::{fmt::Display, collections::HashMap, iter::repeat};

//use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use super::super::day::Day;

type Group = (SpringType, usize);

pub struct Day12
{
    rows: Vec<Row>,
    unknown_groups_cache: HashMap<usize, Vec<Vec<Group>>>,
    score_cache: HashMap<(Vec<Group>, Vec<usize>), usize>,
}

impl Day12 {
    pub fn new() -> Day12
    {
        //let input = "???.### 1,1,3\n.??..??...?##. 1,1,3\n?#?#?#?#?#?#?#? 1,3,1,6\n????.#...#... 4,1,1\n????.######..#####. 1,6,5\n?###???????? 3,2,1";
        let input = include_str!("../../input/y2023/12");

        let rows = input.lines().map(Row::from_str).collect();
            
        Day12 { rows, unknown_groups_cache: HashMap::new(), score_cache: HashMap::new() }
    }
}

impl Day for Day12 {
    fn day_name(&self) -> String { String::from("12") }
    fn answer1(&self) -> String { String::from("7032") }
    fn answer2(&self) -> String { String::from("??") }

    fn part1(&mut self) -> String {
        // let rows = vec![
        //     Row::from_str("?#?#????.? 2,2,1"),
        // ];

        // self.rows.iter()
        //     .map(|row| 
        //         {
        //             let n = Row::count_satisfies(&row.groups, &row.bad_groups, &mut self.unknown_groups_cache, &mut self.score_cache, 0);
        //             // let o = row._resolve_unknowns(&mut HashMap::new()).iter().filter(|c| row._is_satisfied_by(c)).count();
        //             // if n != o {
        //             //     println!("Mismatch on row {row} - got {n} should be {o}");
        //             // }
        //             n
        //         })
        //     .sum::<usize>()
        //     .to_string()
        String::new()
    }

    fn part2(&mut self) -> String {
        // let mut cache: Vec<_> = self.score_cache.iter().collect();
        // cache.sort_by_cached_key(|(k, _v)| _groups_to_string(&k.0).len());
        // for (k, v) in cache {
        //     if v > &0 {
        //         println!("{} {:?} = {}", _groups_to_string(&k.0), k.1, v);
        //         _wait();
        //     }
        // }
        // self.rows.par_iter()
        //     .map(|row| row.unfolded().resolve_unknowns().iter()
        //         .filter(|c| row.is_satisfied_by(c))
        //         .count())
        //     .sum::<usize>()
        //     .to_string()

        // let unfolded = self.rows[self.rows.len() - 1]._unfolded();
        // println!("{:?}", unfolded.groups);
        // let n = Row::count_satisfies(&unfolded.groups, &unfolded.bad_groups, &mut self.unknown_groups_cache, &mut self.score_cache, 0);
        // //let o = unfolded._resolve_unknowns(&mut HashMap::new()).iter().filter(|c| unfolded._is_satisfied_by(c)).count();
        // println!("{unfolded} = {n}");
        // //println!("Original: {o}");

        self.rows.iter()
            .enumerate()
            .map(|(i, row)| 
                {
                    let unfolded = row._unfolded();
                    let n = Row::count_satisfies(&unfolded.groups, &unfolded.bad_groups, &mut self.unknown_groups_cache, &mut self.score_cache, 0);
                    println!("{i}/{} : {unfolded} = {n}", self.rows.len());
                    // let o = row._resolve_unknowns(&mut HashMap::new()).iter().filter(|c| row._is_satisfied_by(c)).count();
                    // if n != o {
                    //     println!("Mismatch on row {row} - got {n} should be {o}");
                    // }
                    n
                })
            .sum::<usize>()
            .to_string()
    }
}

fn _group_to_string(input: &Group) -> String {
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

fn _groups_to_string(input: &Vec<Group>) -> String {
    let mut r = String::new();
    for group in input {
        r += &_group_to_string(group);
    }
    r
}

fn _vec_of_group_vec_to_string(input: &Vec<Vec<Group>>) -> String {
    let mut r = String::new();
    for group in input {
        r += &("->".to_owned() + &_groups_to_string(group) + "\n");
    }
    r
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
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
    groups: Vec<Group>,
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

    fn _springs_from_groups(groups: &Vec<Group>) -> Vec<SpringType> {
        let mut r = vec![];
        for group in groups {
            for _ in 0..group.1 {
                r.push(group.0);
            }
        }
        r
    }

    fn groups_from_springs(springs: &Vec<SpringType>) -> Vec<Group> {
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

    fn _deduplicate(&mut self) {
        println!("Deduplicating: {:?}", self.groups);
        let mut clean_groups = vec![];
        let mut i = 0;
        while i < self.groups.len() {
            clean_groups.push(self.groups[i]);
            let last_clean = clean_groups.len() - 1;
            while i < self.groups.len() - 1 && self.groups[i + 1].0 == clean_groups[last_clean].0 {
                clean_groups[last_clean].1 += self.groups[i + 1].1;
                i += 1;
            }
            i += 1;
        }
        println!("result: {:?}", clean_groups);
        self.groups = clean_groups;
    }

    fn _unfolded(&self) -> Row {
        let mut unfolded_springs = vec![];
        let mut unfolded_bad_groups = vec![];
        const REPEATS: usize = 5;
        for repeat in 0..REPEATS {
            unfolded_springs.append(&mut self.springs.clone());
            if repeat != REPEATS - 1 { unfolded_springs.push(SpringType::Unknown); }
            unfolded_bad_groups.append(&mut self.bad_groups.clone());
        }

        let unfolded = Row {
            springs: unfolded_springs.clone(),
            bad_groups: unfolded_bad_groups,
            groups: Row::groups_from_springs(&unfolded_springs),
        };
        //unfolded.deduplicate();
        unfolded
    }

    fn count_satisfies(groups: &[Group], target_bad_groups: &[usize], unknown_groups_cache: &mut HashMap<usize, Vec<Vec<Group>>>, score_cache: &mut HashMap<(Vec<Group>, Vec<usize>),usize>, indent: usize) -> usize {
        assert_no_consecutives(groups); 
        // if let Some(score) = score_cache.get(&(groups.to_vec(), target_bad_groups.to_vec())) {
        //     return *score;
        // }
        _debug_print(indent, format!("Evaluating '{}' against  {target_bad_groups:?}", _groups_to_string(&groups.to_vec())));
        if groups.is_empty() {
            if target_bad_groups.is_empty() {
                _debug_print(indent, format!("Valid path"));
                return 1;
            } else {
                _debug_print(indent, format!("Failed to satisfy"));
                return 0;
            }
        }

        if target_bad_groups.is_empty() {
            if groups.iter().any(|group| group.0 == SpringType::Broken) {
                _debug_print(indent, format!("Leftover broken"));
                return 0;
            }
        }

        let score = match groups[0].0 {
            SpringType::Operational => {
                // Drop the operational group
                //_debug_print(indent, format!("Dropping leading dots..."));
                Self::count_satisfies(&groups[1..], target_bad_groups, unknown_groups_cache, score_cache, indent + 1)
            },
            SpringType::Broken => {
                if groups.len() > 1 && groups[1].0 == SpringType::Broken {
                        // Two consecutive broken groups (because of a lookahead above), combine them
                        let mut candidate = groups.to_vec().clone();
                        candidate[1].1 += candidate[0].1;
                        candidate.remove(0); 
                        _debug_print(indent, format!("With broken combination into {}", _groups_to_string(&candidate)));
                        Self::count_satisfies(&candidate, target_bad_groups, unknown_groups_cache, score_cache, indent + 1)
                    } 
                else if groups[0].1 < target_bad_groups[0] {
                    // We may be able to satisfy if we have unknowns to expand after this group
                    if groups.len() > 1 && groups[1].0 == SpringType::Unknown {
                        let mut candidates = Self::_unknown_group_permutations(groups[1].1, unknown_groups_cache);
                        candidates.iter_mut()
                            .map(|candidate| {
                                // This is only helpful if the candidate starts with a broken group
                                if candidate[0].0 != SpringType::Broken {
                                    0
                                } else {
                                    // Combine groups[0] + [candidate] + the rest of groups
                                    candidate[0].1 += groups[0].1;
                                    candidate.extend_from_slice(&groups[2..]);
                                    // We have used groups[0] (the broken group) and groups[1] the unkown group
                                    //_debug_print(indent, format!("With combination into {}", _groups_to_string(candidate)));
                                    let score = Self::count_satisfies(&candidate, target_bad_groups, unknown_groups_cache, score_cache, indent + 1);
                                    _debug_print(indent, format!("Subproblem {} {:?} score {}", _groups_to_string(candidate), target_bad_groups, score));
                                    score
                                }
                            }).sum()
                    } else {
                        _debug_print(indent, format!("Group too short and can't expand..."));
                        return 0;
                    }
                } else if groups[0].1 == target_bad_groups[0] {
                    _debug_print(indent, format!("Consuming broken group..."));
                    // When consuming a broken group, if the next group is unknown, we must force the first choice to be a dot
                    let mut new_groups = groups[1..].to_vec().clone();
                    if !new_groups.is_empty() && new_groups[0].0 == SpringType::Unknown {
                        if new_groups[0].1 == 1 {
                            new_groups[0].0 = SpringType::Operational;
                        } else {
                            new_groups[0].1 -= 1;
                            new_groups.insert(0, (SpringType::Operational, 1));
                        }
                    }
                    Self::count_satisfies(&new_groups, &target_bad_groups[1..], unknown_groups_cache, score_cache, indent + 1)
                } else {
                    // The broken group is longer than the first target group
                    _debug_print(indent, format!("Group too long"));
                    return 0;
                }
            },
            SpringType::Unknown => {
                // Check each possible expansion of the unknown groups
                let mut candidates = Self::_unknown_group_permutations(groups[0].1, unknown_groups_cache);
                _debug_print(indent, format!("Expanded {} unknowns...", groups[0].1));

                candidates.iter_mut()
                    .map(|candidate| {
                        _debug_print(indent, format!("\nNew choice: {}", _groups_to_string(candidate)));
                        // Do not send consecutive broken groups
                        let last_i = candidate.len() - 1;
                        // TODO This maybe should jsut be groups[1].0 == groups[0].0
                        //if groups.len() > 1 && groups[1].0 == SpringType::Broken && candidate[last_i].0 == SpringType::Broken {
                        if groups.len() > 1 && groups[1].0 == candidate[last_i].0 {
                            candidate[last_i].1 += groups[1].1;
                            candidate.extend_from_slice(&groups[2..]);
                        } else {
                            candidate.extend_from_slice(&groups[1..]);
                        }
                        //_wait();
                        Self::count_satisfies(&candidate, target_bad_groups, unknown_groups_cache, score_cache, indent + 1)
                    }).sum()
            }
        };

        _debug_print(indent, format!("Score {score} for {} against {target_bad_groups:?}", _groups_to_string(&groups.to_vec())));

        score_cache.insert((groups.to_vec(), target_bad_groups.to_vec()), score);
        score
    }

    fn _resolve_unknowns(&self, _cache: &mut HashMap<usize, Vec<Vec<Group>>>) -> Vec<Vec<SpringType>> {
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
                //.filter(|candidate| self._can_be_satisfied_by(candidate))
                .collect();
            }
        }

        result
    }

    fn _unknown_group_permutations(length: usize, cache: &mut HashMap<usize, Vec<Vec<Group>>>) -> Vec<Vec<Group>> {
        // TODO can we omit all dot answers here? i.e. '...' is that ever going to change the answer...
        if cache.contains_key(&length) {
            return cache.get(&length).unwrap().clone()
        }

        if length == 1 {
            return vec![vec![(SpringType::Operational, 1)], vec![(SpringType::Broken, 1)]];
        }

        let rest_perms = Self::_unknown_group_permutations(length - 1, cache);

        let resolve_with = |spring_type, perms: &Vec<Group>| {
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

    fn _can_be_satisfied_by(&self, candidate: &Vec<SpringType>) -> bool {
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
                //println!("Disqualifying {} as candidate for {self}", _springs_to_string(candidate));
                return false;
            }
        }
        //println!("Happy");
        true
    }

    fn _is_satisfied_by(&self, candidate: &Vec<SpringType>) -> bool {
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

fn assert_no_consecutives(_groups: &[(SpringType, usize)]) {
    // if !groups.is_empty() {
    //     for i in 0..(groups.len() - 1) {
    //         if groups[i].0 == groups[i + 1].0 {
    //             //println!("Consecutive groups here {:?}", &groups.to_vec());
    //             println!("Consecutive groups here {}", groups[0].0);
    //             //assert!(false);
    //         }
    //     }
    // }
}

fn _debug_print(_indent: usize, _message: String) {
    // println!("{}{}", 
    //     repeat(' ').take(_indent).fold(String::new(), |acc, e| acc + &e.to_string()),
    //     _message);
}

fn _wait() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
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