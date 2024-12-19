use std::collections::{HashSet, VecDeque};
use std::env::consts;
use std::sync::{Arc, Mutex};
use array2d::Array2D;
use rudvent_lib::solution::{SolutionBuilder, StructSolutionBuilder};
use rudvent_lib::solution::execution::{EmptyUserMonitor, EmptyUserParams, Example, RunParams, RuntimeMonitor};

use crate::utils::Array2DExt;

// Update these types to reflect the types you want to use to solve the problems. These
// can be simple types (u64), integers, or your own types
type InputPart1 = Vec<(usize, usize)>;
type OutputPart1 = usize;
type InputPart2 = InputPart1;
type OutputPart2 = String;

// These constants hold the answer for the examples, they are used to test your code
const EXAMPLE_1_ANS: OutputPart1 = 22;
const EXAMPLE_2_ANS: &'static str = "6,1";

// This currently only the information about whether the run is an example or not. It may be augmented
type UserParams = EmptyUserParams;
type UserMonitor = EmptyUserMonitor;

// This function is called to prepare the input for part 1
pub fn prepare(input: String) -> InputPart1 {
    input.lines().map(|s| {
        let (fst, snd) = s.split_once(",").unwrap();
        (snd.parse().unwrap(), fst.parse().unwrap())
    }).collect()
}

const DIM_EX: usize = 7;
const DIM_FULL: usize = 71;

const ITS_EX: usize = 12;
const ITS_FULL: usize = 1024;

fn bi_bfs(arr: &Array2D<bool>, start: (usize, usize), end: (usize, usize)) -> Option<(usize, Array2D<(Option<usize>, Option<usize>)>, (usize, usize))> {
    let mut visits: Array2D<(Option<usize>, Option<usize>)> = Array2D::filled_with((None, None), arr.num_rows(), arr.num_columns());
    *visits.get_mut(start.0, start.1).unwrap() = (Some(0), None);
    *visits.get_mut(end.0, end.1).unwrap() = (None, Some(0));
    let mut forwards = true;
    let mut forward_vec = VecDeque::from([(start, 0)]);
    let mut backward_vec = VecDeque::from([(end, 0)]);
    // let mut step_count = 0;
    let mut counter = 0;
    while (forward_vec.len() > 0 && backward_vec.len() > 0) {
        println!("\n\n{}", visits.to_str(|(r, c, (f, b))| {
            let s = if let Some(_f) = f {
                format!("{: <2}", _f)
            } else if let Some(_b) = b {
                format!("{: <2}", _b)
            } else if *arr.get(r, c).unwrap() {
                ". ".to_string()
            } else {
                "# ".to_string()
            };
            s.to_string()
        }));
        println!("\nForward: {forward_vec:?}");
        println!("\nForward: {backward_vec:?}");

        // This is stupid, do it better
        let mut new_forward = forward_vec.split_off(forward_vec.len());
        while let Some((pos, cost)) = forward_vec.pop_front() {
            for (r, c, v) in arr.neighbours_iter(pos, false) {
                if *v {
                    let n = visits.get_mut(r, c).unwrap();
                    match n {
                        (_, Some(back_cost)) => {
                            let _back_cost = *back_cost;
                            *n = (Some(cost + 1), None);
                            return Some((_back_cost + cost + 1, visits, (r, c)))
                        },
                        (None, _) => {
                            
                            *n = (Some(cost + 1), None);
                            new_forward.push_back(((r,c ), cost + 1));
                        },
                        _ => (),
                    }
                }
            }
        }

        let mut new_backward = backward_vec.split_off(backward_vec.len());
        while let Some((pos, cost)) = backward_vec.pop_front() {
            for (r, c, v) in arr.neighbours_iter(pos, false) {
                if *v {
                    let n = visits.get_mut(r, c).unwrap();
                    match n {
                        (Some(forward_cost), _) =>{
                            let _forward_cost = *forward_cost;
                            *n = (None, Some(cost + 1));
                            return Some((_forward_cost + cost + 1, visits, (r, c)))
                        },
                        (_, None) => {
                            *n = (None, Some(cost + 1));
                            new_backward.push_back(((r,c ), cost + 1));
                        },
                        _ => (),
                    }
                }
            }
        }
        forward_vec = new_forward;
        backward_vec = new_backward;
        // counter += 1;
        // if counter > 10 {
        //     break;
        // }
    }
    None
}

fn extract_path(visits: Array2D<(Option<usize>, Option<usize>)>, cost: usize) -> HashSet<(usize, usize)> {
    todo!()
}

// Implement your solution for part 1 here
pub fn part_1(
    mut input: InputPart1,
    run_parameter: &RunParams<UserParams>,
    monitor: Arc<Mutex<RuntimeMonitor<EmptyUserMonitor>>>,
) -> OutputPart1 {
    let dim = if run_parameter.is_example {DIM_EX} else {DIM_FULL};
    let its = if run_parameter.is_example {ITS_EX} else {ITS_FULL};
    let mut arr = Array2D::filled_with(true, dim, dim);
    for (r, c) in input.into_iter().take(its) {
        *arr.get_mut(r, c).unwrap() = false;
    }
    bi_bfs(&arr, (0,0), (dim - 1, dim -1)).unwrap().0
}

// If the puzzle requires a different input for part 2, this function can be updated
pub fn prepare_2(input: String) -> InputPart2 {
    prepare(input)
}

pub fn part_2(
    mut input: InputPart1,
    run_parameter: &RunParams<UserParams>,
    monitor: Arc<Mutex<RuntimeMonitor<EmptyUserMonitor>>>,
) -> OutputPart2 {
    let output = part_1(input, run_parameter, monitor);
    format!("{}", output).to_string()
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
        Example::Value(EXAMPLE_2_ANS.to_string()),
    );
    Box::new(sol)
}
