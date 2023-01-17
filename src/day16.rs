use crate::solution::{Example, StructSolution};
use crate::stack_analysis::{Every, StackInfo};
use crate::DayData;
use regex::Regex;
use std::collections::HashMap;
use std::iter::{empty, once};
use std::str::FromStr;

type Input1 = TunnelMap;
type Output1 = usize;
type Input2 = TunnelMap;
type Output2 = usize;
type ExampleParam = ();

type ValveId = String;

const MAX_MOVES: usize = 30;

#[derive(Debug, Clone)]
pub struct Valve {
    id: ValveId,
    name: String,
    flow: usize,
    links: Vec<ValveId>,
}

impl Valve {
    pub fn options(&self, include_open: bool) -> impl Iterator<Item = Action> + '_ {
        let moves = self.links.iter().map(|valve_id| Action::Move {
            to: valve_id.to_string(),
        });
        let mut all = once(Action::Open).chain(moves);

        if !include_open {
            // return moves
            // return empty().chain(moves)
            all.next();
        }
        all
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ValveState {
    flowing: bool,
    // Record what the flow was the last time this node was visited
    visited_at_flow: Option<usize>,
}

impl ValveState {
    pub fn new() -> ValveState {
        ValveState {
            flowing: false,
            visited_at_flow: None,
        }
    }
}

pub struct TunnelMap {
    valves: HashMap<ValveId, Valve>,
    zero_flows: usize,
    travel_times: HashMap<(ValveId, ValveId), u8>,
}

impl TunnelMap {
    pub fn new(valves: HashMap<ValveId, Valve>) -> TunnelMap {
        let zero_flows = valves.iter().filter(|(k, v)| v.flow == 0).count();

        let travel_times = TunnelMap::calculate_travel_times(&valves);

        TunnelMap {
            valves,
            zero_flows,
            travel_times,
        }
    }

    pub fn calculate_travel_times(
        valves: &HashMap<ValveId, Valve>,
    ) -> HashMap<(ValveId, ValveId), u8> {
        // We need to know how many steps it takes for each valve to get to each other valve. We
        // won't add the "diagonals" i.e. the time it takes for AA to get to AA
        let expected = (valves.len() - 1) * (valves.len());
        let mut map = HashMap::<(ValveId, ValveId), u8>::with_capacity(expected);
        for (valve_id, valve) in valves {
            for link in &valve.links {
                map.insert((valve_id.clone(), link.clone()), 1);
            }
        }
        for step in 2..u8::MAX {
            // println!("Step: {}, \n{:#?}", step, map);
            let mut to_add = Vec::new();
            for ((v1, v2), &d) in &map {
                if d == (step - 1) {
                    for link in &valves.get(v2).unwrap().links {
                        if (v1 != link) & !map.contains_key(&(v1.clone(), link.clone())) {
                            to_add.push((v1.clone(), link.clone()));
                        }
                    }
                }
            }
            for new_link in to_add {
                map.insert(new_link, step);
            }
            if map.len() == expected {
                break;
            }
        }
        map
    }
}

pub struct TunnelState {
    state: HashMap<ValveId, ValveState>,
    num: usize,
    num_open: usize,
    zero_flows: usize,
    most_flows: HashMap<String, (usize, usize)>,
}

impl TunnelState {
    pub fn new(tunnel_map: &TunnelMap) -> TunnelState {
        let state_vec = tunnel_map
            .valves
            .iter()
            .map(|(key, valve)| (key.clone(), ValveState::new()));

        let state = HashMap::from_iter(state_vec);
        let num = state.len();
        let most_flows = HashMap::new();
        TunnelState {
            state,
            num: 0,
            num_open: 0,
            zero_flows: tunnel_map.zero_flows,
            most_flows,
        }
    }

    pub fn update_visit(
        &mut self,
        valve_id: &ValveId,
        current_flow: usize,
    ) -> Option<Option<usize>> {
        // If we've already been here, and haven't turned any valves since, cancel it
        let mut valve = self.state.get_mut(valve_id).unwrap();
        if let Some(flow) = valve.visited_at_flow {
            if flow == current_flow {
                return None;
            }
        }
        let old = valve.visited_at_flow;
        valve.visited_at_flow = Some(current_flow);
        Some(old)
    }

    /// Returns success if the operation was applied. Not state change for false
    pub fn open(&mut self, valve_id: &ValveId, iteration: usize, current_flow: usize) -> bool {
        // println!("Opening {}", valve_id);
        let open_states = self
            .state
            .iter()
            .filter_map(|(k, v)| if v.flowing | (k == valve_id) { Some(k.clone()) } else { None })
            .collect::<String>();
        let can_open = match self.most_flows.get_mut(&*open_states) {
            None => {
                self.most_flows
                    .insert(open_states, (iteration, current_flow));
                true
            }
            Some((best_iteration, best_flow)) => {
                if (iteration >= *best_iteration) & (current_flow <= *best_flow) {
                    false
                } else {
                    self.most_flows
                        .insert(open_states, (iteration, current_flow));
                    true
                }
            }
        };
        let mut valve = self.state.get_mut(valve_id).unwrap();
        if valve.flowing {
            panic!("Tried to open an open valve");
        } else if can_open {
            self.num_open += 1;
            valve.flowing = true;
        }
        can_open
    }

    pub fn close(&mut self, valve_id: &ValveId) {
        // println!("Closing {}", valve_id);
        let mut valve = self.state.get_mut(valve_id).unwrap();
        if !valve.flowing {
            panic!("Tried to close a closed valve");
        } else {
            self.num_open -= 1;
            valve.flowing = false;
        }
    }

    pub fn restore(&mut self, valve_id: &ValveId, old_visit: Option<usize>) {
        let mut valve = self.state.get_mut(valve_id).unwrap();
        *valve = ValveState {
            flowing: valve.flowing,
            visited_at_flow: old_visit,
        };
    }

    pub fn is_open(&self, valve_id: &ValveId) -> bool {
        // If we've already been here, and haven't turned any valves since, cancel it
        self.state[valve_id].flowing
    }

    pub fn show_state(&self) -> String {
        let mut s = String::new();
        self.state
            .iter()
            .map(|(k, v)| format!("{}, {:?}\n", k, v))
            .collect::<String>()
    }

    pub fn all_open(&self) -> bool {
        (self.num_open + self.zero_flows) == self.num
    }
}

pub enum Action {
    Open,
    Move { to: ValveId },
}

impl FromStr for TunnelMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
        // For some reason they have carefully pluralised this
        // Valve HH has flow rate=22; tunnel leads to valve DD, II, BB
        let re = Regex::new(r"Valve (?P<valve>\w+) has flow rate=(?P<flow>\d+); tunnels? leads? to valves? (?P<links>.*)").unwrap();
        let valve_vec = s.lines().map(|line| {
            let caps = re
                .captures(line)
                .expect(&format!("Error unpacking {}", line));
            let links: Vec<String> = caps["links"].split(", ").map(str::to_string).collect();
            let name = (&caps["valve"]).to_string();
            (
                name.clone(),
                Valve {
                    id: name.clone(),
                    name,
                    flow: (&caps["flow"]).parse().unwrap(),
                    links,
                },
            )
        });

        let valves = HashMap::from_iter(valve_vec);
        Ok(TunnelMap::new(valves))
    }
}

pub fn prepare(input: String) -> Input1 {
    TunnelMap::from_str(&*input).unwrap()
}

// Previous attempt at solution, just using random walks and seeing where things ended up
pub fn old_inner(
    tunnel: &TunnelMap,
    tunnel_state: &mut TunnelState,
    current: &Valve,
    total_flow: usize,
    current_flow: usize,
    current_iteration: usize,
    ex_info: &mut StackInfo,
) -> Option<usize> {
    if current_iteration == 30 {
        return Some(total_flow + current_flow);
    } else if tunnel_state.all_open() {
        return Some(total_flow + (31 - current_iteration) * current_flow);
    }

    ex_info.show_depth(Some(Every::Count { count: 100_000 }));
    // println!("{}", tunnel.show_state());

    let current_state = tunnel_state.state[&current.id];
    let include_open = !current_state.flowing & (current.flow > 0);
    let options = current.options(include_open);
    let opts_len = options.size_hint().1.unwrap();
    // println!("Number of options: {}", opts_len);
    options
        .enumerate()
        .filter_map(|(i, a)| {
            ex_info.update_depth_iterations(current_iteration, i + 1, opts_len);
            match a {
                Action::Move { to } => {
                    let old = tunnel_state.update_visit(&to, current_flow);
                    // let old_state = ValveState { open: tunnel.is_open(&to), visited_at_flow: old };
                    match old {
                        None => None,
                        Some(opt_flow) => {
                            let mut new_valve = tunnel.valves.get(&*to).unwrap();
                            let res = old_inner(
                                tunnel,
                                tunnel_state,
                                new_valve,
                                total_flow + current_flow,
                                current_flow,
                                current_iteration + 1,
                                ex_info,
                            );
                            tunnel_state.restore(&to, opt_flow);
                            res
                        }
                    }
                }
                Action::Open => {
                    match tunnel_state.open(&current.id, current_iteration, current_flow) {
                        true => {
                            let res = old_inner(
                                tunnel,
                                tunnel_state,
                                current,
                                total_flow + current_flow,
                                current_flow + current.flow,
                                current_iteration + 1,
                                ex_info,
                            );
                            tunnel_state.close(&current.id);
                            res
                        }
                        false => {
                            None
                        }
                    }
                }
            }
        })
        .max()
}

pub fn inner(
    tunnel: &TunnelMap,
    tunnel_state: &mut TunnelState,
    current: &Valve,
    total_flow: usize,
    current_flow: usize,
    current_step: usize,
    current_iteration: usize,
    ex_info: &mut StackInfo,
) -> Option<usize> {
    if current_step > MAX_MOVES {
        panic!("Too many steps taken");
    }
    if current_step == MAX_MOVES {
        return Some(total_flow + current_flow);
    } else if tunnel_state.all_open() {
        return Some(total_flow + (MAX_MOVES + 1 - current_step) * current_flow);
    }

    ex_info.show_depth(Some(Every::Count { count: 100 }));
    // println!("{}", tunnel_state.show_state());

    let current_state = tunnel_state.state[&current.id];
    let options: Vec<(ValveId, usize)> = tunnel_state
        .state
        .iter()
        .filter_map(|(k, v)| {
            if (tunnel.valves[k].flow != 0) & (k != &current.id) & !v.flowing {
                let key = k.to_string();
                let d = *tunnel
                    .travel_times
                    .get(&(k.to_string(), current.id.to_string()))
                    .unwrap() as usize;
                if d < MAX_MOVES - current_step {
                    Some((key.clone(), d))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();
    let opts_len = options.len();
    // println!("Number of options: {}", opts_len);
    options
        .iter()
        .enumerate()
        .filter_map(|(i, (v, d))| {
            ex_info.update_depth_iterations(current_iteration, i + 1, opts_len);
            let new_valve = tunnel.valves.get(v).unwrap();
            // The amount of steps we've already had plus the time to get to new valve plus opening
            let new_current_step = current_step + d + 1;
            match tunnel_state.open(v, new_current_step, current_flow + new_valve.flow) {
                true => {
                    let res = inner(
                        tunnel,
                        tunnel_state,
                        new_valve,
                        total_flow + ((d + 1) * current_flow),
                        current_flow + new_valve.flow,
                        new_current_step,
                        current_iteration + 1,
                        ex_info,
                    );
                    tunnel_state.close(v);
                    res
                }
                false => {
                    None
                }
            }
        })
        .max()
}

pub fn part_1(mut input: Input1, run_parameter: &ExampleParam, ex_info: &mut StackInfo) -> Output1 {
    let mut start = input.valves.get("AA").unwrap().clone();
    let mut state = TunnelState::new(&input);
    inner(&input, &mut state, &mut start, 0, 0, 1, 1, ex_info).unwrap()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_options() {
        let v = Valve {
            id: "".to_string(),
            name: "".to_string(),
            flow: 0,
            links: vec!["AA".to_string(), "BB".to_string()],
        };
        assert_eq!(v.options(false).count(), 3);
        assert_eq!(v.options(true).count(), 2);
    }

    #[test]
    fn basic_test() {
        // let s = ""
        ()
    }
}
