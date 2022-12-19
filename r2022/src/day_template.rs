use crate::solution::{Example, StructSolution};
use crate::DayData;

type Input1 = Vec<u64>;
type Output1 = usize;
type Input2 = Vec<u64>;
type Output2 = usize;

pub fn prepare(input: String) -> Input1 {
    for line in input.lines() {
        todo!()
    }
    vec![1, 2, 3]
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
        example_part_1: Example::Value(0),
        example_part_2: Example::Value(0),
        day_data: DayData::new(u8::MAX, false),
    };
    struct_solution
}