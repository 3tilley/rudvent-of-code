use std::collections::HashMap;
use std::str::FromStr;
use regex::Regex;
use crate::solution::{Example, StructSolution};
use crate::stack_analysis::StackInfo;
use crate::DayData;

type Input1 = Vec<u64>;
type Output1 = usize;
type Input2 = Vec<u64>;
type Output2 = usize;
type ExampleParam = ();

pub struct Valve {
    name: String,
    flow: usize,
    links: Vec<String>,
}

pub struct TunnelMap {
    valves: HashMap<String, Valve>,
}

impl FromStr for TunnelMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
        let re = Regex::new(r"Valve (?P<valve>\s+)  # valve name
 has flow rate=(?P<rate>\d+); # flow rate
 tunnels lead to valves (?P<links>.*) # the month
").unwrap();
        let valve_vec.lines().map(|line| {
            let caps = re.captures(line).unwrap();
            let links: Vec<String> = &caps["links"].split(", ").collect();
            Valve {name: (&caps["valve"]).to_string(), flow: (&caps["flow"]).parse().unwrap(), links}
        });
        

        Ok(TunnelMap{})
    }
}

pub fn prepare(input: String) -> Input1 {

    for line in input.lines() {
        todo!()
    }
    vec![1, 2, 3]
}

pub fn part_1(mut input: Input1, run_parameter: &ExampleParam, ex_info: &mut StackInfo) -> Output1 {
    todo!("Implement part 1")
}

pub fn part_2(mut input: Input1, run_parameter: &ExampleParam, ex_info: &mut StackInfo) -> Output1 {
    todo!("Implement part 2")
}

pub fn make_sol() -> StructSolution<Input1, Output1, Input2, Output2, ExampleParam, ExampleParam> {
    let struct_solution = StructSolution {
        prepare_part_1: prepare,
        calc_part_1: part_1,
        prepare_part_2: prepare,
        calc_part_2: part_2,
        example_part_1: Example::Value(1651),
        example_part_2: Example::Value(0),
        example_1_run_parameter: ((), ()),
        example_2_run_parameter: ((), ()),
        day_data: DayData::new(16, false),
    };
    struct_solution
}
