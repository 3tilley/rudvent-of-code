use crate::solution::{Example, StructSolution};
use crate::DayData;
use std::str::FromStr;

type Input1 = HeightMap;
type Output1 = usize;
type Input2 = HeightMap;
type Output2 = usize;

pub struct HeightMap {
    // This is in XY
    heights: Vec<Vec<u8>>,
    current_position: (usize, usize),
    target_position: (usize, usize),
    visited: Vec<(usize, usize)>,
}

impl HeightMap {
    pub fn options(&self) {
        for 
    }
}

impl FromStr for HeightMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut current_position = (0, 0);
        let mut target_position = (0, 0);
        let heights = s
            .lines()
            .rev()
            .enumerate()
            .map(|(x, s)| {
                s.chars()
                    .enumerate()
                    .map(|(y, c)| match c {
                        'S' => {
                            current_position = (x, y);
                            0
                        }
                        'E' => {
                            target_position = (x, y);
                            25
                        }
                        x => c as u8 - 97,
                    })
                    .collect()
            })
            .collect();

        Ok(HeightMap {
            heights,
            current_position,
            target_position,
        })
    }
}

pub fn prepare(input: String) -> Input1 {
    HeightMap::from_str(&input).unwrap()
}

pub fn part_1(input: Input1) -> Output1 {
    todo!("Implement part 1")
}

pub fn part_2(input: Input2) -> Output2 {
    todo!("Implement part 2")
}

pub fn make_sol() -> StructSolution<Input1, Output1, Input2, Output2> {
    let struct_solution = StructSolution {
        prepare_part_1: prepare,
        calc_part_1: part_1,
        prepare_part_2: prepare,
        calc_part_2: part_2,
        example_part_1: Example::Value(31),
        example_part_2: Example::Value(0),
        day_data: DayData::new(12, false),
    };
    struct_solution
}
