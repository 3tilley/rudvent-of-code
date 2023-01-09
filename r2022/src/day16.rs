use std::collections::HashMap;
use std::iter::{empty, once};
use std::str::FromStr;
use regex::Regex;
use crate::solution::{Example, StructSolution};
use crate::stack_analysis::StackInfo;
use crate::DayData;

type Input1 = TunnelMap;
type Output1 = usize;
type Input2 = TunnelMap;
type Output2 = usize;
type ExampleParam = ();

type ValveId = String;

#[derive(Debug, Clone)]
pub struct Valve {
    id: ValveId,
    name: String,
    flow: usize,
    links: Vec<ValveId>,
    open: bool,
    // Record what the flow was the last time this node was visited
    visited_at_flow: Option<usize>
}

impl Valve {
    pub fn options(&self) ->  impl Iterator<Item=Action> + '_ {
        let moves = self.links.iter().map(|valve_id| Action::Move {to: valve_id.to_string() });
        let mut all = once(Action::Open).chain(moves);

        if self.open {
            // return moves
            // return empty().chain(moves)
            all.next();
        }
        all
    }
}

pub struct TunnelMap {
    valves: HashMap<ValveId, Valve>,
}

pub enum Action {
    Open,
    Move {to: ValveId}
}

impl FromStr for TunnelMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
        // For some reason they have carefully pluralised this
        // Valve HH has flow rate=22; tunnel leads to valve DD, II, BB
        let re = Regex::new(r"Valve (?P<valve>\w+) has flow rate=(?P<flow>\d+); tunnels? leads? to valves? (?P<links>.*)").unwrap();
        let valve_vec = s.lines().map(|line| {
            let caps = re.captures(line).expect(&format!("Error unpacking {}", line));
            let links: Vec<String> = caps["links"].split(", ").map(str::to_string).collect();
            let name = (&caps["valve"]).to_string();
            (name.clone(), Valve {id: name.clone(), name , flow: (&caps["flow"]).parse().unwrap(), links, open: false, visited_at_flow: None })
        });

        let valves = HashMap::from_iter(valve_vec);

        Ok(TunnelMap{ valves })
    }
}

pub fn prepare(input: String) -> Input1 {
    TunnelMap::from_str(&*input).unwrap()
}

pub fn inner(tunnel: &mut TunnelMap, current: &mut Valve, total_flow : usize, current_flow: usize, current_iteration: usize, ex_info: &mut StackInfo) -> usize {
    if current_iteration == 30 {
        return total_flow + current_flow
    }

    let options = &current.options();
    // let opts_len = &options.size_hint().1.unwrap();
    options.enumerate().filter_map(|(i, a)| {
        ex_info.update_depth_iterations(current_iteration, i, *opts_len);
        match a {
            Action::Move { to } => {
                let mut new_valve = tunnel.valves.get(&*to).unwrap().clone();
                // If we've already been here, and haven't turned any valves since, cancel it
                if let Some(last) = new_valve.visited_at_flow {
                    if last == current_flow {
                        return None;
                    } else {
                        new_valve.visited_at_flow = Some(current_flow);
                    }
                }
                let res = inner(tunnel, &mut new_valve, total_flow + current_flow, current_flow, current_iteration + 1, ex_info);
                Some(res)
            },
            Action::Open => {
                current.open = true;
                Some(inner(tunnel, current, total_flow + current_flow, current_flow + current.flow, current_iteration + 1, ex_info))
            },
        }
    }).max().unwrap()
}

pub fn part_1(mut input: Input1, run_parameter: &ExampleParam, ex_info: &mut StackInfo) -> Output1 {
    let mut start = input.valves.get("AA").unwrap().clone();
    inner(&mut input, &mut start, 0, 0, 1, ex_info )
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
