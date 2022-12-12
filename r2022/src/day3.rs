use crate::DayData;
use crate::solution::{Example, StructSolution};

pub struct RuckSack {
    compartment_1: Vec<char>,
    compartment_2: Vec<char>,
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

fn priority(c: char) -> u8 {
    let digit : u8 = c as u8;
    match digit {
        // Uppercase chars
        65u8..=90u8 => digit - 65 + 27,
        // Lowercase chars
        97u8..=122u8 => digit - 97 + 1,
        _ => panic!("Invalid char: {}", c)
    }
}

pub fn part_1(input: Vec<RuckSack>) -> u64 {
    let mut score = 0;
    for rucksack in input {
        for c1 in &rucksack.compartment_1 {
            for c2 in &rucksack.compartment_2 {
                if c1 == c2 {
                    score += priority(*c1) as u64;
                }
            }
        }
    }
    score
}

pub fn part_2(input: Vec<RuckSack>) -> u64 {
    todo!("Implement part 2")
}

pub fn make_sol() -> StructSolution<Vec<RuckSack>, u64, Vec<RuckSack>, u64> {
    let struct_solution = StructSolution {
        prepare_part_1: prepare,
        calc_part_1: part_1,
        prepare_part_2: prepare,
        calc_part_2: part_2,
        example_part_1: Example::Value(157),
        example_part_2: Example::Value(0),
        day_data: DayData::new(3, false),
    };
    struct_solution
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority() {
        assert_eq!(priority('a'), 1);
        assert_eq!(priority('z'), 26);
        assert_eq!(priority('A'), 27);
        assert_eq!(priority('Z'), 52);
    }
}