use crate::days::day_12::Entry::{Damaged, Operational, Unknown};
use rudvent_lib::solution::execution::{
    EmptyUserMonitor, EmptyUserParams, Example, RunParams, RuntimeMonitor,
};
use rudvent_lib::solution::{SolutionBuilder, StructSolutionBuilder};
use std::collections::HashMap;
use std::iter::once;
use std::ops::ShlAssign;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use tracing::{debug, info, instrument};

// Update these types to reflect the types you want to use to solve the problems. These
// can be simple types (u64), integers, or your own types
type InputPart1 = Vec<SpringLine>;
type OutputPart1 = usize;
type InputPart2 = InputPart1;
type OutputPart2 = usize;

// These constants hold the answer for the examples, they are used to test your code
const EXAMPLE_1_ANS: OutputPart1 = 21;
const EXAMPLE_2_ANS: OutputPart2 = 525152;

// This currently only the information about whether the run is an example or not. It may be augmented
type UserParams = EmptyUserParams;
type UserMonitor = EmptyUserMonitor;

#[derive(Debug, Clone, Copy)]
enum Entry {
    Operational,
    Damaged,
    Unknown,
}

impl Entry {
    fn from_char(c: char) -> Entry {
        match c {
            '.' => Operational,
            '#' => Damaged,
            '?' => Unknown,
            _ => unreachable!("Unrecognised character"),
        }
    }
    fn to_string(&self) -> char {
        match self {
            Operational => '.',
            Damaged => '#',
            Unknown => '?',
        }
    }
}

#[derive(Debug, Clone)]
pub struct SpringLine {
    springs: Vec<Entry>,
    orders: Vec<usize>,
}

impl SpringLine {
    fn from_str(s: &str) -> SpringLine {
        let (spring_str, order_str) = s.split_once(" ").unwrap();
        let springs = spring_str.chars().map(Entry::from_char).collect();
        let orders = order_str
            .split(",")
            .map(|o| usize::from_str(o).unwrap())
            .collect();
        SpringLine { springs, orders }
    }

    fn from_str_5(s: &str) -> SpringLine {
        let (spring_str, order_str) = s.split_once(" ").unwrap();
        let springs: Vec<Entry> = vec![spring_str; 5]
            .join("?")
            .chars()
            .map(Entry::from_char)
            .collect();
        let orders = vec![order_str; 5]
            .join(",")
            .split(",")
            .map(|o| usize::from_str(o).unwrap())
            .collect();
        SpringLine { springs, orders }
    }
}

fn vec_to_str(vec: &Vec<Entry>) -> String {
    vec.iter().map(Entry::to_string).collect()
}

// This function is called to prepare the input for part 1
pub fn prepare(input: String) -> InputPart1 {
    input.lines().map(SpringLine::from_str).collect()
}

// Implement your solution for part 1 here
pub fn part_1(
    mut input: InputPart1,
    run_parameter: &RunParams<UserParams>,
    monitor: Arc<Mutex<RuntimeMonitor<EmptyUserMonitor>>>,
) -> OutputPart1 {
    let mut cache = HashMap::new();
    // let ans1: OutputPart1 =  input.iter().map(|mut spring| {
    //     let res = one_line(&mut spring.springs.clone(), &mut spring.orders.clone(), &mut cache, &monitor).unwrap_or(0);
    //     info!("Possibles: {}", res);
    //     res
    // }).sum();
    input
        .into_iter()
        .map(|mut spring| {
            let res = one_line(
                &mut spring.springs,
                &mut spring.orders,
                &mut cache,
                &monitor,
            )
            .unwrap_or(0);
            info!("Possibles: {}", res);
            res
        })
        .sum()
}

fn vec_to_u128(vec: &Vec<Entry>, orders: &Vec<usize>) -> Result<u128, ()> {
    if vec.len() > 16 || orders.len() > 16 {
        Err(())
    } else {
        let mut full_output: u128 = 0;
        let mut output: u64 = 0;
        for e in vec {
            let num = match e {
                Operational => 1u64,
                Damaged => 2u64,
                Unknown => 3u64,
            };
            output += num;
            output.shl_assign(4);
        }
        full_output += (output as u128);
        full_output.shl_assign(16);

        let mut order_int: u64 = 0;
        for o in orders {
            if o > &4usize {
                return Err(());
            } else {
                order_int += (*o as u64);
                order_int.shl_assign(4);
            }
        }
        full_output += (order_int as u128);
        Ok(full_output)
    }
}

fn one_line_cache(
    mut line: &mut Vec<Entry>,
    mut orders: &Vec<usize>,
    mut cache: &mut HashMap<u128, Option<usize>>,
    monitor: &Arc<Mutex<RuntimeMonitor<EmptyUserMonitor>>>,
) -> Option<usize> {
    let key_res = vec_to_u128(line, orders);
    // info!("{:?}", cache);
    if let Ok(key) = key_res {
        let cache_res = cache.get(&key);
        if let Some(res) = cache_res {
            info!("Found result for {}", vec_to_str(line));
            return *res;
        }
    }
    let res = one_line(line, orders, cache, monitor);
    if let Ok(key) = key_res {
        cache.insert(key, res);
    }
    res
}

fn one_line(
    mut line: &mut Vec<Entry>,
    mut orders: &Vec<usize>,
    mut cache: &mut HashMap<u128, Option<usize>>,
    monitor: &Arc<Mutex<RuntimeMonitor<EmptyUserMonitor>>>,
) -> Option<usize> {
    debug!("Running for {}", vec_to_str(line));
    monitor.lock().unwrap().current_progress += 1;
    let mut current_block_index = 0;
    let mut current_cont = None;
    let mut can_take = true;
    for i in 0..line.len() {
        let entry = line[i];
        match entry {
            Operational => {
                if current_cont.unwrap_or(0) != 0 {
                    return None;
                } else {
                    current_cont = None
                }
            }
            Damaged => match can_take {
                true => match current_cont {
                    None => {
                        current_cont = Some(orders.get(current_block_index)? - 1);
                        current_block_index += 1;
                    }
                    Some(cont) => {
                        current_cont = Some(cont.checked_sub(1)?);
                    }
                },
                false => {
                    return None;
                }
            },
            Unknown => {
                line[i] = Entry::Damaged;
                let dam = one_line_cache(line, orders, cache, monitor);
                line[i] = Entry::Operational;
                let op = one_line_cache(line, orders, cache, monitor);
                line[i] = Entry::Unknown;
                match dam.unwrap_or(0) + op.unwrap_or(0) {
                    0 => return None,
                    x => return Some(x),
                }
            }
        }
    }
    if current_block_index == orders.len() && current_cont.unwrap_or(0) == 0 {
        Some(1)
    } else {
        None
    }
}

// If the puzzle requires a different input for part 2, this function can be updated
pub fn prepare_2(input: String) -> InputPart2 {
    input.lines().map(SpringLine::from_str_5).collect()
}

pub fn part_2(
    mut input: InputPart1,
    run_parameter: &RunParams<UserParams>,
    monitor: Arc<Mutex<RuntimeMonitor<EmptyUserMonitor>>>,
) -> OutputPart1 {
    part_1(input, run_parameter, monitor)
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

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_complete_line() {
//         let mut line = SpringLine::from_str("#.#.### 1,1,3");
//         let mon = RuntimeMonitor::new_arc();
//         assert_eq!(one_line(&mut line.springs, &line.orders, &mon), Some(1))
//     }
//
//     #[test]
//     fn test_complete_line_fail() {
//         let mut line = SpringLine::from_str("#.#.### 1,1,4");
//         let mon = RuntimeMonitor::new_arc();
//         assert_eq!(one_line(&mut line.springs, &line.orders, &mon), None)
//     }
//
//     #[test]
//     fn test_partial_line_pass() {
//         let mut line = SpringLine::from_str("??.#.### 1,1,3");
//         let mon = RuntimeMonitor::new_arc();
//         let res = one_line(&mut line.springs, &line.orders, &mon);
//         println!("{:?}", mon.lock().unwrap().current_progress);
//         assert_eq!(res, Some(2))
//
//     }
//
//     #[test]
//     fn test_partial_line_fail() {
//         let mut line = SpringLine::from_str("??.#.### 1,1,4");
//         let mon = RuntimeMonitor::new_arc();
//         let res = one_line(&mut line.springs, &line.orders, &mon);
//         assert_eq!(res, None)
//
//     }
// }
