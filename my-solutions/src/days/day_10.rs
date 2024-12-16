use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::ops::AddAssign;
use std::sync::{Arc, Mutex};
use array2d::Array2D;
use clap::Parser;
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
const EXAMPLE_2_ANS: OutputPart2 = 81;

// Clone + Debug + Default + Parser + Send
#[derive(Clone, Debug, Default, Parser)]
struct UserParams {
    olek: bool,
}

// This currently only the information about whether the run is an example or not. It may be augmented
// type UserParams = EmptyUserParams;
type UserMonitor = EmptyUserMonitor;

// This function is called to prepare the input for part 1
pub fn prepare(input: String) -> InputPart1 {
    Array2D::from_newline_delimited(&input, |s, _, _| s.to_digit(10).unwrap().try_into().unwrap())
}

pub fn part_1(
    mut input: InputPart1,
    run_parameter: &RunParams<UserParams>,
    monitor: Arc<Mutex<RuntimeMonitor<EmptyUserMonitor>>>,
) -> OutputPart1 {
    // if run_parameter.user_params.olek {
    if false {
        part_1_olek(input, run_parameter, monitor)
    } else {
        part_1_max(input, run_parameter, monitor)
    }
}
// Implement your solution for part 1 here
pub fn part_1_max(
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
    }).enumerate().map(|(i, pos)| (pos, HashSet::from([i])));
    let mut prev_markers: HashMap<(usize, usize), HashSet<usize>> = markers_iter.collect();
    let mut markers: HashMap<(usize, usize), HashSet<usize>> = HashMap::new();
    // let mut id_group: HashMap<usize, usize> = prev_markers.values().map(|&v| (v, v)).collect();
    // let mut group_count: HashMap<usize, HashSet<usize>> = prev_markers.values().map(|&v| (v, HashSet::from([v]))).collect();
    for height in 1u16..=9 {
        for (pos, id) in prev_markers {
            for o in input.offset_iter(pos, false) {
                if let Some((r, c)) =  input.offset(pos, o) {
                    if let Some(x) = input.get(r, c) {
                        if x == &height {
                            markers.entry((r, c)).or_default().extend(id.clone());
                        }
                    }
                }
            }
        }
        // prev_markers = HashMap::new();
        prev_markers = markers;
        // for (pos, id) in markers {
        //     prev_markers.insert(pos, id_group[&id]);
        // }
        markers = HashMap::new();
        // println!("{prev_markers:?}");
        // let s = input.to_str(|(v, r, c)| prev_markers.get(&(r, c)).map(|v| v.to_string()).unwrap_or(".".to_string()));
        // println!("{s}");
    }
    let mut acc = 0;
    for (pos, vec) in prev_markers {
        acc += vec.len();
    }
    // println!("{group_count:?}");
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
    let markers_iter = input.enumerate_column_major().filter_map(|((row, col), e)| {
        if *e == 0 {
            Some((row, col))
        } else {
            None
        }
     }).enumerate().map(|(i, pos)| (pos, 1));
     let mut prev_markers: HashMap<(usize, usize), usize> = markers_iter.collect();
     let mut markers: HashMap<(usize, usize), usize> = HashMap::new();
     // let mut id_group: HashMap<usize, usize> = prev_markers.values().map(|&v| (v, v)).collect();
     // let mut group_count: HashMap<usize, HashSet<usize>> = prev_markers.values().map(|&v| (v, HashSet::from([v]))).collect();
     for height in 1u16..=9 {
         for (pos, id) in prev_markers {
             for o in input.offset_iter(pos, false) {
                 if let Some((r, c)) =  input.offset(pos, o) {
                     if let Some(x) = input.get(r, c) {
                         if x == &height {
                             markers.entry((r, c)).or_default().add_assign(id);
                         }
                     }
                 }
             }
         }
         // prev_markers = HashMap::new();
         prev_markers = markers;
         // for (pos, id) in markers {
         //     prev_markers.insert(pos, id_group[&id]);
         // }
         markers = HashMap::new();
         // println!("{prev_markers:?}");
         // let s = input.to_str(|(v, r, c)| prev_markers.get(&(r, c)).map(|v| v.to_string()).unwrap_or(".".to_string()));
         // println!("{s}");
     }
     let mut acc = 0;
     prev_markers.values().sum()
     // println!("{group_count:?}");
    //  acc
}

// Implement your solution for part 1 here
pub fn part_1_olek(
    mut input: InputPart1,
    run_parameter: &RunParams<UserParams>,
    monitor: Arc<Mutex<RuntimeMonitor<EmptyUserMonitor>>>,
) -> OutputPart1 {
    let mut visited = Array2D::from_row_major(
        &vec![false; input.num_elements()],
        input.num_rows(),
        input.num_columns(),
    )
    .unwrap();

    let mut res = Array2D::from_row_major(
        &vec![HashSet::new(); input.num_elements()],
        input.num_rows(),
        input.num_columns(),
    )
    .unwrap();

    let mut total = 0;

    for row in 0..input.num_rows() {
        for col in 0..input.num_columns() {
            if !visited.get(row, col).unwrap() && input.get(row, col).unwrap() == &0 {
                total += dfs(row, col, &mut visited, &mut res, &input).len();
                // println!("done {total}");
            }
        }
    }
    total
}

fn dfs(
    row: usize,
    col: usize,
    visited: &mut Array2D<bool>,
    res: &mut Array2D<HashSet<(usize, usize)>>,
    input: &Array2D<u16>,
) -> HashSet<(usize, usize)> {
    if visited[(row, col)] {
        return res[(row, col)].clone();
    }
    visited[(row, col)] = true;
    let ex_h = input[(row, col)] + 1;
    // println!("{} at {} {}", ex_h, row, col);
    let mut node_res = HashSet::new();
    if ex_h == 10 {
        node_res.insert((row, col));

        res[(row, col)] = node_res.clone();
        return node_res;
    }
    for offset in [-1, 1] {
        // col
        if let Some(col) = col.checked_add_signed(offset) {
            if input.get(row, col).copied().unwrap_or(0) == ex_h {
                node_res.extend(dfs(row, col, visited, res, input))
            }
        }
        // row
        if let Some(row) = row.checked_add_signed(offset) {
            if input.get(row, col).copied().unwrap_or(0) == ex_h {
                node_res.extend(dfs(row, col, visited, res, input))
            }
        }
    }
    res[(row, col)] = node_res.clone();
    // println!("set {:?} at {} {}", node_res, row, col);
    node_res
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
