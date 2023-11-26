use crate::solution::{Example, StructSolution};
use crate::DayData;
use std::str::FromStr;

type Input_1 = Vec<ElfPair>;
type Output_1 = usize;
type Input_2 = Vec<ElfPair>;
type Output_2 = usize;

#[derive(Debug)]
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

impl ElfPair {
    pub fn has_total_overlaps(&self) -> bool {
        let result = if self.elf_1_start == self.elf_2_start {
            true
        } else if self.elf_1_start < self.elf_2_start {
            self.elf_1_end >= self.elf_2_end
        } else {
            self.elf_2_end >= self.elf_1_end
        };
        // println!("{:?}, result: {}", self, result);
        result
    }

    pub fn has_any_overlaps(&self) -> bool {
        let result = if self.elf_1_start == self.elf_2_start {
            true
        } else if self.elf_1_start < self.elf_2_start {
            self.elf_1_end >= self.elf_2_start
        } else {
            self.elf_2_end >= self.elf_1_start
        };
        // println!("{:?}, result: {}", self, result);
        result
    }
}

pub fn prepare(input: String) -> Input_1 {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn part_1(input: Input_1) -> Output_1 {
    input.iter().filter(|p| p.has_total_overlaps()).count()
}

pub fn part_2(input: Input_2) -> Output_2 {
    input.iter().filter(|p| p.has_any_overlaps()).count()
}

pub fn make_sol() -> StructSolution<Input_1, Output_1, Input_2, Output_2> {
    let struct_solution = StructSolution {
        prepare_part_1: prepare,
        calc_part_1: part_1,
        prepare_part_2: prepare,
        calc_part_2: part_2,
        example_part_1: Example::Value(2),
        example_part_2: Example::Value(4),
        day_data: DayData::new(4, false),
    };
    struct_solution
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_basic() {
        let e: ElfPair = "3-3,2-4".parse().unwrap();
        let e2: ElfPair = "4-4,2-4".parse().unwrap();
        let e3: ElfPair = "1-2,2-4".parse().unwrap();
        let e4: ElfPair = "2-2,2-4".parse().unwrap();
        let e5: ElfPair = "2-2,2-2".parse().unwrap();
        let e6: ElfPair = "2-4,3-3".parse().unwrap();
        let e7: ElfPair = "2-4,4-4".parse().unwrap();
        let e8: ElfPair = "2-4,1-2".parse().unwrap();
        let e9: ElfPair = "2-4,2-2".parse().unwrap();
        let e10: ElfPair = "2-2,2-2".parse().unwrap();
        assert_eq!(e.has_total_overlaps(), true);
        assert_eq!(e2.has_total_overlaps(), true);
        assert_eq!(e3.has_total_overlaps(), false);
        assert_eq!(e4.has_total_overlaps(), true);
        assert_eq!(e5.has_total_overlaps(), true);
        assert_eq!(e6.has_total_overlaps(), true);
        assert_eq!(e7.has_total_overlaps(), true);
        assert_eq!(e8.has_total_overlaps(), false);
        assert_eq!(e9.has_total_overlaps(), true);
        assert_eq!(e10.has_total_overlaps(), true);
    }
}
