use crate::solution::{Example, StructSolution};
use crate::DayData;
use std::str::FromStr;

type Input1 = Vec<u64>;
type Output1 = usize;
type Input2 = Vec<u64>;
type Output2 = usize;

pub enum Operand {
    Old,
    Const { val: u32 },
}

pub enum Operator {
    Mul,
    Add,
}

pub struct Operation {
    left: Operand,
    right: Operand,
    operator: Operator,
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {}
}

pub struct MonkeyTest {
    divisible_by: u16,
    true_monkey: usize,
    false_monkey: usize,
}

pub struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    Test: MonkeyTest,
}

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
        example_part_1: Example::Value(10605),
        example_part_2: Example::Value(0),
        day_data: DayData::new(11, false),
    };
    struct_solution
}
