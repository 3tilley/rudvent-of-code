use array2d::Array2D;
use rudvent_lib::solution::execution::{
    EmptyUserMonitor, EmptyUserParams, Example, RunParams, RuntimeMonitor,
};
use rudvent_lib::solution::{SolutionBuilder, StructSolutionBuilder};
use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

// Update these types to reflect the types you want to use to solve the problems. These
// can be simple types (u64), integers, or your own types
type InputPart1 = Universe;
type OutputPart1 = usize;
type InputPart2 = InputPart1;
type OutputPart2 = usize;

// These constants hold the answer for the examples, they are used to test your code
const EXAMPLE_1_ANS: OutputPart1 = 374;
const EXAMPLE_2_ANS: OutputPart2 = 0;

// This currently only the information about whether the run is an example or not. It may be augmented
type UserParams = EmptyUserParams;
type UserMonitor = EmptyUserMonitor;

pub struct Universe {
    galaxies: Vec<(usize, usize)>,
}

impl Universe {
    fn from_str(input: String, expansion: usize) -> Universe {
        let mut cols = HashSet::<usize>::new();
        let mut rows = HashSet::<usize>::new();
        let mut original_galaxies = Vec::new();
        let mut max_row = 0;
        let mut max_col = 0;
        for (row, line) in input.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                if c == '#' {
                    cols.insert(col);
                    rows.insert(row);
                    original_galaxies.push((row, col));
                }
                max_col = max(max_col, col);
            }
            max_row = max(max_row, row);
        }
        let row_map: Vec<usize> = (0..max_row * 2)
            .scan(0, |acc, val| {
                if rows.contains(&val) {
                    *acc += 1
                } else {
                    *acc += expansion
                }
                Some(*acc)
            })
            .collect();
        let col_map: Vec<usize> = (0..max_col * 2)
            .scan(0, |acc, val| {
                if cols.contains(&val) {
                    *acc += 1
                } else {
                    *acc += expansion
                }
                Some(*acc)
            })
            .collect();

        let galaxies = original_galaxies
            .into_iter()
            .map(|(row, col)| (row_map[row], col_map[col]))
            .collect();
        Universe { galaxies }
    }
}

// This function is called to prepare the input for part 1
pub fn prepare(input: String) -> InputPart1 {
    Universe::from_str(input, 2)
}

// Implement your solution for part 1 here
pub fn part_1(
    mut input: InputPart1,
    run_parameter: &RunParams<UserParams>,
    monitor: Arc<Mutex<RuntimeMonitor<EmptyUserMonitor>>>,
) -> OutputPart1 {
    let map = HashMap::<(usize, usize), usize>::new();
    input
        .galaxies
        .iter()
        .enumerate()
        .map(|(gal1, (row1, col1))| {
            input
                .galaxies
                .iter()
                .enumerate()
                .filter_map(move |(gal2, (row2, col2))| {
                    (gal1 < gal2).then_some(row1.abs_diff(*row2) + col1.abs_diff(*col2))
                })
        })
        .flatten()
        .sum()
}

// If the puzzle requires a different input for part 2, this function can be updated
pub fn prepare_2(input: String) -> InputPart2 {
    Universe::from_str(input, 1_000_000)
}

pub fn part_2(
    mut input: InputPart1,
    run_parameter: &RunParams<UserParams>,
    monitor: Arc<Mutex<RuntimeMonitor<EmptyUserMonitor>>>,
) -> OutputPart1 {
    part_1(input, run_parameter, monitor)
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
