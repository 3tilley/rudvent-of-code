use rudvent_lib::day_data::Monitor;
use rudvent_lib::solution::{
    Example, RunParams, Solution, SolutionBuilder, StructSolution, StructSolutionBuilder,
};
use std::str::FromStr;

// Update these types to reflect the types you want to use to solve the problems. These
// can be simple types (u64), integers, or your own types
type InputPart1 = Vec<String>;
type OutputPart1 = usize;
type InputPart2 = String;
type OutputPart2 = usize;

// These constants hold the answer for the examples, they are used to test your code
const EXAMPLE_1_ANS: OutputPart1 = 142;
const EXAMPLE_2_ANS: OutputPart2 = 281;

// This currently only the information about whether the run is an example or not. It may be augmented
type UserParams = ();

// This function is called to prepare the input for part 1
pub fn prepare(input: String) -> InputPart1 {
    input.lines().map(|line| line.to_string()).collect()
}

// Implement your solution for part 1 here
pub fn part_1(
    mut input: InputPart1,
    run_parameter: &RunParams<UserParams>,
    monitor: &mut Monitor,
) -> OutputPart1 {
    input
        .iter()
        .map(|s| {
            let mut first = None;
            let mut last = None;
            for c in s.chars() {
                if let Ok(int) = u8::from_str(&*c.to_string()) {
                    first = first.or(Some(int));
                    last = Some(int);
                }
            }
            ((10 * first.expect(&*format!("Failure finding anything in {}", s)))
                + last.expect(&*format!("Failure finding anything in {}", s))) as usize
        })
        .sum()
}

// If the puzzle requires a different input for part 2, this function can be updated
pub fn prepare_2(input: String) -> InputPart2 {
    input
}

pub fn part_2(
    mut input: InputPart2,
    run_parameter: &RunParams<UserParams>,
    monitor: &mut Monitor,
) -> OutputPart1 {
    let swapped = input
        .replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine");
    part_1(prepare(swapped), run_parameter, monitor)
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
