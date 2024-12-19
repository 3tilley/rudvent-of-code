use std::cell::RefCell;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::iter;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use array2d::Array2D;
use rudvent_lib::solution::{SolutionBuilder, StructSolutionBuilder};
use rudvent_lib::solution::execution::{EmptyUserMonitor, EmptyUserParams, Example, RunParams, RuntimeMonitor};

use crate::utils::Array2DExt;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Wall,
    Empty,
    Start,
    Finish,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Horse {
    pos: (usize, usize),
    facing: (i8, i8),
}

impl Horse {
    fn cost(&self, new_offset: &(i8, i8)) -> usize {
        let total = max(self.facing.0.abs_diff(new_offset.0), self.facing.1.abs_diff(new_offset.1));
        match total {
            0 => 0,
            1 => 1000,
            2 => 2000,
            _ => unreachable!("Shouldn't be more than that"),
        }
    }
}

struct Course {
    start: (usize, usize),
    end: (usize, usize),
    map: Array2D<Tile>,
}

// Update these types to reflect the types you want to use to solve the problems. These
// can be simple types (u64), integers, or your own types
type InputPart1 = Course;
type OutputPart1 = usize;
type InputPart2 = InputPart1;
type OutputPart2 = usize;

// These constants hold the answer for the examples, they are used to test your code
const EXAMPLE_1_ANS: OutputPart1 = 7036;
const EXAMPLE_2_ANS: OutputPart2 = 0;

// This currently only the information about whether the run is an example or not. It may be augmented
type UserParams = EmptyUserParams;
type UserMonitor = EmptyUserMonitor;

// This function is called to prepare the input for part 1
pub fn prepare(input: String) -> InputPart1 {
    // let mut start = None;
    // let mut end = None;
    
    let map = Array2D::from_newline_delimited(&input, |(r, c, s)| {
        match s {
            '#' => Tile::Wall,
            '.' => Tile::Empty,
            'S' => {
                // start = Some((r, c));
                Tile::Start
            },
            'E' => {
                // end.get_or_insert((r, c));
                Tile::Finish
            }
            _ =>  panic!("Unrecongised char")
        }
    });
    let start = map.enumerate_row_major().find(|(_, &t)| t == Tile::Start).unwrap().0;
    let end = map.enumerate_row_major().find(|(_, &t)| t == Tile::Finish).unwrap().0;

    Course {
        start,
        end,
        map
    }
}

fn update(horse: &Horse, cost: usize, cost_map: &mut HashMap<Horse, usize>) -> Option<()> {
    match cost_map.get_mut(&horse) {
        None => {
            cost_map.insert(*horse, cost);
            Some(())
        },
        Some(old_cost) => {
            if *old_cost < cost {
                None
            } else {
                *old_cost = cost;
                Some(())
            }
        }
    }
}

fn next_steps<'a>(current: Horse, cost: usize, course: &'a InputPart1, cost_map: &'a mut HashMap<Horse, usize>) -> impl Iterator<Item = (Horse, usize)> + 'a {
    // course.map.neighbours_iter(current, include_diagonal)
    course.map.offset_iter(current.pos, false).filter_map(move |offset| {
        let new_r = current.pos.0.checked_add_signed(offset.0.into()).unwrap();
        let new_c = current.pos.1.checked_add_signed(offset.1.into()).unwrap();
        let cost = current.cost(&offset) + cost;
        // let mut new_horse = current;
        // new_horse.cost += cost;
        // match cost_map.get(&current) {
        //     None
        // }
        // cost_map.entry(current).and_modify(|old_cost| {
        //     if cost < *old_cost {
        //         return None
        //     } else {
        //         *old_cost = cost;
        //     }
        // });
        let rot_horse = Horse {pos: current.pos, facing: offset};
        update(&rot_horse, cost, cost_map)?;
        let new_horse = Horse {pos: (new_r, new_c), facing: offset};
        let new_cost = cost + 1;
        match course.map.get(new_r, new_c).unwrap() {
            Tile::Wall => None,
            Tile::Finish => {
                update(&new_horse, new_cost, cost_map);
                None
            },
            Tile::Empty | Tile::Start => {
                if update(&new_horse, new_cost, cost_map).is_some() {
                    Some((new_horse, new_cost))
                } else {
                    None
                }
            },
        }
    })
}

// Implement your solution for part 1 here
pub fn part_1(
    mut input: InputPart1,
    run_parameter: &RunParams<UserParams>,
    monitor: Arc<Mutex<RuntimeMonitor<EmptyUserMonitor>>>,
) -> OutputPart1 {
    let mut cost_map = HashMap::new();
    let mut current = vec![(Horse {pos: input.start, facing: (0, 1)}, 0)];
    while current.len() > 0 {
        // println!("{cost_map:?}");
        let mut next = Vec::new();
        for (horse, cost) in current {
            next.extend(next_steps(horse, cost, &input, &mut cost_map));
        }
        current = next;
        // current = current.into_iter().flat_map(|(horse, cost)| ).collect();
        // current = next;
    }
    cost_map.into_iter().filter_map(|(k, v)| (k.pos == input.end).then_some(v)).min().expect("Didn't find finish in the cost map")
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
