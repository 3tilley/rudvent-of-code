use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use rudvent_lib::solution::{SolutionBuilder, StructSolutionBuilder};
use rudvent_lib::solution::execution::{EmptyUserMonitor, EmptyUserParams, Example, RunParams, RuntimeMonitor};

// Update these types to reflect the types you want to use to solve the problems. These
// can be simple types (u64), integers, or your own types
type InputPart1 = Vec<(usize, usize)>;
type OutputPart1 = usize;
type InputPart2 = InputPart1;
type OutputPart2 = usize;

// These constants hold the answer for the examples, they are used to test your code
const EXAMPLE_1_ANS: OutputPart1 = 11;
const EXAMPLE_2_ANS: OutputPart2 = 31;

// This currently only the information about whether the run is an example or not. It may be augmented
type UserParams = EmptyUserParams;
type UserMonitor = EmptyUserMonitor;

// This function is called to prepare the input for part 1
pub fn prepare(input: String) -> InputPart1 {
    let mut vec = Vec::new();
    for line in input.lines() {
        let mut split = line.split_whitespace();
        let fst =  split.next().unwrap().parse().unwrap();
        let snd =  split.next().unwrap().parse().unwrap();
        vec.push((fst, snd))
    }
    vec
}

// Implement your solution for part 1 here
pub fn part_1(
    mut input: InputPart1,
    run_parameter: &RunParams<UserParams>,
    monitor: Arc<Mutex<RuntimeMonitor<EmptyUserMonitor>>>,
) -> OutputPart1 {
    let (mut fst, mut snd) : (Vec<OutputPart1>, Vec<OutputPart1>) = input.into_iter().unzip();
    fst.sort();
    snd.sort();
    fst.iter().zip(snd).map(|(f, s)| f.abs_diff(s)).sum()
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
    let (mut fst, mut snd) : (Vec<OutputPart2>, Vec<OutputPart2>) = input.into_iter().unzip();
    let mut snd_map: HashMap<OutputPart2, OutputPart2> = HashMap::new();
    snd.into_iter().for_each(|s| {
        let v = snd_map.get_mut(&s);
        match v {
            None => _ = snd_map.insert(s, 1),
            Some(old_val) => *old_val = &*old_val + 1,
        }
    });
    let ans = fst.into_iter().map(|f| f * snd_map.get(&f).unwrap_or(&0usize)).sum();
    ans
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
