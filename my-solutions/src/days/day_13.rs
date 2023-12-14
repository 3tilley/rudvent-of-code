use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use array2d::Array2D;
use tracing::info;
use tracing::instrument::WithSubscriber;
use rudvent_lib::solution::{SolutionBuilder, StructSolutionBuilder};
use rudvent_lib::solution::execution::{EmptyUserMonitor, EmptyUserParams, Example, RunParams, RuntimeMonitor};

// Update these types to reflect the types you want to use to solve the problems. These
// can be simple types (u64), integers, or your own types
type InputPart1 = Vec<Array2D<bool>>;
type OutputPart1 = usize;
type InputPart2 = InputPart1;
type OutputPart2 = usize;

// These constants hold the answer for the examples, they are used to test your code
const EXAMPLE_1_ANS: OutputPart1 = 405;
const EXAMPLE_2_ANS: OutputPart2 = 0;

// This currently only the information about whether the run is an example or not. It may be augmented
type UserParams = EmptyUserParams;
type UserMonitor = EmptyUserMonitor;

// This function is called to prepare the input for part 1
pub fn prepare(input: String) -> InputPart1 {
    let blocks = input.split("\n\n").map(|block| {
        let block_iter: Vec<Vec<bool>> = block.trim().lines().map(|line| {
            let rows = line.len();
            (line.chars().map(|c| c == '#').collect())
        }).collect();
        Array2D::from_rows(&block_iter).unwrap()
    });
    blocks.collect()
}

fn reflections_for_block(block: &Array2D<bool>) -> usize {
    info!("{:?}", block);
    let mut rows = block.as_rows();
    for row in 1..rows.len() {
        let mut found = false;
        // For the rows moving upwards we actually start with the row below current, as the first step is an offset
        let mut prev_row = row + 1;
        let mut post_row = row;
        while let (Some(row_a), Some(row_b)) = (block.offset_row(prev_row, -1), block.offset_row(post_row, 1)) {
            found = rows[row_a] == rows[row_b];
            prev_row = row_a;
            post_row = row_b;
            if !found { break; }
        }
        if found {
            return 100 * (row + 1)
        }
    }
    let mut found = false;
    let cols = block.as_columns();
    for col in 1..cols.len() {
        let mut prev_col = col + 1;
        let mut post_col = col;
        while let (Some(col_a), Some(col_b)) = (block.offset_col(prev_col, -1), block.offset_col(post_col, 1)) {
            info!("Checking col: {} and col: {}", col_a, col_b);
            info!("col_a: {}, {:?}", col_a, cols[col_a]);
            info!("col_b: {}, {:?}", col_b, cols[col_b]);
            found = cols[col_a] == cols[col_b];
            prev_col = col_a;
            post_col = col_b;
            if !found {break;}
        }
        if found {
            return col + 1
        }
    }
    unreachable!("Found no reflections")
}
trait Array2DExt<T> {
    fn row(&self, index:usize) -> Vec<&T>;
    fn col(&self, index:usize) -> Vec<&T>;

    fn offset_row<V: TryInto<isize>>(&self, index: usize, offset: V) -> Option<usize> where <V as TryInto<isize>>::Error: Debug;
    fn offset_col<V: TryInto<isize>>(&self, index: usize, offset: V) -> Option<usize> where <V as TryInto<isize>>::Error: Debug;
}

impl<T> Array2DExt<T> for Array2D<T> {
    fn row(&self, row_index: usize) -> Vec<&T> {
        self.row_iter(row_index).expect(&format!("Row {} out of bounds", row_index)).collect()
    }

    fn col(&self, col_index: usize) -> Vec<&T> {
        self.column_iter(col_index).expect(&format!("Column {} out of bounds", col_index)).collect()
    }
    fn offset_row<V: TryInto<isize>>(&self, index: usize, offset: V) -> Option<usize> where <V as TryInto<isize>>::Error: Debug {
        index.checked_add_signed(offset.try_into().unwrap()).filter(|row| row < &self.column_len())
    }
    fn offset_col<V: TryInto<isize>>(&self, index: usize, offset: V) -> Option<usize> where <V as TryInto<isize>>::Error: Debug {
        index.checked_add_signed(offset.try_into().unwrap()).filter(|col| col < &self.row_len())
    }
}
// Implement your solution for part 1 here
pub fn part_1(
    mut input: InputPart1,
    run_parameter: &RunParams<UserParams>,
    monitor: Arc<Mutex<RuntimeMonitor<EmptyUserMonitor>>>,
) -> OutputPart1 {
    input.iter().map(reflections_for_block).sum()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array2d_row_offset() {
        let arr = Array2D::filled_with(0,4,1);
        assert_eq!(arr.offset_row(3, -1), Some(2));
        assert_eq!(arr.offset_row(0, -1), None);
        assert_eq!(arr.offset_row(3, 1), None);
        assert_eq!(arr.offset_row(2, 1), Some(3));
    }
    #[test]
    fn test_array2d_col_offset() {
        let arr = Array2D::filled_with(0,1,4);
        assert_eq!(arr.offset_col(3, -1), Some(2));
        assert_eq!(arr.offset_col(0, -1), None);
        assert_eq!(arr.offset_col(3, 1), None);
        assert_eq!(arr.offset_col(2, 1), Some(3));
    }
}
