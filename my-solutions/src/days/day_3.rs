use std::sync::{Arc, Mutex};

use nom::bytes::complete::{tag, take_until};
use nom::branch::alt;
use nom::combinator::value;
use nom::multi::{many0, separated_list0, many_till};
use nom::sequence::tuple;
use nom::IResult;
use nom::character::complete::{anychar, u64, not_line_ending};

use rudvent_lib::solution::{SolutionBuilder, StructSolutionBuilder};
use rudvent_lib::solution::execution::{EmptyUserMonitor, EmptyUserParams, Example, RunParams, RuntimeMonitor};

// Update these types to reflect the types you want to use to solve the problems. These
// can be simple types (u64), integers, or your own types
type InputPart1 = String;
type OutputPart1 = u64;
type InputPart2 = InputPart1;
type OutputPart2 = u64;

// These constants hold the answer for the examples, they are used to test your code
const EXAMPLE_1_ANS: OutputPart1 = 161;
const EXAMPLE_2_ANS: OutputPart2 = 48;

// This currently only the information about whether the run is an example or not. It may be augmented
type UserParams = EmptyUserParams;
type UserMonitor = EmptyUserMonitor;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Op {
    Mul(OutputPart1, OutputPart1),
    Do,
    Dont,
}

// This function is called to prepare the input for part 1
pub fn prepare(input: String) -> InputPart1 {
    input
}

fn parse_mul(input: &str) -> IResult<&str, Op> {
    let (leftover, output) = tuple((tag("mul("), u64, tag(","), u64, tag(")")))(input)?;
    return Ok((leftover, Op::Mul(output.1, output.3)))
}

fn parse_do_dont(input: &str) -> IResult<&str, Op> {
    alt((value(Op::Do, tag("do()")), value(Op::Dont, tag("don't()"))))(input)
}

// TODO: There must be a better way of doing this, but many_separator is greedy
fn parse_messy_mul(input: &str) -> IResult<&str, Op> {
    let (rem, res) = many_till(anychar, parse_mul)(input)?;
    return Ok((rem, res.1))
}

fn parse_messy_all(input: &str) -> IResult<&str, Op> {
    let (rem, res) = many_till(anychar, alt((parse_mul, parse_do_dont)))(input)?;
    return Ok((rem, res.1))
}

fn parse_many_mul(input: &str) -> IResult<&str, Vec<Op>> {
    many0(parse_messy_mul)(input)
}

fn parse_many_all(input: &str) -> IResult<&str, Vec<Op>> {
    many0(parse_messy_all)(input)
}

// Implement your solution for part 1 here
pub fn part_1(
    mut input: InputPart1,
    run_parameter: &RunParams<UserParams>,
    monitor: Arc<Mutex<RuntimeMonitor<EmptyUserMonitor>>>,
) -> OutputPart1 {
    println!("{input}");
    let ops = parse_many_mul(&input).unwrap().1;
    println!("{ops:?}");
    ops.into_iter().map(|o| {
        match o {
            Op::Mul(x, y) => x * y,
            _ => panic!("Shouldn't find any of these in part_1"),
        }
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
    let ops = parse_many_all(&input).unwrap().1;
    ops.into_iter().fold((0, true), |(acc, doing), val| {
        match val {
            Op::Do => (acc, true),
            Op::Dont => (acc, false),
            Op::Mul(x, y) => {
                let new_acc = if (doing) {acc + (x * y)} else {acc};
                (new_acc, doing)
            }
        }
    }).0
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
    fn mul_recognise() {
        let (remaining, result) = parse_mul("mul(2,3)").unwrap();
        assert_eq!(result, Op::Mul(2, 3));
    }

    #[test]
    fn many_mul_recognise() {
        let (remaining, result) = parse_many_mul("xk*mul(2,3)__mul(3,4)").unwrap();
        assert_eq!(result, vec![Op::Mul(2, 3), Op::Mul(3,4)]);
    }
}