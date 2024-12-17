use std::sync::{Arc, Mutex};
use array2d::Array2D;
use crate::utils::Array2DExt;
use rudvent_lib::solution::{SolutionBuilder, StructSolutionBuilder};
use rudvent_lib::solution::execution::{EmptyUserMonitor, EmptyUserParams, Example, RunParams, RuntimeMonitor};

// Update these types to reflect the types you want to use to solve the problems. These
// can be simple types (u64), integers, or your own types
type InputPart1 = Array2D<char>;
type OutputPart1 = usize;
type InputPart2 = InputPart1;
type OutputPart2 = usize;

// These constants hold the answer for the examples, they are used to test your code
const EXAMPLE_1_ANS: OutputPart1 = 18;
const EXAMPLE_2_ANS: OutputPart2 = 9;

// This currently only the information about whether the run is an example or not. It may be augmented
type UserParams = EmptyUserParams;
type UserMonitor = EmptyUserMonitor;

// This function is called to prepare the input for part 1
pub fn prepare(input: String) -> InputPart1 {
    Array2D::from_newline_delimited(&input, |s| s.2)
}

// Implement your solution for part 1 here
pub fn part_1(
    mut input: InputPart1,
    run_parameter: &RunParams<UserParams>,
    monitor: Arc<Mutex<RuntimeMonitor<EmptyUserMonitor>>>,
) -> OutputPart1 {
    let xmas = vec!['X', 'M', 'A', 'S'];
    // println!("Row offset {:?}", input.direction_iter((0,0), (1,0)).collect::<Vec<_>>());
    input.enumerate_column_major().map(|(pos, v)| {
        match v {
            'X' => {
                // println!("{}", v);
                input.offset_iter(pos, true).filter(|offset| {
                    // println!("Pos: {:?}, Offset: {:?}", pos, offset);
                    let word = input.direction_iter(pos, *offset).take(4);
                    // println!("{:?}", &word.collect::<Vec<_>>());
                    // let samx = "samx".chars();
                    word.eq(xmas.iter())
                    // false
                }).count()
            },
            _ => 0
        }
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
