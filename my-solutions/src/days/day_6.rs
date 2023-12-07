use std::str::FromStr;
use tracing::{info, info_span};
use rudvent_lib::solution::{SolutionBuilder, StructSolutionBuilder};
use rudvent_lib::solution::execution::{EmptyUserMonitor, Example, RunParams, RuntimeMonitor};

// Update these types to reflect the types you want to use to solve the problems. These
// can be simple types (u64), integers, or your own types
type InputPart1 = Vec<Race>;
type OutputPart1 = usize;
type InputPart2 = Race;
type OutputPart2 = usize;

// These constants hold the answer for the examples, they are used to test your code
const EXAMPLE_1_ANS: OutputPart1 = 288;
const EXAMPLE_2_ANS: OutputPart2 = 71503;

// This currently only the information about whether the run is an example or not. It may be augmented
type UserParams = ();
type UserMonitor = EmptyUserMonitor;

#[derive(Debug, Copy, Clone)]
pub struct Race {
    time: usize,
    record_distance: usize,
}

// This function is called to prepare the input for part 1
pub fn prepare(input: String) -> InputPart1 {
    let mut lines = input.lines();
    let times = lines.next().unwrap().split_whitespace().skip(1).map(|t| usize::from_str(t).unwrap());
    let dists = lines.next().unwrap().split_whitespace().skip(1).map(|t| usize::from_str(t).unwrap());
    times.zip(dists).map(|(t, d)| Race {time: t, record_distance: d}).collect()
}

// Implement your solution for part 1 here
// distance_covered = (t - t_charge) * v = (t - t_charge) * t_charge
// t_charge**2 - t_charge*t - winning_distance = 0
// t_c           t_c      t_t   w_d
// quadratic: (-b +- sqrt(b**2 - 4ac))/2a
// (t_t +- sqrt(t_t**2 + 4*w_d)) / 2

fn num_possibles(t_t: f64, w_d: f64) -> usize {
    // Adjust for exact matches
    let w_d = w_d + 1.0;
    let span = info_span!("Counting possibes", time=t_t, dist=w_d);
    let enter_ = span.enter();
    let low_time = (t_t - (t_t*t_t - 4.0*w_d).sqrt()) / 2.0;
    let high_time = (t_t + (t_t*t_t - 4.0*w_d).sqrt()) / 2.0;
    let ceil_low = low_time.ceil() as usize;
    let floor_high = high_time.floor() as usize;
    let res = floor_high - ceil_low + 1;
    info!("{}", format!("Result: {}", res));
    res
}
pub fn part_1(
    mut input: InputPart1,
    run_parameter: &RunParams<UserParams>,
    monitor: &mut RuntimeMonitor<EmptyUserMonitor>,
) -> OutputPart1 {
    input.iter().map(|r| num_possibles(r.time as f64, r.record_distance as f64)).product()
}

// If the puzzle requires a different input for part 2, this function can be updated
pub fn prepare_2(input: String) -> InputPart2 {
    let mut lines = input.lines();
    let time = usize::from_str(&lines.next().unwrap().replace("Time:", "").replace(" ", "")).unwrap();
    let dist = usize::from_str(&lines.next().unwrap().replace("Distance:", "").replace(" ", "")).unwrap();
    Race{ time, record_distance: dist }
}

pub fn part_2(
    mut input: InputPart2,
    run_parameter: &RunParams<UserParams>,
    monitor: &mut RuntimeMonitor<EmptyUserMonitor>,
) -> OutputPart1 {
    num_possibles(input.time as f64, input.record_distance as f64)
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
