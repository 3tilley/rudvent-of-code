use std::collections::HashMap;
use crate::utils::Array2DExt;
use array2d::Array2D;
use rudvent_lib::solution::execution::{
    EmptyUserMonitor, EmptyUserParams, Example, RunParams, RuntimeMonitor,
};
use rudvent_lib::solution::{SolutionBuilder, StructSolutionBuilder};
use std::ops::{IndexMut, ShlAssign};
use std::sync::{Arc, Mutex};
use ibig::UBig;
use tracing::info;

// Update these types to reflect the types you want to use to solve the problems. These
// can be simple types (u64), integers, or your own types
type InputPart1 = Map;
type OutputPart1 = usize;
type InputPart2 = InputPart1;
type OutputPart2 = usize;

// These constants hold the answer for the examples, they are used to test your code
const EXAMPLE_1_ANS: OutputPart1 = 136;
const EXAMPLE_2_ANS: OutputPart2 = 64;

// This currently only the information about whether the run is an example or not. It may be augmented
type UserParams = EmptyUserParams;
type UserMonitor = EmptyUserMonitor;

pub struct Map {
    map: Array2D<Option<Rock>>,
}

impl Map {
    fn next_space(
        &self,
        start: (usize, usize),
        offset: (i8, i8),
        current: (usize, usize),
    ) -> Option<(usize, usize)> {
        let new_spot = match self.map.offset(current, offset) {
            None => return (start != current).then_some(current),
            Some(next) => next,
        };
        let is_space = self.map.get(new_spot.0, new_spot.1).unwrap().is_none();
        if is_space {
            self.next_space(start, offset, new_spot)
        } else {
            (start != current).then_some(current)
        }
    }
    fn tip(&mut self, offset: (i8, i8)) {
        // TODO: Check if it's row offset
        assert!((offset.0 == 0) ^ (offset.1 == 0));
        let is_horizontal = offset.0 == 0;
        let reverse = (offset.0 > 0) || (offset.1 > 0);

        let rollers: Vec<(usize, usize)> = self.map.enumerate_row_major().filter_map(|(pos, &rock)| (rock == Some(Rock::Round)).then_some((pos)) ).collect();
        if reverse {

            for pos in rollers.iter().rev() {
                if let Some(new_space) = self.next_space(*pos, offset, *pos) {
                    *self.map.index_mut(new_space) = Some(Rock::Round);
                    *self.map.index_mut(*pos) = None;
                }
            }
        } else {

            for pos in rollers {
                if let Some(new_space) = self.next_space( pos, offset, pos) {
                    *self.map.index_mut(new_space) = Some(Rock::Round);
                    *self.map.index_mut(pos) = None;
                }
            }
        }
    }

    fn print(&self) {
        for row in 0..self.map.num_rows() {
            let row_str: String = self.map
                .row_iter(row)
                .unwrap()
                .map(|r| match r {
                    None => '.',
                    Some(rock) => match rock {
                        Rock::Round => 'O',
                        Rock::Square => '#',
                    },
                })
                .collect();
            println!("{}", row_str);
        }
        println!("\n");
    }

    fn calc_moment(&self, from_south: bool) -> usize {
        self.map.enumerate_row_major()
            .filter_map(|((row, _), rock)| {
                rock.filter(|r| r == &Rock::Round)
                    .map(|_| if from_south {(self.map.num_rows() - row)} else {row + 1})
            })
            .sum()
    }

    fn make_int(&self) -> UBig {
        let mut big_int = ibig::UBig::default();
        for (pos, rock) in self.map.enumerate_row_major() {
            if let Some(Rock::Round) = rock {
                big_int += 1;
            }
            big_int.shl_assign(1);
        }
        big_int
    }

}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Rock {
    Round,
    Square,
}

impl Rock {
    fn from_char(c: char) -> Option<Rock> {
        match c {
            'O' => Some(Rock::Round),
            '#' => Some(Rock::Square),
            '.' => None,
            _ => unreachable!("Unrecognised char"),
        }
    }
}

// This function is called to prepare the input for part 1
pub fn prepare(input: String) -> InputPart1 {
    let map = Array2D::from_newline_delimited(&input, Rock::from_char);
    Map {map}
}



// Implement your solution for part 1 here
pub fn part_1(
    mut input: InputPart1,
    run_parameter: &RunParams<UserParams>,
    monitor: Arc<Mutex<RuntimeMonitor<EmptyUserMonitor>>>,
) -> OutputPart1 {
    input.tip((-1, 0));
    input.print();
    input.calc_moment(true)
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
    let binding = vec![(-1, 0), (0, -1), (1, 0), (0, 1)];
    let cycles = 1_000_000_000;
    let total_cycles = 4 * cycles;
    monitor.lock().unwrap().total_progress =  total_cycles;
    let mut history = HashMap::<UBig, usize>::new();
    let mut cycle = binding.iter().cycle().take( total_cycles);
    for (i, offset) in cycle.clone().enumerate() {
        if i % 100 == 0 {
            monitor.lock().unwrap().current_progress = i;
        }
        input.tip(*offset);
        if offset == &(0, 1) {
            let key = input.make_int();
            if let Some(previous) = history.get(&key) {

                let cycle_length = i - previous;
                let mod_ = (total_cycles - i - 1) % cycle_length;
                input.print();

                for j in 0..mod_ {
                    input.tip(*cycle.next().unwrap());
                }
                break;
            } else {
                history.insert(key, i);
            }
        }
    }
    // input.print();
    input.calc_moment(true)
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
    fn test_next_space_easy() {
        let arr = prepare(".\n.\n.\nO".to_string());
        let spot = arr.next_space( (3, 0), (0, -1), (3, 0));
        assert_eq!(spot, Some((0, 0)))
    }

    #[test]
    fn test_next_space_blocker() {
        let arr = prepare(".\n#\n.\nO".to_string());
        let spot = arr.next_space( (3, 0), (-1, 0), (3, 0));
        assert_eq!(spot, Some((2, 0)))
    }

    #[test]
    fn test_calc_moment() {
        let arr = prepare("O.\n.#\n.O\n..".to_string());
        let moment = arr.calc_moment(true);
        let north_moment = arr.calc_moment(false);
        assert_eq!(moment, 6);
        assert_eq!(north_moment, 4);
    }
}
