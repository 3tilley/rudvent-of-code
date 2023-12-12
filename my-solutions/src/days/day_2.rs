use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::u64 as nom64;
use nom::multi::separated_list1;
use nom::sequence::Tuple;
use nom::{IResult, Parser};
use rudvent_lib::solution::execution::{EmptyUserMonitor, Example, RunParams, RuntimeMonitor};
use rudvent_lib::solution::{Solution, SolutionBuilder, StructSolutionBuilder};
use std::cmp::max;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use tracing::{info, info_span};

// Update these types to reflect the types you want to use to solve the problems. These
// can be simple types (u64), integers, or your own types
type InputPart1 = Vec<Game>;
type OutputPart1 = usize;
type InputPart2 = InputPart1;
type OutputPart2 = usize;

// These constants hold the answer for the examples, they are used to test your code
const EXAMPLE_1_ANS: OutputPart1 = 8;
const EXAMPLE_2_ANS: OutputPart2 = 2286;

// This currently only the information about whether the run is an example or not. It may be augmented
type UserParams = ();
type UserMonitor = EmptyUserMonitor;

#[derive(Debug, Clone)]
pub struct Game {
    draws: Vec<Draw>,
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, (_, game_id, _, draws)) = ((
            tag("Game "),
            nom64,
            tag(":"),
            separated_list1(tag(";"), Draw::find_all),
        ))
            .parse(s)
            .unwrap();
        Ok(Game { draws })
    }
}

#[derive(Debug, Clone, Default)]
struct Draw {
    blue: u64,
    green: u64,
    red: u64,
}
impl Draw {
    fn find_blue(input: &str) -> IResult<&str, Draw> {
        let (input, (_, count, _)) = ((tag(" "), nom64, tag(" blue"))).parse(input)?;
        Ok((
            input,
            Draw {
                blue: count,
                ..Default::default()
            },
        ))
    }
    fn find_red(input: &str) -> IResult<&str, Draw> {
        let (input, (_, count, _)) = ((tag(" "), nom64, tag(" red"))).parse(input)?;
        Ok((
            input,
            Draw {
                red: count,
                ..Default::default()
            },
        ))
    }
    fn find_green(input: &str) -> IResult<&str, Draw> {
        let (input, (_, count, _)) = ((tag(" "), nom64, tag(" green"))).parse(input)?;
        Ok((
            input,
            Draw {
                green: count,
                ..Default::default()
            },
        ))
    }

    fn combine(left: Draw, right: Draw) -> Draw {
        Draw {
            blue: left.blue + right.blue,
            green: left.green + right.green,
            red: left.red + right.red,
        }
    }

    fn find_all(input: &str) -> IResult<&str, Draw> {
        let (input, cols) = separated_list1(
            tag(","),
            alt((Draw::find_blue, Draw::find_green, Draw::find_red)),
        )
        .parse(input)?;
        let combined = cols.into_iter().reduce(Draw::combine).unwrap();
        Ok((input, combined))
    }
}

// This function is called to prepare the input for part 1
pub fn prepare(input: String) -> InputPart1 {
    input
        .lines()
        .map(|line| Game::from_str(&line).unwrap())
        .collect()
}

// Implement your solution for part 1 here
pub fn part_1(
    mut input: InputPart1,
    run_parameter: &RunParams<UserParams>,
    monitor: Arc<Mutex<RuntimeMonitor<EmptyUserMonitor>>>,
) -> OutputPart1 {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;
    input
        .iter()
        .enumerate()
        .filter_map(|(id, game)| {
            let invalid = game
                .draws
                .iter()
                .any(|draw| draw.green > max_green || draw.red > max_red || draw.blue > max_blue);
            match invalid {
                true => None,
                false => Some(id + 1),
            }
        })
        .sum()
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
    input
        .iter()
        .enumerate()
        .map(|(i, game)| {
            let span = info_span!("Calculating maximums", game_id = (i + 1).to_string());
            let _enter = span.enter();
            let (min_red, min_green, min_blue) = game
                .draws
                .iter()
                .enumerate()
                .map(|(j, draw)| {
                    info!(
                        "Draw: {} Red {},  Green {}, Blue {}",
                        j + 1,
                        draw.red,
                        draw.green,
                        draw.blue
                    );
                    (draw.red, draw.green, draw.blue)
                })
                .reduce(|acc, d| (max(acc.0, d.0), max(acc.1, d.1), max(acc.2, d.2)))
                .unwrap();
            info!(
                "Game: {} Red: {}, Green: {}, Blue: {}",
                i + 1,
                min_red,
                min_green,
                min_blue
            );
            (min_red * min_green * min_blue) as usize
        })
        .sum()
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
