use nom::bytes::complete::{tag, take_until1, take_while};
use nom::character::complete::space1;
use nom::combinator::map_res;
use nom::multi::many1;
use nom::sequence::{preceded, separated_pair, Tuple};
use nom::{AsChar, IResult, Parser};
use rudvent_lib::solution::execution::{EmptyUserMonitor, Example, RunParams, RuntimeMonitor};
use rudvent_lib::solution::{Solution, SolutionBuilder, StructSolutionBuilder};
use std::collections::HashSet;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

// Update these types to reflect the types you want to use to solve the problems. These
// can be simple types (u64), integers, or your own types
type InputPart1 = Vec<Card>;
type OutputPart1 = usize;
type InputPart2 = InputPart1;
type OutputPart2 = usize;

#[derive(Debug, Clone)]
pub struct Card {
    my_numbers: HashSet<usize>,
    winning_numbers: HashSet<usize>,
}
impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parser = ((
            take_until1(":"),
            tag(":"),
            separated_pair(numbers, tag(" |"), numbers),
        ));
        let (_, (_, _, (mine, winners))): (&str, (&str, &str, (Vec<usize>, Vec<usize>))) =
            parser.parse(s).unwrap();
        Ok(Card {
            my_numbers: mine.into_iter().collect(),
            winning_numbers: winners.into_iter().collect(),
        })
    }
}

impl Card {
    fn count_winners(&self) -> usize {
        self.my_numbers
            .iter()
            .filter(|&mine| self.winning_numbers.contains(mine))
            .count()
    }
    fn score(&self) -> usize {
        let count = self.count_winners();
        if count == 0 {
            0
        } else {
            2u64.pow((count.checked_sub(1).unwrap()).try_into().unwrap()) as usize
        }
    }
}

fn numbers(input: &str) -> IResult<&str, Vec<usize>> {
    let mut nums = many1(preceded(
        space1,
        map_res(take_while(AsChar::is_dec_digit), usize::from_str),
    ));
    nums.parse(input)
}

// These constants hold the answer for the examples, they are used to test your code
const EXAMPLE_1_ANS: OutputPart1 = 13;
const EXAMPLE_2_ANS: OutputPart2 = 30;

// This currently only the information about whether the run is an example or not. It may be augmented
type UserParams = ();
type UserMonitor = EmptyUserMonitor;

// This function is called to prepare the input for part 1
pub fn prepare(input: String) -> InputPart1 {
    input
        .lines()
        .map(|line| Card::from_str(line).unwrap())
        .collect()
}

// Implement your solution for part 1 here
pub fn part_1(
    mut input: InputPart1,
    run_parameter: &RunParams<UserParams>,
    monitor: Arc<Mutex<RuntimeMonitor<EmptyUserMonitor>>>,
) -> OutputPart1 {
    input.iter().map(|card| card.score()).sum()
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
    let mut counts: Vec<usize> = vec![1; input.len()];
    input.iter().enumerate().for_each(|(i, card)| {
        let card_score = card.count_winners();
        for c in 1..=card_score {
            counts[i + c] += counts[i];
        }
    });
    counts.into_iter().sum()
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
