use std::collections::HashMap;
use std::ops::{AddAssign, Mul};
use std::sync::{Arc, Mutex};
use array2d::Array2D;
use num_traits::ToPrimitive;
use rudvent_lib::solution::{SolutionBuilder, StructSolutionBuilder};
use rudvent_lib::solution::execution::{EmptyUserMonitor, EmptyUserParams, Example, RunParams, RuntimeMonitor};

struct Robot {
    pos: (usize, usize),
    v: (isize, isize),
}

// Update these types to reflect the types you want to use to solve the problems. These
// can be simple types (u64), integers, or your own types
type InputPart1 = Vec<Robot>;
type OutputPart1 = usize;
type InputPart2 = InputPart1;
type OutputPart2 = usize;



// These constants hold the answer for the examples, they are used to test your code
const EXAMPLE_1_ANS: OutputPart1 = 12;
const EXAMPLE_2_ANS: OutputPart2 = 0;

const FULL_ROWS: usize = 103;
const FULL_COLS: usize = 101;
const EX_ROWS: usize = 7;
const EX_COLS: usize = 11;


// This currently only the information about whether the run is an example or not. It may be augmented
type UserParams = EmptyUserParams;
type UserMonitor = EmptyUserMonitor;

// This function is called to prepare the input for part 1
pub fn prepare(input: String) -> InputPart1 {
    let mut vec = Vec::new();
    for line in input.lines() {
        let (fst, snd) = line.split_once(" ").unwrap();
        let (c, r) = fst.split_once("=").unwrap().1.split_once(",").unwrap();
        let (c_v, r_v) = snd.split_once("=").unwrap().1.split_once(",").unwrap();
        let r = Robot{
            pos: (r.parse().unwrap(), c.parse().unwrap()),
            v: (r_v.parse().unwrap(), c_v.parse().unwrap()),
        };
        vec.push(r);
    }
    vec
}

fn add(p: usize, v: isize, limit: usize, its: usize) -> usize {
    let movement = v.checked_mul(its.to_isize().unwrap()).expect("Movement too large");
    match p.checked_add_signed(movement) {
        Some(new_pos) => new_pos % limit,
        None => {
            // There is definitely a simpler way of doing this
            let m = movement % limit.to_isize().unwrap();
            let new_pos = p.checked_add_signed(m).unwrap_or_else(|| {
                let adj_pos = p.checked_add_signed(limit.to_isize().unwrap() + m).unwrap();
                // (limit - m.abs().try_into().unwrap()).try_into().unwrap();
                adj_pos
            });
            new_pos % limit
        }
    }
}

// Implement your solution for part 1 here
pub fn part_1(
    mut input: InputPart1,
    run_parameter: &RunParams<UserParams>,
    monitor: Arc<Mutex<RuntimeMonitor<EmptyUserMonitor>>>,
) -> OutputPart1 {

    let rows = if run_parameter.is_example { EX_ROWS } else { FULL_ROWS };
    let cols = if run_parameter.is_example { EX_COLS } else { FULL_COLS };
    let quads = add_quadrants(&input, rows, cols, 100, false);
    // for i in 0..500 {
    //     add_quadrants(&input, rows, cols, i);
    // }
    quads.into_values().reduce(|a, c| a * c).unwrap()
}

fn add_quadrants(input: &InputPart1, rows: usize, cols: usize, its: usize, print: bool) -> HashMap<(bool, bool), usize> {
    let mid_row = rows / 2;
    let mid_col = cols / 2;
    let mut quads: HashMap<(bool, bool), usize> = HashMap::new();
    let mut new_pos: HashMap<(usize, usize), usize> = HashMap::new();
    for robot in input {
        let new_row = add(robot.pos.0, robot.v.0, rows, its);
        let new_col = add(robot.pos.1, robot.v.1, cols, its);
        new_pos.entry((new_row, new_col)).or_insert(0).add_assign(1);
        if new_row != mid_row && new_col != mid_col {
            let key = (new_row < mid_row, new_col < mid_col);
            quads.entry(key).or_insert(0).add_assign(1);
        }
    }
    if print {
        for r in 0..rows {
            for c in 0..cols {
                let ch = new_pos.get(&(r, c)).map(|u| u.to_string()).unwrap_or(".".to_string());
                print!("{}", ch);
            }
            print!("\n");
        }
        print!("\n");
    }
    quads
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
    fn test_add_pos() {
        let res = add(2, 6, 10, 2);
        assert_eq!(res, 4);
    }

    #[test]
    fn test_add_neg() {
        let res = add(2, -3, 10, 4);
        assert_eq!(res, 0);
    }

    #[test]
    fn test_add_neg_3() {
        let res = add(3, -3, 7, 100);
        assert_eq!(res, 4);
    }

    #[test]
    fn test_mod() {
        assert_eq!(-10 % 3, -1);
        assert_eq!(-11 % 3, -2);
        assert_eq!(-12 % 3, 0);
        assert_eq!(-300 % 7, -6);
    }

    #[test]
    fn test_add_both() {
        let res_r = add(4, -3, 7, 5);
        let res_c = add(2, 2, 11, 5);
        assert_eq!(res_r, 3);
        assert_eq!(res_c, 1);
    }
}