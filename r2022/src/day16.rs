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
}

impl Valve {
    pub fn options(&self, is_open: bool) ->  impl Iterator<Item=Action> + '_ {
        let moves = self.links.iter().map(|valve_id| Action::Move {to: valve_id.to_string() });
        let mut all = once(Action::Open).chain(moves);

        if is_open {
            // return moves
            // return empty().chain(moves)
            all.next();
        }
        all
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ValveState {
    open: bool,
    // Record what the flow was the last time this node was visited
    visited_at_flow: Option<usize>
}

impl ValveState {
    pub fn new() -> ValveState {
        ValveState { open: false, visited_at_flow: None }
    }
}

pub struct TunnelMap {
    valves: HashMap<ValveId, Valve>,
    state: HashMap<ValveId, ValveState>,
}

impl TunnelMap {
    pub fn update_visit(&mut self, valve_id: &ValveId, current_flow: usize) -> Option<usize> {
        // If we've already been here, and haven't turned any valves since, cancel it
        let mut valve = self.state[valve_id];
        if let Some(flow) = valve.visited_at_flow {
            if flow == current_flow {
                return None;
            }
        }
        let old = valve.visited_at_flow;
        valve.visited_at_flow = Some(current_flow);
        old
    }

    pub fn open(&mut self, valve_id: &ValveId) {
        let mut valve = self.state[valve_id];
        if valve.open {
            panic!("Shouldn't reach here");
        } else {
            valve.open = true;
        }
    }

    pub fn close(&mut self, valve_id: &ValveId) {
        let mut valve = self.state[valve_id];
        if !valve.open {
            panic!("Shouldn't reach here");
        } else {
            valve.open = false;
        }
    }

    pub fn restore(&mut self, valve_id: &ValveId, old_visit: Option<usize>) {
        let mut valve = self.state[valve_id];
        valve = ValveState{ open: valve.open, visited_at_flow: old_visit };
    }

    pub fn is_open(&self, valve_id: &ValveId) -> bool {
        // If we've already been here, and haven't turned any valves since, cancel it
        self.state[valve_id].open
    }
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
            (name.clone(), Valve {id: name.clone(), name , flow: (&caps["flow"]).parse().unwrap(), links })
        });

        let valves = HashMap::from_iter(valve_vec);
        let state_vec = valves.iter().map(|(key, valve)| {
            (key.clone(), ValveState::new())
        });

        let state = HashMap::from_iter(state_vec);

        Ok(TunnelMap{ valves, state })
    }
}

pub fn prepare(input: String) -> Input1 {
    TunnelMap::from_str(&*input).unwrap()
}

pub fn inner(tunnel: &mut TunnelMap, current: &Valve, total_flow : usize, current_flow: usize, current_iteration: usize, ex_info: &mut StackInfo) -> usize {
    if current_iteration == 30 {
        return total_flow + current_flow
    }

    let current_state = tunnel.state[&current.id];
    let options = current.options(current_state.open);
    let opts_len = options.size_hint().1.unwrap();
    options.enumerate().filter_map(|(i, a)| {
        ex_info.update_depth_iterations(current_iteration, i, opts_len);
        match a {
            Action::Move { to } => {
                let old = tunnel.update_visit(&to, current_flow);
                // let old_state = ValveState { open: tunnel.is_open(&to), visited_at_flow: old };
                if old.is_none() {
                    return None;
                }
                let mut new_valve = tunnel.valves.get(&*to).unwrap().clone();
                let res = inner(tunnel, &mut new_valve, total_flow + current_flow, current_flow, current_iteration + 1, ex_info);
                tunnel.restore(&to, old);
                Some(res)
            },
            Action::Open => {
                tunnel.open(&current.id);
                let res = inner(tunnel, current, total_flow + current_flow, current_flow + current.flow, current_iteration + 1, ex_info);
                tunnel.close(&current.id);
                Some(res)
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
