use crate::utils::{Example, StructSolution};
use crate::DayData;
use color_eyre::eyre::{eyre, Result};
use std::fmt::Display;

pub struct Elf {
    items: Vec<u64>,
}

impl Elf {
    fn new(items: Vec<u64>) -> Self {
        Self { items }
    }

    fn calorie_sum(&self) -> u64 {
        self.items.iter().sum()
    }
}

pub fn prepare(input: String) -> Vec<Elf> {
    let mut elves = Vec::new();
    let mut cals = Vec::new();
    for line in input.lines() {
        if line.len() == 0 {
            elves.push(Elf::new(cals));
            cals = Vec::new();
        } else {
            cals.push(line.parse::<u64>().unwrap());
        }
    }
    if cals.len() > 0 {
        elves.push(Elf::new(cals));
    }
    elves
}

pub fn part_1(input: Vec<Elf>) -> u64 {
    input.iter().map(|elf| elf.calorie_sum()).max().unwrap()
}

pub fn part_2(input: Vec<Elf>) -> u64 {
    // Sort the elves by calorie count
    let mut cals: Vec<u64> = input.iter().map(|elf| elf.calorie_sum()).collect();
    cals.sort();
    cals[cals.len() - 3..].iter().sum()
}

pub fn make_sol() -> StructSolution<Vec<Elf>, u64, u64> {
    let struct_solution = StructSolution {
        prepare_part_1: prepare,
        calc_part_1: part_1,
        prepare_part_2: prepare,
        calc_part_2: part_2,
        example_part_1: Example::Value(24000),
        example_part_2: Example::Value(45000),
        day_data: DayData::new(1, false),
    };
    struct_solution
}

pub(crate) fn solution(example: bool) -> Result<impl Display> {
    let day_data = DayData::new(1, false);
    let elves = prepare(day_data.input_1());
    let answer = elves.iter().map(|elf| elf.calorie_sum()).max().unwrap();
    // day_data.post_1(answer.to_string().as_str())?;
    Ok(answer)
}
