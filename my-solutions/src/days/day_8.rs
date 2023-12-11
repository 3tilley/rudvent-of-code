use std::collections::HashMap;
use std::ops::Shl;
use tracing::info;
use rudvent_lib::solution::{SolutionBuilder, StructSolutionBuilder};
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;
use rudvent_lib::solution::execution::{EmptyUserMonitor, Example, RunParams, RuntimeMonitor};

// Update these types to reflect the types you want to use to solve the problems. These
// can be simple types (u64), integers, or your own types
type InputPart1 = Map;
type OutputPart1 = usize;
type InputPart2 = InputPart1;
type OutputPart2 = usize;

// These constants hold the answer for the examples, they are used to test your code
const EXAMPLE_1_ANS: OutputPart1 = 6;
const EXAMPLE_2_ANS: OutputPart2 = 6;

// This currently only the information about whether the run is an example or not. It may be augmented
type UserParams = ();
type UserMonitor = EmptyUserMonitor;
type Node = u8;
#[derive(Debug)]
pub struct Map {
    seq: String,
    links: HashMap<u32, (u32, u32)>,
}

fn str_to_tuple(s: &str) -> u32 {
    let mut ints = s.chars().map(|c| u32::from(c));
    let tup = (ints.next().unwrap(), ints.next().unwrap(), ints.next().unwrap());
    tup.0.shl(16) + tup.1.shl(8) + tup.2
}

fn ends_with_a(node: &u32) -> bool {
    node % 256 == 65
}
fn ends_with_z(node: &u32) -> bool {
    node % 256 == 90
}
// This function is called to prepare the input for part 1
pub fn prepare(input: String) -> InputPart1 {
    let mut lines = input.lines();
    let seq = lines.next().unwrap().to_string();
    lines.next();
    let map = lines.map(|line| {
        let (left, right) = line.split_once("=").unwrap();
        let binding = right.replace("(", "").replace(")","");
        let tuple = binding.split_once(",").unwrap();
        (str_to_tuple(left.trim()), (str_to_tuple(tuple.0.trim()), str_to_tuple(tuple.1.trim())))
    }).collect();
    Map { seq, links: map }
}

// Implement your solution for part 1 here
pub fn part_1(
    mut input: InputPart1,
    run_parameter: &RunParams<UserParams>,
    monitor: Arc<Mutex<RuntimeMonitor<EmptyUserMonitor>>>,
) -> OutputPart1 {
    let mut current = str_to_tuple("AAA");
    let end = str_to_tuple("ZZZ");
    let cycle = input.seq.chars().cycle();
    let mut steps = 0;
    for step in cycle {
        monitor.lock().unwrap().current_progress += 1;
        sleep(Duration::from_secs(1));
        steps += 1;
        let next = input.links.get(&current).unwrap();
        // info!("{:?}", next);
        current = if step == 'L' {
            next.0
        } else {
            next.1
        };
        if current == end {
            break
        }
    }
    return steps
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
    let mut currents = input.links.iter().filter_map(|link| ends_with_a(link.0).then_some((*link.0, *link.1))).collect::<Vec<_>>();
    let cycle = input.seq.chars().cycle();
    let mut steps = 0;
    for step in cycle {
        monitor.lock().unwrap().current_progress += 1;
        sleep(Duration::from_secs(1));
        steps += 1;
        currents = currents.iter().map(|link| {
            // info!("{:?}", next);
            let next = if step == 'L' {
                link.1.0
            } else {
                link.1.1
            };
            (next, *input.links.get(&next).unwrap())
        }).collect();
        if currents.iter().all(|link| ends_with_z(&link.0)) { break }
    }
    return steps
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
    fn test_is_a() {
        let node = str_to_tuple("bbA");
        let not_node = str_to_tuple("SAD");
        assert!(ends_with_a(&node));
        assert!(!ends_with_a(&not_node));
    }

    #[test]
    fn test_is_z() {
        let node = str_to_tuple("BBZ");
        let not_node = str_to_tuple("SZD");
        assert!(ends_with_z(&node));
        assert!(!ends_with_z(&not_node));
    }
}
