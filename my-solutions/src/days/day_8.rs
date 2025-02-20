use core::num;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::iter::once;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use rudvent_lib::solution::{SolutionBuilder, StructSolutionBuilder};
use rudvent_lib::solution::execution::{EmptyUserMonitor, EmptyUserParams, Example, RunParams, RuntimeMonitor};

use crate::utils::SparseArray;

#[derive(Debug)]
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
const EXAMPLE_2_ANS: OutputPart2 = 34;

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
            }
            num_cols = num_cols.max(col);
            num_rows = num_cols.max(row);
        }
    }
    let map_iter = letters.into_iter().map(|(key, val)| {
        (key, SparseArray::new(val, (num_rows, num_cols), 0).unwrap())
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

fn anti_location(this: &(usize, usize), other: &(usize, usize)) -> Option<(usize, usize)> {
    let (x_diff, y_diff) = ((other.0 as isize) - (this.0 as isize), (other.1 as isize) - (this.1 as isize));
    // println!("x_diff={x_diff}, y_diff={y_diff}");
    // if (x_diff % 2 != 0) || (y_diff % 2 != 0) {
    //     return None
    // }
    // let (x_2, y_2) = (x_diff / 2, y_diff / 2);
    let new_x = other.0.checked_add_signed(x_diff)?;
    let new_y = other.1.checked_add_signed(y_diff)?;
    Some((new_x, new_y))
}

fn anti_locations(this: (usize, usize), other: (usize, usize), max_bounds: (usize, usize)) -> impl Iterator<Item = Option<(usize, usize)>> {
    let (x_diff, y_diff) = ((other.0 as isize) - (this.0 as isize), (other.1 as isize) - (this.1 as isize));
    let mut counter = Rc::new(RefCell::new(-1));
    std::iter::repeat_with(move || {
        *counter.borrow_mut() += 1;
        // let c = counter.borrow()
        let new_x = other.0.checked_add_signed(*counter.borrow() * x_diff)?;
        let new_y = other.1.checked_add_signed(*counter.borrow() * y_diff)?;
        if (new_x > max_bounds.0) && (new_y > max_bounds.1) {
            return None
        }
        // counter += 1;
        Some((new_x, new_y))
    })
}

// Implement your solution for part 1 here
pub fn part_1(
    mut input: InputPart1,
    run_parameter: &RunParams<UserParams>,
    monitor: Arc<Mutex<RuntimeMonitor<EmptyUserMonitor>>>,
) -> OutputPart1 {
    let mut antinodes = HashSet::new();
    for (c, sparse) in input.map {
        let mut antis = Vec::new();
        for (this, _) in sparse.enumerate_iter() {
            for (other, _) in sparse.enumerate_iter() {
                if this != other {
                    antis.push((this, other));
                }
            }
        }
        antis.iter().filter_map(|(this, other)| {
            let new_loc = anti_location(this, other)?;
            // counter += 1;
            if sparse.get_with_default(&new_loc).is_some() {
                // counter_2 += 1;
                Some(new_loc)
            } else {
                None
            }
        }).for_each(|new_loc| drop(antinodes.insert(new_loc)));
    }
    antinodes.len()
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
    let mut antinodes = HashSet::new();
    for (c, sparse) in input.map {
        let mut antis = Vec::new();
        for (this, _) in sparse.enumerate_iter() {
            for (other, _) in sparse.enumerate_iter() {
                if this != other {
                    antis.push((this, other));
                }
            }
        }
        for (this, other) in antis {
            let mut new_locs = anti_locations(*this, *other, sparse.max_bounds);
            while let Some(Some(loc)) = new_locs.next() {
                if sparse.get_with_default(&loc).is_some() {
                    antinodes.insert(loc);
                }
            }
        }
        // antis.iter().filter_map(|(this, other)| {
        //     let new_loc = anti_location(this, other)?;
        //     // counter += 1;
        //     if sparse.get_with_default(&new_loc).is_some() {
        //         // counter_2 += 1;
        //         Some(new_loc)
        //     } else {
        //         None
        //     }
        // }).for_each(|new_loc| drop(antinodes.insert(new_loc)));
    }
    antinodes.len()
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
    fn test_anti() {
        assert_eq!(anti_location(&(0, 0), &(4, 2)), Some((6, 3)));
        assert_eq!(anti_location(&(0, 0), &(3, 2)), None);
        assert_eq!(anti_location(&(2, 4), &(1, 2)), None);
    }
}