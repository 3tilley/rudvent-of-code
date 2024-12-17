use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use array2d::Array2D;
use rudvent_lib::solution::{SolutionBuilder, StructSolutionBuilder};
use rudvent_lib::solution::execution::{EmptyUserMonitor, EmptyUserParams, Example, RunParams, RuntimeMonitor};
use crate::utils::Array2DExt;

// Update these types to reflect the types you want to use to solve the problems. These
// can be simple types (u64), integers, or your own types
type InputPart1 = Array2D<char>;
type OutputPart1 = usize;
type InputPart2 = InputPart1;
type OutputPart2 = usize;

// These constants hold the answer for the examples, they are used to test your code
const EXAMPLE_1_ANS: OutputPart1 = 1930;
const EXAMPLE_2_ANS: OutputPart2 = 1206;

// This currently only the information about whether the run is an example or not. It may be augmented
type UserParams = EmptyUserParams;
type UserMonitor = EmptyUserMonitor;

// This function is called to prepare the input for part 1
pub fn prepare(input: String) -> InputPart1 {
    Array2D::from_newline_delimited(&input, |x| x.2)
}

#[derive(Debug)]
struct Tile {
    group_id: u32,
    fences: usize,
}

// Implement your solution for part 1 here
pub fn part_1(
    mut input: InputPart1,
    run_parameter: &RunParams<UserParams>,
    monitor: Arc<Mutex<RuntimeMonitor<EmptyUserMonitor>>>,
) -> OutputPart1 {
    let mut tiles: HashMap<(usize, usize), Tile> = HashMap::new();
    let mut group_counter = 0;

    let mut its = input.enumerate_row_major();
    let mut current_field = Vec::new();
    while let Some(((r, c), v)) = its.next() {
        monitor.lock().unwrap().current_progress += 1;
        if !tiles.contains_key(&(r, c)) {
            current_field.push((r, c));
            group_counter += 1;
        }
        while let Some((pos)) = current_field.pop() {
            handle_tile(&input, pos, &mut tiles, group_counter, &mut current_field, &mut monitor.lock().unwrap());
        }
    }

    let mut groups = vec![(0, 0); (group_counter + 1) as usize];
    for tile in tiles.values() {
        let cur = groups.get_mut(tile.group_id as usize).unwrap();
        *cur = (cur.0 + 1, cur.1 + tile.fences);
    }
    let p = input.to_str(|(r, c, v)| {
        format!(" {: >2}", tiles.get(&(r, c)).unwrap().group_id).to_string()
    });
    groups.iter().map(|(a, f)| a * f).sum()

}

fn handle_tile(input: &InputPart1, pos: (usize, usize), tiles: &mut HashMap<(usize, usize), Tile>, group_id: u32, current_field: &mut Vec<(usize, usize)>, runtime_monitor: &mut RuntimeMonitor<EmptyUserMonitor>) {
    let mut fences = 4;
    let v = input.get(pos.0, pos.1).unwrap();
    for (n_r, n_c, n_v) in input.neighbours_iter(pos, false) {
        if v == n_v {
            fences -= 1;
            if !tiles.contains_key(&(n_r, n_c)) {
                runtime_monitor.current_progress += 1;
                current_field.push((n_r, n_c));
            }
        }
    }
    tiles.insert(pos, Tile { group_id, fences });
}

fn handle_tile_2(input: &InputPart1, pos: (usize, usize), tiles: &mut HashMap<(usize, usize), Tile>, group_id: u32, current_field: &mut Vec<(usize, usize)>, runtime_monitor: &mut RuntimeMonitor<EmptyUserMonitor>) {
    let mut fences = 4;
    let v = input.get(pos.0, pos.1).unwrap();
    for (n_r, n_c, n_v) in input.neighbours_iter(pos, false) {
        if v == n_v {
            fences -= 1;
            if !tiles.contains_key(&(n_r, n_c)) {
                runtime_monitor.current_progress += 1;
                current_field.push((n_r, n_c));
            }
        }
    }
    tiles.insert(pos, Tile { group_id, fences });
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
