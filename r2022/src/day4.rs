use std::str::FromStr;
use crate::DayData;
use crate::solution::{Example, StructSolution};

type Input_1 = Vec<ElfPair>;
type Output_1 = u32;
type Input_2 = Vec<ElfPair>;
type Output_2 = ();

pub struct ElfPair {
    pub elf_1_start: u32,
    pub elf_1_end: u32,
    pub elf_2_start: u32,
    pub elf_2_end: u32,
}

impl FromStr for ElfPair {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, second) = s.split_once(",").unwrap();
        let (elf_1_start, elf_1_end) = first.split_once("-").unwrap();
        let (elf_2_start, elf_2_end) = second.split_once("-").unwrap();
        Ok(ElfPair {
            elf_1_start: elf_1_start.parse().unwrap(),
            elf_1_end: elf_1_end.parse().unwrap(),
            elf_2_start: elf_2_start.parse().unwrap(),
            elf_2_end: elf_2_end.parse().unwrap(),
        })
    }
}

pub fn prepare(input: String) -> Input_1 {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn part_1(input: Input_1) -> Output_1 {
    todo!("Implement part 1")
}

pub fn part_2(input: Input_2) -> Output_2 {
    todo!("Implement part 2")
}

pub fn make_sol() -> StructSolution<Input_1, Output_1, Input_2, Output_2> {
    let struct_solution = StructSolution {
        prepare_part_1: prepare,
        calc_part_1: part_1,
        prepare_part_2: prepare,
        calc_part_2: part_2,
        example_part_1: Example::Value(2),
        example_part_2: Example::Value(()),
        day_data: DayData::new(u8::MAX, false),
    };
    struct_solution
}
