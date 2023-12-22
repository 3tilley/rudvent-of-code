// TODO: See if this is faster by checking pure indexes instead of the try_fold method
use crate::utils::Array2DExt;
use array2d::Array2D;
use rudvent_lib::solution::execution::{
    EmptyUserMonitor, EmptyUserParams, Example, RunParams, RuntimeMonitor,
};
use rudvent_lib::solution::{SolutionBuilder, StructSolutionBuilder};
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use tracing::info;
use tracing::instrument::WithSubscriber;

// Update these types to reflect the types you want to use to solve the problems. These
// can be simple types (u64), integers, or your own types
type InputPart1 = Vec<Array2D<bool>>;
type OutputPart1 = usize;
type InputPart2 = InputPart1;
type OutputPart2 = usize;

// These constants hold the answer for the examples, they are used to test your code
const EXAMPLE_1_ANS: OutputPart1 = 405;
const EXAMPLE_2_ANS: OutputPart2 = 400;

// This currently only the information about whether the run is an example or not. It may be augmented
type UserParams = EmptyUserParams;
type UserMonitor = EmptyUserMonitor;

// This function is called to prepare the input for part 1
pub fn prepare(input: String) -> InputPart1 {
    let blocks = input.split("\n\n").map(|block| {
        let block_iter: Vec<Vec<bool>> = block
            .trim()
            .lines()
            .map(|line| {
                let rows = line.len();
                (line.chars().map(|c| c == '#').collect())
            })
            .collect();
        Array2D::from_rows(&block_iter).unwrap()
    });
    blocks.collect()
}
fn check_vecs(left: &[bool], right: &[bool], max: usize) -> Option<usize> {
    left.iter().zip(right.iter()).try_fold(0, |acc, (l, r)| {
        let new_acc = if l != r { (acc + 1) } else { acc };
        (new_acc <= max).then_some(new_acc)
    })
}

fn reflections_for_block(block: &Array2D<bool>, smudges: usize) -> usize {
    info!("{:?}", block);
    let mut rows = block.as_rows();
    for row in 0..rows.len() {
        let mut differences = Some(0);
        // For the rows moving upwards we actually start with the row below current, as the first step is an offset
        let mut prev_row = row + 1;
        let mut post_row = row;
        while let (Some(row_a), Some(row_b), Some(diffs)) = (
            block.offset_row(prev_row, -1),
            block.offset_row(post_row, 1),
            differences,
        ) {
            info!("Checking row: {} and row: {}", row_a, row_b);
            info!("row_a: {}, {:?}", row_a, rows[row_a]);
            info!("row_b: {}, {:?}", row_b, rows[row_b]);
            prev_row = row_a;
            post_row = row_b;
            differences =
                check_vecs(&rows[row_a], &rows[row_b], smudges - diffs).map(|d| d + diffs);
            info!("Differences: {:?}", differences);
        }
        if differences.filter(|d| d == &smudges).is_some() && post_row != row {
            return 100 * (row + 1);
        }
    }
    let cols = block.as_columns();
    for col in 0..cols.len() {
        let mut differences = Some(0);
        let mut prev_col = col + 1;
        let mut post_col = col;
        while let (Some(col_a), Some(col_b), Some(diffs)) = (
            block.offset_col(prev_col, -1),
            block.offset_col(post_col, 1),
            differences,
        ) {
            info!("Checking col: {} and col: {}", col_a, col_b);
            info!("col_a: {}, {:?}", col_a, cols[col_a]);
            info!("col_b: {}, {:?}", col_b, cols[col_b]);
            prev_col = col_a;
            post_col = col_b;
            differences =
                check_vecs(&cols[col_a], &cols[col_b], smudges - diffs).map(|d| d + diffs);
        }
        if differences.filter(|d| d == &smudges).is_some() && post_col != col {
            return col + 1;
        }
    }
    unreachable!("Found no reflections")
}

// Implement your solution for part 1 here
pub fn part_1(
    mut input: InputPart1,
    run_parameter: &RunParams<UserParams>,
    monitor: Arc<Mutex<RuntimeMonitor<EmptyUserMonitor>>>,
) -> OutputPart1 {
    input
        .iter()
        .map(|block| {
            let res = reflections_for_block(block, 0);
            info!("Reflection value: {}", res);
            res
        })
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
        .iter()
        .map(|block| {
            let res = reflections_for_block(block, 1);
            info!("Reflection value: {}", res);
            res
        })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_vec() {
        let left = &[true, true, false, false, false];
        let right = &[true, true, true, false, true];
        assert_eq!(check_vecs(left, left, 0), Some(0));
        assert_eq!(check_vecs(left, right, 1), None);
        assert_eq!(check_vecs(left, right, 2), Some(2));
    }
}
