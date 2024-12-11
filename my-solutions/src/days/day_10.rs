use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::sync::{Arc, Mutex};
use array2d::Array2D;
use nom::combinator::into;
use rudvent_lib::solution::{SolutionBuilder, StructSolutionBuilder};
use rudvent_lib::solution::execution::{EmptyUserMonitor, EmptyUserParams, Example, RunParams, RuntimeMonitor};
use crate::utils::Array2DExt;

// Update these types to reflect the types you want to use to solve the problems. These
// can be simple types (u64), integers, or your own types
type InputPart1 = Array2D<u16>;
type OutputPart1 = usize;
type InputPart2 = InputPart1;
type OutputPart2 = usize;

// These constants hold the answer for the examples, they are used to test your code
const EXAMPLE_1_ANS: OutputPart1 = 36;
const EXAMPLE_2_ANS: OutputPart2 = 0;

// This currently only the information about whether the run is an example or not. It may be augmented
type UserParams = EmptyUserParams;
type UserMonitor = EmptyUserMonitor;

// This function is called to prepare the input for part 1
pub fn prepare(input: String) -> InputPart1 {
    Array2D::from_newline_delimited(&input, |s, _, _| s.to_digit(10).unwrap().try_into().unwrap())
}

// Implement your solution for part 1 here
pub fn part_1(
    mut input: InputPart1,
    run_parameter: &RunParams<UserParams>,
    monitor: Arc<Mutex<RuntimeMonitor<EmptyUserMonitor>>>,
) -> OutputPart1 {
    let markers_iter = input.enumerate_column_major().filter_map(|((row, col), e)| {
       if *e == 0 {
           Some((row, col))
       } else {
           None
       }
    }).enumerate().map(|(i, pos)| (pos, i));
    let mut prev_markers: HashMap<(usize, usize), usize> = markers_iter.collect();
    let mut markers = HashMap::new();
    let mut id_group: HashMap<usize, usize> = prev_markers.values().map(|&v| (v, v)).collect();
    let mut group_count: HashMap<usize, HashSet<usize>> = prev_markers.values().map(|&v| (v, HashSet::from([v]))).collect();
    for height in 1u16..=9 {
        for (pos, id) in prev_markers {
            for o in input.offset_iter(pos, false) {
                if let Some((r, c)) =  input.offset(pos, o) {
                    if let Some(x) = input.get(r, c) {
                        if x == &height {
                            if let Some(old_id) = markers.insert((r, c), id) {
                                id_group.insert(old_id, id);
                                group_count.entry(id).and_modify(|hs| {hs.insert(old_id);});
                            }
                        }
                    }
                }
            }
        }
        prev_markers = HashMap::new();
        for (pos, id) in markers {
            prev_markers.insert(pos, id_group[&id]);
        }
        markers = HashMap::new();
        println!("{prev_markers:?}");
        let s = input.to_str(|(v, r, c)| prev_markers.get(&(r, c)).map(|v| v.to_string()).unwrap_or(".".to_string()));
        println!("{s}");
    }
    let mut acc = 0;
    for (pos, group_id) in prev_markers {
        acc += group_count[&group_id].len();
    }
    println!("{group_count:?}");
    acc
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
