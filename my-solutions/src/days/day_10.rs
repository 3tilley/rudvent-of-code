use std::cmp::max;
use std::ops::Index;
use array2d;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use array2d::Array2D;
use clap::Parser;
use tracing::info;
use rudvent_lib::solution::{SolutionBuilder, StructSolutionBuilder};
use rudvent_lib::solution::execution::{EmptyUserMonitor, EmptyUserParams, Example, RunParams, RuntimeMonitor};

// Update these types to reflect the types you want to use to solve the problems. These
// can be simple types (u64), integers, or your own types
type InputPart1 = Map;
type OutputPart1 = usize;
type InputPart2 = InputPart1;
type OutputPart2 = usize;

// These constants hold the answer for the examples, they are used to test your code
const EXAMPLE_1_ANS: OutputPart1 = 8;
const EXAMPLE_2_ANS: OutputPart2 = 10;

// This currently only the information about whether the run is an example or not. It may be augmented
type UserParams = Params;
type UserMonitor = EmptyUserMonitor;

#[derive(Parser, Clone, Debug, Default)]
pub struct Params {
    #[arg(short, long,action)]
    row_major: bool,
    #[arg(short, long)]
    method: Option<String>,
}

type Piece = ((i8, i8), (i8, i8));

fn piece_from_char(c: char) -> Piece {
    match c {
        '|' => ((-1, 0), (1, 0)),
        '-' => ((0, -1), (0, 1)),
        'L' => ((-1, 0), (0, 1)),
        '7' => ((0, -1), (1, 0)),
        'F' => ((0, 1), (1, 0)),
        'J' => ((0, -1), (-1, 0)),
        '.' => ((0, 0), (0, 0)),
        'S' => ((0,0), (0,0)),
        _ => unreachable!("Unrecognised char")
    }
}

fn next_index(c: char, current_index: (usize, usize), previous_index: (usize, usize)) -> (usize, usize) {
    let piece = piece_from_char(c);
    let first = (current_index.0.checked_add_signed(piece.0.0 as isize).unwrap(), current_index.1.checked_add_signed(piece.0.1 as isize).unwrap());
    let second = (current_index.0.checked_add_signed(piece.1.0 as isize).unwrap(), current_index.1.checked_add_signed(piece.1.1 as isize).unwrap());
    if first == previous_index {
        second
    } else if second == previous_index {
        first
    }
    else {
        unreachable!("It must be one or the other")
    }
}

fn char_touches(c: char, offset: (i8, i8)) -> bool {
    let piece = piece_from_char(c);
    (-piece.0.0, -piece.0.1) == offset || (-piece.1.0, -piece.1.1) == offset
}

pub struct Map {
    map: Array2D<char>,
    rows: usize,
    cols: usize,
}

// This function is called to prepare the input for part 1
pub fn prepare(input: String) -> InputPart1 {
    let elements = input.lines().map(|line| line.chars().collect()).collect::<Vec<Vec<char>>>();
    let rows = elements.len();
    let cols = elements[0].len();
    let map = array2d::Array2D::from_iter_row_major(&mut elements.into_iter().flatten(), rows, cols).unwrap();
    Map {
        map,
        rows,
        cols
    }
}

// Implement your solution for part 1 here
pub fn part_1(
    mut input: InputPart1,
    run_parameter: &RunParams<UserParams>,
    monitor: Arc<Mutex<RuntimeMonitor<EmptyUserMonitor>>>,
) -> OutputPart1 {
    let s = input.map.enumerate_row_major().find(|((row_col), char)| **char == 'S').unwrap();
    let mut previous = s.0;
    let touching = &[(0,-1), (1, 0), (1, 0), (0, -1)].iter().filter_map(|offset| {
        let new_row = previous.0.checked_add_signed(offset.0).and_then(|row| (row < input.rows).then_some(row));
        let new_col = previous.1.checked_add_signed(offset.1).and_then(|col| (col < input.cols).then_some(col));
        new_col.and_then(|col| new_row.map(|row| (row, col)))
    } ).next().unwrap();
    let mut current = *touching;
    let mut steps = 1;
    loop {
        let new_char = input.map.index(current);
        if new_char == &'S' { break }
        let temp = current;
        current = next_index(*new_char, current, previous);
        previous = temp;
        steps += 1;
    }
    (steps + 1) / 2
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
