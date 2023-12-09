use std::collections::HashMap;
use tracing::info;
use rudvent_lib::solution::{SolutionBuilder, StructSolutionBuilder};
use rudvent_lib::solution::execution::{EmptyUserMonitor, Example, RunParams, RuntimeMonitor};

// Update these types to reflect the types you want to use to solve the problems. These
// can be simple types (u64), integers, or your own types
type InputPart1 = Map;
type OutputPart1 = usize;
type InputPart2 = InputPart1;
type OutputPart2 = usize;

// These constants hold the answer for the examples, they are used to test your code
const EXAMPLE_1_ANS: OutputPart1 = 6;
const EXAMPLE_2_ANS: OutputPart2 = 0;

// This currently only the information about whether the run is an example or not. It may be augmented
type UserParams = ();
type UserMonitor = EmptyUserMonitor;
type Node = u8;
#[derive(Debug)]
pub struct Map {
    seq: String,
    links: HashMap<(Node, Node, Node), ((Node, Node, Node), (Node, Node, Node))>,
}

fn str_to_tuple(s: &str) -> (Node, Node, Node) {
    let mut ints = s.chars().map(|c| u32::from(c) as Node);
    (ints.next().unwrap(), ints.next().unwrap(), ints.next().unwrap())
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
    monitor: &mut RuntimeMonitor<EmptyUserMonitor>,
) -> OutputPart1 {
    let mut current = str_to_tuple("AAA");
    let end = str_to_tuple("ZZZ");
    let cycle = input.seq.chars().cycle();
    let mut steps = 0;
    for step in cycle {
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
    monitor: &mut RuntimeMonitor<EmptyUserMonitor>,
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
