use num_traits::Zero;
use rudvent_lib::solution::execution::{EmptyUserMonitor, Example, RunParams, RuntimeMonitor};
use rudvent_lib::solution::{SolutionBuilder, StructSolutionBuilder};
use std::fmt::format;
use std::ops::BitOr;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use tracing::info;

// Update these types to reflect the types you want to use to solve the problems. These
// can be simple types (u64), integers, or your own types
type Reading = isize;
type InputPart1 = Vec<Vec<Reading>>;
type OutputPart1 = isize;
type InputPart2 = InputPart1;
type OutputPart2 = isize;

// These constants hold the answer for the examples, they are used to test your code
const EXAMPLE_1_ANS: OutputPart1 = 114;
const EXAMPLE_2_ANS: OutputPart2 = 2;

// This currently only the information about whether the run is an example or not. It may be augmented
type UserParams = ();
type UserMonitor = EmptyUserMonitor;

// This function is called to prepare the input for part 1
pub fn prepare(input: String) -> InputPart1 {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|c| Reading::from_str(c).expect(&format!("Unable to parse {}", c)))
                .collect()
        })
        .collect()
}

fn diffs(readings: Vec<Reading>, forwards: bool) -> Reading {
    info!("{:?}", readings);
    let mut previous = readings[0];
    let first = previous;
    let mut vec = Vec::with_capacity(readings.len() - 1);
    let mut all_zero = true;
    for reading in &readings[1..] {
        let diff = reading - previous;
        previous = *reading;
        all_zero = all_zero & diff.eq(&Reading::zero());
        vec.push(diff);
    }
    if all_zero {
        if forwards {
            previous
        } else {
            first
        }
    } else {
        if forwards {
            info!("{}", previous);
            diffs(vec, forwards) + previous
        } else {
            info!("{}", first);
            first - diffs(vec, forwards)
        }
    }
}

// Implement your solution for part 1 here
pub fn part_1(
    mut input: InputPart1,
    run_parameter: &RunParams<UserParams>,
    monitor: Arc<Mutex<RuntimeMonitor<EmptyUserMonitor>>>,
) -> OutputPart1 {
    input
        .into_iter()
        .map(|readings| diffs(readings, true))
        .sum()
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
    input
        .into_iter()
        .map(|readings| diffs(readings, false))
        .sum()
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
