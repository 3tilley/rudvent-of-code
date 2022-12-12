use std::collections::HashMap;
use crate::DayData;
use crate::solution::{Example, StructSolution};

pub struct RuckSack {
    compartment_1: Vec<char>,
    compartment_2: Vec<char>,
}

impl RuckSack {
    pub fn repeated_item_type(&self) -> u8 {
        for c1 in &self.compartment_1 {
            for c2 in &self.compartment_2 {
                if c1 == c2 {
                    let prior = priority(c1);
                    // println!("{} == {}. Priority: {}", c1, c2, prior);
                    return prior;
                }
            }
        }
        0
    }
}

pub fn prepare(input: String) -> Vec<RuckSack> {
    let mut rucksacks = Vec::new();
    for line in input.lines() {
        let (first, second) = line.split_at(line.len() / 2);
        rucksacks.push(RuckSack {
            compartment_1: first.chars().collect(),
            compartment_2: second.chars().collect(),
        });
    }
    rucksacks
}

fn priority(c: &char) -> u8 {
    let digit : u8 = *c as u8;
    match digit {
        // Uppercase chars
        65u8..=90u8 => digit - 65 + 27,
        // Lowercase chars
        97u8..=122u8 => digit - 97 + 1,
        _ => panic!("Invalid char: {}", c)
    }
}

pub fn part_1(input: Vec<RuckSack>) -> u64 {
    input.iter().map(|rucksack| rucksack.repeated_item_type() as u64).sum()
}

fn process_three_rucksacks(rucksacks: &[RuckSack]) -> u64 {
    let mut bigmap = HashMap::new();
    for r in rucksacks {
        let mut small_map = HashMap::new();
        let minimap = r.compartment_1.iter().chain(r.compartment_2.iter()).fold(small_map, |mut map, c| {
            let count = map.entry(c).or_insert(0u8);
            *count += 1;
            map
        });
        for (key, value) in minimap {
            if value > 0u8 {
                let big_count = bigmap.entry(key).or_insert(0u8);
                *big_count += 1;
            }
        }
    }
    let priors: Vec<&char> = bigmap.iter().filter_map(|(key, value)| {
        if *value == 3u8 {
            Some(*key)
        } else {
            None
        }
    }).collect();

    if priors.len() == 1 {
        priority(priors[0]) as u64
    } else {
        panic!("Invalid number of priorities: {}, {:?}", priors.len(), priors)
    }
}


pub fn part_2(input: Vec<RuckSack>) -> u64 {
    input.chunks(3).map(|rucksacks| process_three_rucksacks(rucksacks)).sum()
}

pub fn make_sol() -> StructSolution<Vec<RuckSack>, u64, Vec<RuckSack>, u64> {
    let struct_solution = StructSolution {
        prepare_part_1: prepare,
        calc_part_1: part_1,
        prepare_part_2: prepare,
        calc_part_2: part_2,
        example_part_1: Example::Value(157),
        example_part_2: Example::Value(70),
        day_data: DayData::new(3, false),
    };
    struct_solution
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority() {
        assert_eq!(priority(&'a'), 1);
        assert_eq!(priority(&'z'), 26);
        assert_eq!(priority(&'A'), 27);
        assert_eq!(priority(&'Z'), 52);
    }

    #[test]
    fn test_part_1_basic() {
        let input = prepare("abcdef".to_string());
        assert_eq!(input.len(), 1);
        assert_eq!(input[0].compartment_1, vec!['a', 'b', 'c']);
        assert_eq!(input[0].compartment_2, vec!['d', 'e', 'f']);
        assert_eq!(part_1(input), 0);
    }

    #[test]
    fn test_part_1_a() {
        let input = prepare("abcdea".to_string());
        assert_eq!(input.len(), 1);
        assert_eq!(input[0].compartment_1, vec!['a', 'b', 'c']);
        assert_eq!(input[0].compartment_2, vec!['d', 'e', 'a']);
        assert_eq!(part_1(input), 1);
    }

    #[test]
    fn test_part_1_more_complicated() {
        let input = prepare("abcdea\nfeDDAK".to_string());
        assert_eq!(input.len(), 2);
        assert_eq!(part_1(input), 1+30);
    }

    #[test]
    fn test_part_2_basic() {
        let input = prepare("aecdea\nfeDDAK\nPObvek".to_string());
        assert_eq!(input.len(), 3);
        assert_eq!(part_2(input), 5);
    }
}