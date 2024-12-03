use std::sync::{Arc, Mutex};
use num_traits::real::Real;
use rudvent_lib::solution::{SolutionBuilder, StructSolutionBuilder};
use rudvent_lib::solution::execution::{EmptyUserMonitor, EmptyUserParams, Example, RunParams, RuntimeMonitor};

// Update these types to reflect the types you want to use to solve the problems. These
// can be simple types (u64), integers, or your own types
type InputPart1 = Vec<Vec<usize>>;
type OutputPart1 = usize;
type InputPart2 = InputPart1;
type OutputPart2 = usize;

// These constants hold the answer for the examples, they are used to test your code
const EXAMPLE_1_ANS: OutputPart1 = 2;
const EXAMPLE_2_ANS: OutputPart2 = 4;

// This currently only the information about whether the run is an example or not. It may be augmented
type UserParams = EmptyUserParams;
type UserMonitor = EmptyUserMonitor;

// This function is called to prepare the input for part 1
pub fn prepare(input: String) -> InputPart1 {
    input.lines().map(|line| line.split_whitespace().map(|s| s.parse().unwrap()).collect()).collect()
}

fn in_bounds(fst: OutputPart2, snd: OutputPart2, is_inc: bool) -> bool {
    if fst.abs_diff(snd) > 3 || fst.abs_diff(snd) < 1 {
        return false
    } else {
        !(is_inc ^ (fst < snd))
    }
}

// Implement your solution for part 1 here
pub fn part_1(
    mut input: InputPart1,
    run_parameter: &RunParams<UserParams>,
    monitor: Arc<Mutex<RuntimeMonitor<EmptyUserMonitor>>>,
) -> OutputPart1 {
    input.into_iter().filter(|level| {
        let mut it = level.into_iter();
        let fst = it.next().unwrap();
        let snd = it.next().unwrap();
        if !in_bounds(*fst, *snd, snd > fst) { return false;}
        it.try_fold((snd > fst, snd), |acc, v| {
            if in_bounds(*acc.1, *v, acc.0) {
                Ok((acc.0, v))
            } else {
                Err(())
            }
        }).is_ok()
    }).count()
}

// If the puzzle requires a different input for part 2, this function can be updated
pub fn prepare_2(input: String) -> InputPart2 {
    prepare(input)
}

// struct ProblemLocation {
//     err_index: usize,
//     was_inc: bool,
// }

// impl ProblemLocation {
//     fn check_vec(vec: Vec<usize>) -> bool {

//     }
// }

// fn check_iter(it: _, is_inc: bool) -> Option<ProblemLocation> {

//     it.try_fold((in_inc, 1, ))
// }

pub fn part_2(
    mut input: InputPart1,
    run_parameter: &RunParams<UserParams>,
    monitor: Arc<Mutex<RuntimeMonitor<EmptyUserMonitor>>>,
) -> OutputPart1 {
    input.into_iter().filter(|level| {
        let mut it = level.into_iter();
        let fst = it.next().unwrap();
        let snd = it.next().unwrap();
        if !in_bounds(*fst, *snd, snd > fst) { return false;}
        it.try_fold((snd > fst, snd), |acc, v| {
            if in_bounds(*acc.1, *v, acc.0) {
                Ok((acc.0, v))
            } else {
                Err(())
            }
        }).is_ok()
    }).count()
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
