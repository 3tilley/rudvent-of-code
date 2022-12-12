use crate::DayData;
use crate::solution::{Example, StructSolution};

pub fn prepare(input: String) -> () {
    for line in input.lines() {
        todo!()
    }
}

pub fn part_1(input: ()) -> () {
    todo!("Implement part 1")
}

pub fn part_2(input: ()) -> () {
    todo!("Implement part 2")
}

pub fn make_sol() -> StructSolution<(), (), (), ()> {
    let struct_solution = StructSolution {
        prepare_part_1: prepare,
        calc_part_1: part_1,
        prepare_part_2: prepare,
        calc_part_2: part_2,
        example_part_1: Example::Value(()),
        example_part_2: Example::Value(()),
        day_data: DayData::new(u8::MAX, false),
    };
    struct_solution
}
