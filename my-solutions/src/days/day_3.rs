use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::character::complete::digit1;
use nom::multi::many1;
use nom::{IResult, Parser};
use rudvent_lib::solution::execution::{EmptyUserMonitor, Example, RunParams, RuntimeMonitor};
use rudvent_lib::solution::{Solution, SolutionBuilder, StructSolutionBuilder};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::sync::{Arc, Mutex};

// Update these types to reflect the types you want to use to solve the problems. These
// can be simple types (u64), integers, or your own types
type InputPart1 = Schematic;
type OutputPart1 = usize;
type InputPart2 = InputPart1;
type OutputPart2 = usize;

// These constants hold the answer for the examples, they are used to test your code
const EXAMPLE_1_ANS: OutputPart1 = 4361;
const EXAMPLE_2_ANS: OutputPart2 = 467835;

// This currently only the information about whether the run is an example or not. It may be augmented
type UserParams = ();
type UserMonitor = EmptyUserMonitor;

#[derive(Debug, Copy, Clone)]
struct Number {
    value: usize,
    start_location: (usize, usize),
    length: usize,
}

#[derive(Debug, Clone)]
pub struct Schematic {
    grid: Vec<Vec<char>>,
    numbers: Vec<Number>,
    symbols: HashSet<(usize, usize)>,
    gears: HashSet<(usize, usize)>,
    rows: usize,
    cols: usize,
}

impl Schematic {
    fn new(
        grid: Vec<Vec<char>>,
        numbers: Vec<Number>,
        symbols: HashSet<(usize, usize)>,
        gears: HashSet<(usize, usize)>,
    ) -> InputPart1 {
        let rows = grid.len();
        let cols = grid[0].len();
        Schematic {
            grid,
            numbers,
            symbols,
            gears,
            rows,
            cols,
        }
    }
}

enum Character<'a> {
    Num(&'a str, usize),
    Symbol,
    Gear,
    Dot,
}

impl<'a> Character<'_> {
    fn dot(input: &str) -> IResult<&str, Character> {
        let (input, _) = tag(".")(input)?;
        Ok((input, Character::Dot))
    }

    fn symbol(input: &str) -> IResult<&str, Character> {
        let (input, _) = take(1usize)(input)?;
        Ok((input, Character::Symbol))
    }

    fn num(input: &str) -> IResult<&str, Character> {
        let (input, s) = digit1(input)?;
        Ok((input, Character::Num(s, s.len())))
    }
    fn gear(input: &str) -> IResult<&str, Character> {
        let (input, s) = tag("*")(input)?;
        Ok((input, Character::Gear))
    }
}

fn line_parser(
    input: &str,
    row_index: usize,
) -> (Vec<Number>, Vec<(usize, usize)>, Vec<(usize, usize)>) {
    let (_, char_enums) = many1(alt((
        Character::dot,
        Character::num,
        Character::gear,
        Character::symbol,
    )))
    .parse(input)
    .unwrap();
    let mut index = 0;
    let mut symbols = vec![];
    let mut numbers = vec![];
    let mut gears = vec![];
    for en in char_enums {
        match en {
            Character::Num(s, len) => {
                let num = Number {
                    value: usize::from_str(s).unwrap(),
                    start_location: (row_index, index),
                    length: len,
                };
                numbers.push(num);
                index += len;
            }
            Character::Symbol => {
                symbols.push((row_index, index));
                index += 1;
            }
            Character::Dot => index += 1,
            Character::Gear => {
                symbols.push((row_index, index));
                gears.push((row_index, index));
                index += 1;
            }
        }
    }
    (numbers, symbols, gears)
}
// This function is called to prepare the input for part 1
pub fn prepare(input: String) -> InputPart1 {
    let mut symbols = HashSet::new();
    let mut gears = HashSet::new();
    let mut grid = Vec::new();
    let mut numbers = Vec::new();

    for (row, line) in input.lines().enumerate() {
        let (nums, syms, gs) = line_parser(line, row);
        numbers.extend(nums);
        symbols.extend(syms);
        gears.extend(gs);
        grid.push(line.chars().collect())
    }

    Schematic::new(grid, numbers, symbols, gears)
}

fn get_borders(
    start_row: usize,
    start_col: usize,
    len: usize,
    rows: usize,
    cols: usize,
) -> impl Iterator<Item = (usize, usize)> {
    (-1..=1)
        .flat_map(move |row| {
            (-1..=(len as isize)).filter_map(move |(col)| {
                let next = (
                    start_row.checked_add_signed(row),
                    start_col.checked_add_signed(col),
                );
                match next {
                    (Some(r), Some(c)) if (c < cols) && (r < rows) => Some((r, c)),
                    _ => None,
                }
            })
        })
        .filter(move |(r, c)| *r != start_row || *c < start_col || *c >= (start_col + len))
}

// Implement your solution for part 1 here
pub fn part_1(
    mut input: InputPart1,
    run_parameter: &RunParams<UserParams>,
    monitor: Arc<Mutex<RuntimeMonitor<EmptyUserMonitor>>>,
) -> OutputPart1 {
    input
        .numbers
        .iter()
        .filter(|n| {
            let mut borders = get_borders(
                n.start_location.0,
                n.start_location.1,
                n.length,
                input.rows,
                input.cols,
            );
            borders.any(|pos| input.symbols.contains(&pos))
        })
        .map(|n| n.value)
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
    // The HashMap type is <(gear_row, gear_col), (num_count, running_product)>
    let mut gear_map: HashMap<(usize, usize), (usize, usize)> =
        input.gears.iter().map(|&g| (g, (0, 1))).collect();
    input
        .numbers
        .iter()
        .flat_map(|n| {
            let mut borders = get_borders(
                n.start_location.0,
                n.start_location.1,
                n.length,
                input.rows,
                input.cols,
            );
            borders.filter_map(|pos| {
                if (&input.gears).contains(&pos) {
                    Some((*n, pos))
                } else {
                    None
                }
            })
        })
        .for_each(|(num, gear)| {
            let mut val = gear_map.get_mut(&gear);
            if let Some((count, product)) = val {
                *count += 1;
                *product *= num.value
            }
        });
    gear_map
        .iter()
        .filter_map(|(_, (count, product))| match count {
            2 => Some(product),
            _ => None,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_parser() {
        let (nums, syms, gears) = line_parser("467..114$.", 3);
        assert_eq!(nums.len(), 2);
        assert_eq!(nums[1].start_location, (3, 5));
        assert_eq!(nums[1].length, 3);
    }

    #[test]
    fn test_get_borders() {
        let borders = get_borders(1, 1, 1, 5, 6).collect::<Vec<_>>();
        assert_eq!(borders.len(), 8);
    }

    #[test]
    fn test_get_borders_corner() {
        let borders = get_borders(0, 0, 2, 5, 6).collect::<Vec<_>>();
        println!("{:?}", borders);
        assert_eq!(borders.len(), 5)
    }
}
