use std::cmp::{max, min};
use std::sync::{Arc, Mutex};
use nom::bytes::complete::tag;
use nom::character::complete::u64;
use nom::IResult;
use nom::sequence::tuple;
use tracing::info;
use rudvent_lib::solution::{SolutionBuilder, StructSolutionBuilder};
use rudvent_lib::solution::execution::{EmptyUserMonitor, EmptyUserParams, Example, RunParams, RuntimeMonitor};

#[derive(Debug)]
struct Machine {
    a: (usize, usize),
    b: (usize, usize),
    prize: (usize, usize),
}

// Update these types to reflect the types you want to use to solve the problems. These
// can be simple types (u64), integers, or your own types
type InputPart1 = Vec<Machine>;
type OutputPart1 = usize;
type InputPart2 = InputPart1;
type OutputPart2 = usize;

// These constants hold the answer for the examples, they are used to test your code
const EXAMPLE_1_ANS: OutputPart1 = 480;
const EXAMPLE_2_ANS: OutputPart2 = 0;

// This currently only the information about whether the run is an example or not. It may be augmented
type UserParams = EmptyUserParams;
type UserMonitor = EmptyUserMonitor;

// Button A: X+94, Y+34
// Button B: X+22, Y+67
// Prize: X=8400, Y=5400

fn us(input: &str) -> IResult<&str, usize> {
    let (input, num) = u64(input)?;
    Ok((input, num.try_into().unwrap()))
}
fn parse_machine(s: &str) -> IResult<&str, Machine> {
    let (input, _) = tag("Button A: X+")(s)?;
    let (input, (a_x, _, a_y, _)) = tuple((us, tag(", Y+"), us, tag("\nButton B: X+")))(input)?;
    let (input, (b_x, _, b_y, _, x, _, y)) = tuple((us, tag(", Y+"), us, tag("\nPrize: X="), us, tag(", Y="), us))(input)?;
    Ok((input, Machine {
        a: (a_x, a_y),
        b: (b_x, b_y),
        prize: (x, y)
    }))
}

// This function is called to prepare the input for part 1
pub fn prepare(input: String) -> InputPart1 {
    let chunks = input.split("\n\n");
    chunks.map(|chunk| {
        parse_machine(chunk).unwrap().1
    }).collect()
}

fn check_mod_0(target: usize, a: usize, b: usize) -> bool {
    let lower = min(a, b);
    let higher = max(a, b);
    (higher & lower == 0) && (target % lower != 0)
}

// (n * A) + (m * B) = P
// (n * a_x) = p_x - (m * b_x)
// (n * a_y) = p_y - (m * b_y)

// (p_x - (m * bx)) / a_x = (p_y - (m * b_y)) / a_y

// (m * by)/ay - (m * bx)/ax = py/ay - px/ax
// m = (py/ay - px/ax) / (by/ay - bx/ax)
// m = (ax*py - ay*px) / (ax*by - ay*bx)

// m*bx = px - n*ax
// m*by = py - n*ay
// by*px - n*ax*by = bx*py - n*ay*bx
// n = (bx*py - by*px) / (ay*bx - ax*by)

// n = (by*px - bx*py) / (ax*by - ay*bx)
// m = (ax*py - ay*px) / (ax*by - ay*bx)

fn n_m(machine: &Machine) -> Option<(usize, usize)> {
    let div = ((machine.a.0 * machine.b.1) as isize) - ((machine.a.1 * machine.b.0) as isize);
    if div == 0 {
        // Test this path when there is alignment
        info!("Divisor=0");
        return None
    }

    let num_n = ((machine.b.1 * machine.prize.0) as isize) - ((machine.b.0 * machine.prize.1) as isize);
    let num_m = ((machine.a.0 * machine.prize.1) as isize) - ((machine.a.1 * machine.prize.0) as isize);
    match ((num_n / div, num_n % div), (num_m / div, num_m % div)) {
        ((n, _), (m, _)) if (n < 0 || m < 0) => {
            info!("Negative divisors: {n}, {m}");
            None
        },
        ((n, 0), (m, 0)) => Some((n as usize, m as usize)),
        ((_, rem_n), (_, rem_m)) => {
            info!("Non zero remainders: {rem_n}, {rem_m}");
            None
        },
    }
}

fn can_make(machine: &Machine, press_limit: Option<usize>) -> Option<usize> {
    info!("{machine:?}");
    let (n, m) = n_m(&machine)?;
    if let Some(limit) = press_limit {
        if (n > limit || m > limit) {
            info!("Presses too high: {n}, {m}");
            return None
        }
    }
    Some((n * 3) + m)
}


// Implement your solution for part 1 here
pub fn part_1(
    mut input: InputPart1,
    run_parameter: &RunParams<UserParams>,
    monitor: Arc<Mutex<RuntimeMonitor<EmptyUserMonitor>>>,
) -> OutputPart1 {
    input.iter().filter_map(|m| can_make(m, Some(100))).sum()
}

// If the puzzle requires a different input for part 2, this function can be updated
pub fn prepare_2(input: String) -> InputPart2 {
    let chunks = input.split("\n\n");
    let offset = 10_000_000_000_000usize;
    chunks.map(|chunk| {
        let mut m = parse_machine(chunk).unwrap().1;
        m.prize.0 += offset;
        m.prize.1 += offset;
        m
    }).collect()
}

pub fn part_2(
    mut input: InputPart1,
    run_parameter: &RunParams<UserParams>,
    monitor: Arc<Mutex<RuntimeMonitor<EmptyUserMonitor>>>,
) -> OutputPart1 {
    input.iter().filter_map(|m| can_make(m, None)).sum()
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
