use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use rudvent_lib::solution::{SolutionBuilder, StructSolutionBuilder};
use rudvent_lib::solution::execution::{EmptyUserMonitor, EmptyUserParams, Example, RunParams, RuntimeMonitor};

// Update these types to reflect the types you want to use to solve the problems. These
// can be simple types (u64), integers, or your own types
type InputPart1 = Input;
type OutputPart1 = usize;
type InputPart2 = InputPart1;
type OutputPart2 = usize;

// These constants hold the answer for the examples, they are used to test your code
const EXAMPLE_1_ANS: OutputPart1 = 143;
const EXAMPLE_2_ANS: OutputPart2 = 0;

// This currently only the information about whether the run is an example or not. It may be augmented
type UserParams = EmptyUserParams;
type UserMonitor = EmptyUserMonitor;

struct Input {
    rules: Vec<(usize, usize)>,
    updates: Vec<Vec<usize>>,
}

// This function is called to prepare the input for part 1
pub fn prepare(input: String) -> InputPart1 {

    let (fst, snd) = input.split_once("\n\n").unwrap();

    let rules = fst.lines().map(|s| {
        let (f, s) = s.split_once("|").unwrap();
        (f.parse().unwrap(), s.parse().unwrap())
    }).collect();

    let updates = snd.lines().map(|s| {
        s.split(",").map(|c| c.parse().unwrap()).collect()
    }).collect();

    Input {rules, updates}
}

// Implement your solution for part 1 here
pub fn part_1(
    mut input: InputPart1,
    run_parameter: &RunParams<UserParams>,
    monitor: Arc<Mutex<RuntimeMonitor<EmptyUserMonitor>>>,
) -> OutputPart1 {
    let mut rules : HashMap<usize, HashSet<usize>> = HashMap::new();
    for (before, after) in input.rules {
        // let entry = rules.get_mut(&before);
        rules.entry(before).or_insert(HashSet::new()).insert(after);
    }
    input.updates.iter().filter_map(|updates| {
        let mut seen = HashSet::new();
        for update in updates {
            let _must_before = rules.get(update);
            if let Some(must_before) = _must_before {
                if !must_before.is_disjoint(&seen) {
                    return None
                }
            }
            seen.insert(*update);
        }
        Some(updates[(updates.len() - 1) / 2])
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
