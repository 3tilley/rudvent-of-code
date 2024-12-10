use std::sync::{Arc, Mutex};
use rudvent_lib::solution::{SolutionBuilder, StructSolutionBuilder};
use rudvent_lib::solution::execution::{EmptyUserMonitor, EmptyUserParams, Example, RunParams, RuntimeMonitor};
use tracing::debug;

struct Calibration {
    result: usize,
    params: Vec<usize>,
}

// Update these types to reflect the types you want to use to solve the problems. These
// can be simple types (u64), integers, or your own types
type InputPart1 = Vec<Calibration>;
type OutputPart1 = usize;
type InputPart2 = InputPart1;
type OutputPart2 = usize;

// These constants hold the answer for the examples, they are used to test your code
const EXAMPLE_1_ANS: OutputPart1 = 3749;
const EXAMPLE_2_ANS: OutputPart2 = 11387;

// This currently only the information about whether the run is an example or not. It may be augmented
type UserParams = EmptyUserParams;
type UserMonitor = EmptyUserMonitor;

// This function is called to prepare the input for part 1
pub fn prepare(input: String) -> InputPart1 {
    input.lines().map(|s| {
        let (fst, snd) = s.split_once(":").unwrap();
        let result = fst.parse().unwrap();
        let params = snd.trim().split(" ").map(|num| num.parse().unwrap()).collect();
        Calibration { result, params}
    }).collect()
}

enum Operation {
    Add,
    Mul,
}

impl Operation {
    fn calc(self, left: usize, right: usize) -> usize {
        match self {
            Operation::Add => left + right,
            Operation::Mul => left * right
        }
    }
}

fn calc_vec(vec: &[usize], acc: usize, target: usize, inc_concat: bool) -> Option<usize> {
    match & *vec {
        [head, tail @ ..] => {
            if acc > target || head > &target {
                // debug!("Target: {target}. Removed {acc} and {head}");
                return None
            }
            let res = calc_vec(tail, *head + acc, target, inc_concat).or_else(
                || calc_vec(tail, *head * acc, target, inc_concat)
            );
            if inc_concat {
                res.or_else(|| {
                    let new_acc = concat(acc, *head)?;
                    calc_vec(tail, new_acc, target, inc_concat)
                })
            } else {
                res
            }
        }
        &[] => if acc == target {Some(target)} else {None}
    }

}

fn concat(acc: usize, head: usize) -> Option<usize> {
    let power = 10_usize.pow(head.checked_ilog10().unwrap_or(0) + 1);
    let new_acc = power.checked_mul(acc)?;
    let new_acc = new_acc + head;
    // debug!("Concating {acc} {head} to make {new_acc}");
    Some(new_acc)
}

// Implement your solution for part 1 here
pub fn part_1(
    mut input: InputPart1,
    run_parameter: &RunParams<UserParams>,
    monitor: Arc<Mutex<RuntimeMonitor<EmptyUserMonitor>>>,
) -> OutputPart1 {
    input.into_iter().filter_map(|cal| {
        calc_vec(&cal.params[1..], cal.params[0], cal.result, false)
    }).sum()
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
    input.into_iter().filter_map(|cal| {
        calc_vec(&cal.params[1..], cal.params[0], cal.result, true)
    }).sum()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concat() {
        assert_eq!(concat(10, 20), Some(1020));
        assert_eq!(concat(100, 20), Some(10020));
        assert_eq!(concat(1, 20), Some(120));
    }
}