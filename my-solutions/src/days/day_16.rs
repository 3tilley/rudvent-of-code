use std::sync::{Arc, Mutex};
use array2d::Array2D;
use rudvent_lib::solution::{SolutionBuilder, StructSolutionBuilder};
use rudvent_lib::solution::execution::{EmptyUserMonitor, EmptyUserParams, Example, RunParams, RuntimeMonitor};
use crate::days::day_16::Tile::{DownMirror, HorizSplit, UpMirror, VertSplit};
use crate::utils::Array2DExt;

// Update these types to reflect the types you want to use to solve the problems. These
// can be simple types (u64), integers, or your own types
type InputPart1 = Map;
type OutputPart1 = usize;
type InputPart2 = InputPart1;
type OutputPart2 = usize;

// These constants hold the answer for the examples, they are used to test your code
const EXAMPLE_1_ANS: OutputPart1 = 46;
const EXAMPLE_2_ANS: OutputPart2 = 0;

// This currently only the information about whether the run is an example or not. It may be augmented
type UserParams = EmptyUserParams;
type UserMonitor = EmptyUserMonitor;

pub struct Map {
    map: Array2D<Tile>
}

enum Tile {
    Empty,
    UpMirror,
    DownMirror,
    HorizSplit,
    VertSplit,
}

enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn offset(&self) -> (i8, i8) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1,0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }
}
struct BeamFront {
    offset: (i8, i8),
}
impl BeamFront {
    fn direction(&self) -> Direction {
        match self.offset {
            (-1, 0) => Direction::Up,
            (1, 0) => Direction::Down,
            (0, -1) => Direction::Left,
            (0, 1) => Direction::Right,
            _ => unreachable!("Invalid beam")
        }
    }

    fn from_dir
}

impl Tile {
    fn from_char(c: char) -> Tile {
        match c {
            '.' => Tile::Empty,
            '|' => VertSplit,
            '-' => HorizSplit,
            '\\' => DownMirror,
            '/' => UpMirror,
            _ => unreachable!("Unrecognised char"),
        }
    }

    fn interact(&self, light: BeamFront) -> (BeamFront, Option<BeamFront>) {
        match self {
            Tile::Empty => (light, None),
            UpMirror => {match light.direction() {
                Direction::Up => (BeamFront)
                Direction::Down => {}
                Direction::Left => {}
                Direction::Right => {}
            }
            }
            DownMirror => {}
            HorizSplit => {}
            VertSplit => {}
        }
    }
}

// This function is called to prepare the input for part 1
pub fn prepare(input: String) -> InputPart1 {
    let map = Array2D::from_newline_delimited(&input, Tile::from_char);
    Map { map }
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
