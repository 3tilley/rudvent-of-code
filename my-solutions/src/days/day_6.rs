use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use array2d::Array2D;
use rudvent_lib::solution::{SolutionBuilder, StructSolutionBuilder};
use rudvent_lib::solution::execution::{EmptyUserMonitor, EmptyUserParams, Example, RunParams, RuntimeMonitor};

use crate::utils::Array2DExt;

#[derive(Copy, Clone)]
enum Tile {
    Empty,
    Obstacle,
}

struct State {
    map: Array2D<Tile>,
    location: (usize, usize),
    facing: (i8, i8),
    start_pos: (usize, usize),
    start_facing: (i8, i8),
}

fn check_90(facing: (i8, i8)) -> (i8, i8) {
    match facing {
        (1, 0) => (0, -1),
        (0, -1) => (-1, 0),
        (-1, 0) => (0, 1),
        (0, 1) => (1, 0),
        _ => panic!("Unexpected facing: {:?}", facing)
    }
}

enum GuardMove {
    MoveTo((usize, usize), (i8, i8)),
    ExitAt((usize, usize)),
}

impl State {
    fn next_spot(&self) -> GuardMove {
        let mut face = self.facing;
        loop {
            let next_step = self.map.offset(self.location, face);
            match next_step {
                None => return GuardMove::ExitAt(self.location),
                Some(next) => {
                    match self.map.get(next.0, next.1).unwrap() {
                        Tile::Empty => return GuardMove::MoveTo(next, face),
                        Tile::Obstacle => face = check_90(face),
                    }
                },
            }
        }
    }
}
// Update these types to reflect the types you want to use to solve the problems. These
// can be simple types (u64), integers, or your own types
type InputPart1 = State;
type OutputPart1 = usize;
type InputPart2 = InputPart1;
type OutputPart2 = usize;

// These constants hold the answer for the examples, they are used to test your code
const EXAMPLE_1_ANS: OutputPart1 = 41;
const EXAMPLE_2_ANS: OutputPart2 = 6;

// This currently only the information about whether the run is an example or not. It may be augmented
type UserParams = EmptyUserParams;
type UserMonitor = EmptyUserMonitor;

// This function is called to prepare the input for part 1
pub fn prepare(input: String) -> InputPart1 {
    use std::cell::RefCell;
    use std::rc::Rc;

    let location = Rc::new(RefCell::new((0, 0)));
    let map = Array2D::from_newline_delimited(&input, |c, row, col| -> Tile {
        match c {
            '.' => Tile::Empty,
            '#' => Tile::Obstacle,
            '^' => {
                *location.borrow_mut() = (row, col);
                Tile::Empty
            },
            _ => panic!("Unrecognised char {}", c)
        }
    });
    let location = *location.borrow();
    State {map, location, facing: (-1, 0), start_facing: (-1, 0), start_pos: location}
}

// Implement your solution for part 1 here
pub fn part_1(
    mut input: InputPart1,
    run_parameter: &RunParams<UserParams>,
    monitor: Arc<Mutex<RuntimeMonitor<EmptyUserMonitor>>>,
) -> OutputPart1 {
    let mut spots = HashSet::from([input.location]);
    while let GuardMove::MoveTo(new_pos, facing) = input.next_spot() {
        spots.insert(new_pos);
        input.facing = facing;
        input.location = new_pos;
    }
    spots.len()
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
    let mut spots = HashSet::new();
    while let GuardMove::MoveTo(new_pos, facing) = input.next_spot() {
        spots.insert(new_pos);
        input.facing = facing;
        input.location = new_pos;
    }
    spots.iter().filter(|pos| {
        let mut positions = HashSet::new();
        let mut new_map = input.map.clone();
        new_map.set(pos.0, pos.1, Tile::Obstacle);
        let new_input = State { map: new_map, start_facing: input.start_facing, start_pos: input.start_pos, location: input.start_pos, facing: input.start_facing };
        while let GuardMove::MoveTo(new_pos, facing) = input.next_spot() {
            if positions.contains(&(new_pos, facing)) {
                return true
            }
            positions.insert((new_pos, facing));
            input.facing = facing;
            input.location = new_pos;
        }
        false
    }).count()
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
