use crate::solution::{DayArguments, Example, StructSolution};
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
type Args = DayArgs;

type ValveId = String;

#[derive(Debug, Clone)]
pub struct DayArgs {
    moves: usize,
}

impl DayArgs {
    pub fn from_vec(mut vec: Vec<(String, String)>) -> DayArgs {
        let moves = match vec.iter().position(|(k, v)| {k == "moves"}) {
            None => 30,
            Some(i) => {
                let val = vec.remove(i);
                val.1.parse().unwrap()
            },
        };
        if vec.len() > 0 {
            panic!("Unrecognised elements in vec initialiser")
        } else {
            DayArgs{ moves }
        }
    }
}

impl DayArguments for DayArgs {}

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
    visitor_en_route: bool,
}

impl ValveState {
    pub fn new() -> ValveState {
        ValveState {
            flowing: false,
            visited_at_flow: None,
            visitor_en_route: false,
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

    /// Returns success if the operation was applied. No state change for false
    pub fn open(&mut self, valve_id: &ValveId, iteration: usize, current_flow: usize) -> bool {
        // println!("Opening {}", valve_id);
        let open_states = self
            .state
            .iter()
            .filter_map(|(k, v)| if v.flowing | (k == valve_id) { Some(k.clone()) } else { None })
            .collect::<String>();
        // let can_open = match self.most_flows.get_mut(&*open_states) {
        //     None => {
        //         self.most_flows
        //             .insert(open_states, (iteration, current_flow));
        //         true
        //     }
        //     Some((best_iteration, best_flow)) => {
        //         if (iteration >= *best_iteration) & (current_flow < *best_flow) {
        //             false
        //         } else {
        //             self.most_flows
        //                 .insert(open_states, (iteration, current_flow));
        //             true
        //         }
        //     }
        // };
        let can_open= true;
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
            visitor_en_route: valve.visitor_en_route,
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

    pub fn options(&self) -> impl Iter<Item = ValveId> {
        self.state.iter().filter_map(|(key, val)| {
            if val.visitor_en_route | val.flowing {
                None
            } else {
                Some(key)
            }
        })
    }

    pub fn switch_en_route(&mut self, valve_id: &ValveId) {
        let mut state = self.state.get_mut(valve_id).unwrap();
        state.visitor_en_route = !state.visitor_en_route;
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


pub struct FlowTracker {
    total_flow: usize,
    current_flow: usize,
    current_step: usize,
    max_steps: usize,
    current_iteration: usize,
}

impl FlowTracker {
    pub fn new(max_steps: usize) -> FlowTracker {
        FlowTracker {
            total_flow: 0,
            current_flow: 0,
            current_step: 1,
            max_steps,
            current_iteration: 1,
        }
    }
    pub fn next_move(&self, added_flow: usize, steps: usize) -> FlowTracker {
        FlowTracker {
            total_flow: self.total_flow + (steps * self.current_flow),
            current_flow: self.current_flow + added_flow,
            current_step: self.current_step + steps,
            max_steps: self.max_steps,
            current_iteration: self.current_iteration + 1,
        }
    }

    pub fn finish(&self) -> usize {
        self.total_flow + self.steps_left() * self.current_flow
    }

    pub fn steps_left(&self) -> usize {
        (self.max_steps + 1) - self.current_step
    }
}

pub struct MoverState<'a> {
    to_state: &'a Valve,
    moves_left: usize,
}

impl MoverState {
    pub fn new<'a>(to_state: &Valve, moves_left: usize) -> MoverState<'a> {
        MoverState{ to_state, moves_left }
    }

    pub fn tick(&mut self) -> Option<usize> {
        match self.moves_left {
            0 => Some(self.to_state.flow),
            x => {
                self.moves_left -= 1;
                None
            }
        }
    }
}

pub fn inner_2(
    tunnel: &TunnelMap,
    tunnel_state: &mut TunnelState,
    mover_states: Vec<MoverState>,
    flow: FlowTracker,
    ex_info: &mut StackInfo,
) -> Option<usize> {
    if flow.current_step > flow.max_steps {
        panic!("Too many steps taken");
    }
    if flow.current_step == flow.max_steps {
        // println!("Last step - we're done");
        return Some(flow.total_flow + flow.current_flow);
    } else if tunnel_state.all_open() {
        // println!("Everything is open - we're done");
        return Some(flow.finish());
    } else {
        // println!("{} steps left!", steps_left);
    }

    let dests = tunnel_state.options();

    for mut mover_state in mover_states {
        if let Some(x) = mover_state.tick() {
            let new_flow = flow.next_move(x, 1 );
            dests.filter_map(|valve_id| {
                match tunnel_state.open(valve_id, new_flow.current_step, new_flow.current_flow) {
                    true => {
                        let res = inner_2(tunnel, tunnel_state, mover_states, new_flow, ex_info);
                        tunnel_state.close(valve_id);
                        res
                    }
                    false => None
                }
            })
        }
    }.max().or(Some(flow.total_flow + (flow.steps_left() * flow.current_flow)))
}

pub fn inner(
    tunnel: &TunnelMap,
    tunnel_state: &mut TunnelState,
    current: &Valve,
    flow: FlowTracker,
    ex_info: &mut StackInfo,
) -> Option<usize> {
    // println!("Current step: {}. At {}", current_step, current.id);
    if flow.current_step > flow.max_steps {
        panic!("Too many steps taken");
    }
    if flow.current_step == flow.max_steps {
        // println!("Last step - we're done");
        return Some(flow.total_flow + flow.current_flow);
    } else if tunnel_state.all_open() {
        // println!("Everything is open - we're done");
        return Some(flow.finish());
    } else {
        // println!("{} steps left!", steps_left);
    }

    ex_info.show_depth(Some(Every::Count { count: 10000 }));
    // println!("{}", tunnel_state.show_state());

    let current_state = tunnel_state.state[&current.id];
    let options: Vec<(ValveId, usize)> = tunnel_state
        .state
        .iter()
        .filter_map(|(k, v)| {
            let res = if (tunnel.valves[k].flow != 0) & (k != &current.id) & !v.flowing {
                let key = k.to_string();
                let d = *tunnel
                    .travel_times
                    .get(&(k.to_string(), current.id.to_string()))
                    .unwrap() as usize;
                // Plus one here to account for the time taken to open the valve
                // println!("Would take {} steps to get to and open {}. {} steps left", d+1, k, steps_left);
                if (d + 1) < flow.steps_left() {
                    Some((key.clone(), d+1))
                } else {
                    None
                }
            } else {
                None
            };
            // println!("Considering {}. Result: {:?}", k, res);
            res
        })
        .collect();
    let opts_len = options.len();
    // println!("Number of options: {}", opts_len);
    options
        .iter()
        .enumerate()
        .filter_map(|(i, (v, d))| {
            ex_info.update_depth_iterations(flow.current_iteration, i + 1, opts_len);
            let new_valve = tunnel.valves.get(v).unwrap();
            // The amount of steps we've already had plus the time to get to new valve plus opening
            let new_current_step = flow.current_step + d;
            match tunnel_state.open(v, new_current_step, flow.current_flow + new_valve.flow) {
                true => {
                    let res = inner(
                        tunnel,
                        tunnel_state,
                        new_valve,
                        flow.next_move(
                            new_valve.flow,
                            *d
                        ),
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
        .max().or(Some(flow.total_flow + (flow.steps_left() * flow.current_flow)))
}

pub fn part_1(mut input: Input1, day_args: &Args, ex_info: &mut StackInfo) -> Output1 {
    let mut start = input.valves.get("AA").unwrap().clone();
    let mut state = TunnelState::new(&input);
    let flow = FlowTracker::new(day_args.moves);
    inner(&input, &mut state, &mut start, flow, ex_info).unwrap()
}

pub fn part_2(mut input: Input1, day_args: &Args, ex_info: &mut StackInfo) -> Output1 {
    todo!("Implement part 2")
}

pub fn make_sol(runtime_args: Vec<(String, String)>) -> StructSolution<Input1, Output1, Input2, Output2, Args> {
    let struct_solution = StructSolution {
        prepare_part_1: prepare,
        calc_part_1: part_1,
        prepare_part_2: prepare,
        calc_part_2: part_2,
        example_part_1: Example::Value(1651),
        example_part_2: Example::Value(0),
        day_args: Args::from_vec(runtime_args),
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
