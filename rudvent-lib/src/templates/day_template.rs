use rudvent_lib::runner::{Monitor, StructSolution};
use rudvent_lib::runner::Monitor;

type Input1 = Vec<u64>;
type Output1 = usize;
type Input2 = Vec<u64>;
type Output2 = usize;
type ExampleParam = ();

pub fn prepare(input: String) -> Input1 {
    for line in input.lines() {
        todo!()
    }
    vec![1, 2, 3]
}

pub fn part_1(mut input: Input1, run_parameter: &ExampleParam, monitor: &mut Monitor) -> Output1 {
    todo!("Implement part 1")
}

pub fn part_2(mut input: Input1, run_parameter: &ExampleParam, monitor: &mut Monitor) -> Output1 {
    todo!("Implement part 2")
}

pub fn make_sol() -> StructSolution<Input1, Output1, Input2, Output2, ExampleParam, ExampleParam> {
    let struct_solution = StructSolution {
        prepare_part_1: prepare,
        calc_part_1: part_1,
        prepare_part_2: prepare,
        calc_part_2: part_2,
        example_part_1: Example::Value(0),
        example_part_2: Example::Value(0),
        example_1_run_parameter: ((), ()),
        example_2_run_parameter: ((), ()),
        day_data: DayData::new(u8::MAX, false),
    };
    struct_solution
}
