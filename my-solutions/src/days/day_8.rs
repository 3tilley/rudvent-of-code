use core::num;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use rudvent_lib::solution::{SolutionBuilder, StructSolutionBuilder};
use rudvent_lib::solution::execution::{EmptyUserMonitor, EmptyUserParams, Example, RunParams, RuntimeMonitor};

use crate::utils::SparseArray;

struct Map {
    map: HashMap<char, SparseArray<(usize, usize), u8, (isize, isize)>>,
    antennas: HashSet<(usize, usize)>,
}
// Update these types to reflect the types you want to use to solve the problems. These
// can be simple types (u64), integers, or your own types
type InputPart1 = Map;
type OutputPart1 = usize;
type InputPart2 = InputPart1;
type OutputPart2 = usize;

// These constants hold the answer for the examples, they are used to test your code
const EXAMPLE_1_ANS: OutputPart1 = 14;
const EXAMPLE_2_ANS: OutputPart2 = 0;

// This currently only the information about whether the run is an example or not. It may be augmented
type UserParams = EmptyUserParams;
type UserMonitor = EmptyUserMonitor;

// This function is called to prepare the input for part 1
pub fn prepare(input: String) -> InputPart1 {
    let mut letters = HashMap::new();
    let mut antennas = HashSet::new();
    let mut num_rows = 0;
    let mut num_cols = 0;
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c != '.' {
                antennas.insert((row, col));
                letters.entry(c).or_insert(Vec::new()).push(((row, col), 1u8));
                num_cols = num_cols.max(col);
                num_rows = num_cols.max(row);
            }
        }
    }
    let map_iter = letters.into_iter().map(|(key, val)| {
        (key, SparseArray::new(val, (num_rows, num_cols)).unwrap())
    });
    let mut map = HashMap::new();
    for (c, sparse) in map_iter.into_iter() {
        map.insert(c, sparse);
    }
    // let map = HashMap::from(map_iter.into_iter());
    Map {
        map, antennas
    }
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
