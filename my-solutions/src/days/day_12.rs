use std::str::FromStr;
use std::sync::{Arc, Mutex};
use rudvent_lib::solution::{SolutionBuilder, StructSolutionBuilder};
use rudvent_lib::solution::execution::{EmptyUserMonitor, EmptyUserParams, Example, RunParams, RuntimeMonitor};
use crate::days::day_12::Entry::{Damaged, Operational, Unknown};

// Update these types to reflect the types you want to use to solve the problems. These
// can be simple types (u64), integers, or your own types
type InputPart1 = Vec<SpringLine>;
type OutputPart1 = usize;
type InputPart2 = InputPart1;
type OutputPart2 = usize;

// These constants hold the answer for the examples, they are used to test your code
const EXAMPLE_1_ANS: OutputPart1 = 21;
const EXAMPLE_2_ANS: OutputPart2 = 0;

// This currently only the information about whether the run is an example or not. It may be augmented
type UserParams = EmptyUserParams;
type UserMonitor = EmptyUserMonitor;

enum Entry {
    Operational,
    Damaged,
    Unknown,
}

impl Entry {
    fn from_char(c: char) -> Entry {
        match c {
            '.' => Operational,
            '#' => Damaged,
            '?' => Unknown,
            _ => unreachable!("Unrecognised character")
        }
    }
}

pub struct SpringLine {
    springs: Vec<Entry>,
    orders: Vec<usize>,
}

impl SpringLine {
    fn from_str(s: &str) -> SpringLine {
        let (spring_str, order_str) = s.split_once(" ").unwrap();
        let springs = spring_str.chars().map(Entry::from_char).collect();
        let orders = order_str.split(",").map(|o| usize::from_str(o).unwrap()).collect();
        SpringLine {
            springs,
            orders,
        }
    }
}

// This function is called to prepare the input for part 1
pub fn prepare(input: String) -> InputPart1 {
    input.lines().map(SpringLine::from_str).collect()
}

// Implement your solution for part 1 here
pub fn part_1(
    mut input: InputPart1,
    run_parameter: &RunParams<UserParams>,
    monitor: Arc<Mutex<RuntimeMonitor<EmptyUserMonitor>>>,
) -> OutputPart1 {
    todo!("Implement part 1")
}

// If the puzzle requires a different input for part 2, this function can be updated
pub fn prepare_2(input: String) -> InputPart2 {
    prepare(input)
}

pub fn part_2(
    mut input: InputPart1,
    run_parameter: &RunParams<UserParams>,
    monitor: Arc<Mutex<RuntimeMonitor<EmptyUserMonitor>>>,
) -> OutputPart1 {
    todo!("Implement part 2")
}

// ----- There is no need to change anything below this line -----
// The below code creates a solution that is generic over several types. These types might change
// between different days, for example integers on some and strings on others. They are type-aliased
// above to make it easier to change them all at once
pub fn make_sol() -> Box<dyn SolutionBuilder> {
    let sol = StructSolutionBuilder::new(
        prepare,
        part_1,
        prepare_2,
        part_2,
        Example::Value(EXAMPLE_1_ANS),
        Example::Value(EXAMPLE_2_ANS),
    );
    Box::new(sol)
}
