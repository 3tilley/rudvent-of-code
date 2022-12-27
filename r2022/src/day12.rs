use crate::solution::{Example, StructSolution};
use crate::stack_analysis::StackInfo;
use crate::utils::add;
use crate::DayData;
use chrono::Utc;
use chrono_humanize::HumanTime;
use std::cmp::max;
use std::collections::HashMap;
use std::fmt;
use std::str::{from_utf8_unchecked, FromStr};
use std::thread::sleep;
use std::time::Duration;

type Input1 = HeightMap;
type Output1 = usize;
type Input2 = HeightMap;
type Output2 = usize;

#[derive(Clone)]
pub struct HeightMap {
    // This is in XY
    heights: Vec<Vec<u8>>,
    x_len: usize,
    y_len: usize,
    current_position: (usize, usize),
    target_position: (usize, usize),
    start_position: (usize, usize),
    visited: Vec<(usize, usize)>,
    min_so_far: Option<usize>,
    cheapest_journey: Vec<Vec<usize>>,
    history: Vec<Vec<bool>>,
    max_dim: usize,
    max_route: Option<usize>,
    prefer_new: bool,
}

impl HeightMap {
    pub fn options(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        let mut opts = [(-1, 0), (1, 0), (0, -1), (0, 1)]
            // [(1, 0), (-1, 0), (0, -1), (0, 1)]
            .into_iter()
            .filter_map(
                |(x_, y_)| match (self.add_step(x_, true), self.add_step(y_, false)) {
                    (None, _) => None,
                    (_, None) => None,
                    (Some(x), Some(y)) => {
                        if self.heights[x][y]
                            > self.heights[self.current_position.0][self.current_position.1] + 1
                        {
                            None
                        } else if self.cheapest_journey[x][y] < self.visited.len() {
                            None
                        } else if self.history[x][y] {
                            None
                        } else {
                            let distance_from_target = self.target_position.0.abs_diff(x)
                                + self.target_position.1.abs_diff(y);
                            let improvement = self.cheapest_journey[x][y] - self.visited.len();
                            if self.prefer_new {
                                Some((
                                    self.max_dim - improvement,
                                    distance_from_target,
                                    (127 - self.heights[x][y]) as usize,
                                    x,
                                    y,
                                ))
                            } else {
                                Some((
                                    (127 - self.heights[x][y]) as usize,
                                    distance_from_target,
                                    self.max_dim - improvement,
                                    x,
                                    y,
                                ))
                            }
                        }
                    }
                },
            )
            .collect::<Vec<_>>();
        opts.sort_unstable();
        opts.into_iter().map(|(_, _, _, x, y)| (x, y))
    }

    fn add_step(&self, diff: i32, is_x: bool) -> Option<usize> {
        let res = if is_x {
            add(self.current_position.0, diff).filter(|r| r < &self.x_len)
        } else {
            add(self.current_position.1, diff).filter(|r| r < &self.y_len)
        };
        // println!("{:?}", res);
        res
    }

    fn kill_path(&mut self) {
        let last = self.visited.pop().unwrap();
        self.current_position = *self.visited.last().unwrap();
        self.history[last.0][last.1] = false;
    }

    pub fn take_step(&mut self, new_pos: (usize, usize)) {
        if self.visited.len() > self.cheapest_journey[new_pos.0][new_pos.1] {
            panic!("Fuckup at {:?}", new_pos);
        }
        self.cheapest_journey[new_pos.0][new_pos.1] = self.visited.len();
        self.visited.push(new_pos);
        self.current_position = new_pos;
        self.history[new_pos.0][new_pos.1] = true;
    }

    pub fn reset(&mut self) {
        self.visited = vec![self.start_position];
        self.current_position = self.start_position;
        self.history = vec![vec![false; self.y_len]; self.x_len];
        self.cheapest_journey = vec![vec![self.max_dim; self.y_len]; self.x_len];
    }

    pub fn len_per_distance(&self) {}

    fn add_steps(&self, diff_x: i32, diff_y: i32) -> Option<(usize, usize)> {
        let x = self.add_step(diff_x, true);
        let y = self.add_step(diff_y, false);
        match (x, y) {
            (None, _) | (_, None) => None,
            (Some(x), Some(y)) => Some((x, y)),
        }
    }

    pub fn is_touching_self(&self) -> bool {
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .filter_map(|(x, y)| {
                self.add_steps(x, y)
                    .map(|xy| {
                        if self
                            .visited
                            .len()
                            .checked_sub(2)
                            .map(|i| self.visited.get(i).unwrap())
                            .map(|xy_| *xy_ == xy)
                            .unwrap_or(false)
                        {
                            false
                        } else {
                            // If the neighbour has been visited we're wrapped around ourselves
                            self.history[xy.0][xy.1]
                                & (self.heights[xy.0][xy.1] + 1
                                    >= self.heights[self.current_position.0]
                                        [self.current_position.1])
                        }
                    })
                    .filter(|b| *b)
            })
            .any(|o| o)
    }

    fn char_to_int(c: char) -> u8 {
        c as u8 - 97
    }

    fn from_str_and_target(s: &str, target_char: char) -> Result<Self, ()> {
        let mut current_position = (0, 0);
        let mut target_position = (0, 0);
        let mut first_lines = s.lines();
        let lines = s.lines();
        let x_len = first_lines.next().unwrap().len();
        println!("x_len: {}", x_len);
        let mut heights = vec![Vec::new(); x_len];
        // let first_heights: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();
        for (y, s) in s.lines().rev().enumerate() {
            for (x, c) in s.chars().enumerate() {
                match c {
                    'S' => {
                        current_position = (x, y);
                        heights[x].push(0);
                    }
                    'E' => {
                        target_position = (x, y);
                        heights[x].push(HeightMap::char_to_int(target_char));
                    }
                    char => heights[x].push(HeightMap::char_to_int(char)),
                }
            }
        }

        let x_len = heights.len();
        let y_len = heights[0].len();
        let max_dim = x_len * y_len;
        let cheapest = vec![vec![max_dim; y_len]; x_len];
        Ok(HeightMap {
            heights,
            x_len,
            y_len,
            current_position,
            target_position,
            start_position: current_position,
            visited: vec![current_position],
            min_so_far: None,
            cheapest_journey: cheapest,
            history: vec![vec![false; y_len]; x_len],
            max_dim,
            max_route: None,
            prefer_new: true,
        })
    }
}

impl fmt::Debug for HeightMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n");
        for y in (0..self.y_len).rev() {
            for x in (0..self.x_len) {
                write!(f, "{:.>2}", self.heights[x][y]);
            }
            write!(f, "\n");
        }
        write!(f, "\n")
    }
}

fn print_cheapest(height_map: &HeightMap) -> String {
    const SIZE: usize = 128 * 1000;
    // let mut s = ArrayString::<SIZE>::new();
    let mut s = String::new();
    // match write!(s, "{}{}", "0123456789", "0123456789") {
    //     Ok(_) => println!("success!"),
    //     Err(err) => println!("{}", err),
    // }

    s.push_str("\n");
    for y in (0..height_map.y_len).rev() {
        for x in (0..height_map.x_len) {
            let spot = height_map.cheapest_journey[x][y];
            let clone = spot.clone().to_string();
            let lowest = height_map.visited.len().saturating_sub(3);
            let char: char = if height_map.current_position == (x, y) {
                let last_pos = height_map.visited.last().unwrap();
                if height_map.current_position.0 > last_pos.0 {
                    '>'
                } else if height_map.current_position.0 < last_pos.0 {
                    '<'
                } else if height_map.current_position.1 > last_pos.1 {
                    '^'
                } else {
                    'v'
                }
            } else if height_map.visited[lowest..].contains(&(x, y)) {
                '*'
            } else if height_map.visited.contains(&(x, y)) {
                '.'
            } else if spot == height_map.max_dim {
                'M'
            } else if spot < 10 {
                clone.chars().next().unwrap()
            } else if spot < 100 {
                'x'
            } else {
                'X'
            };
            s.push(format!("{}", char).parse().unwrap());
        }
        s.push("\n".parse().unwrap());
    }
    s
}

impl FromStr for HeightMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        HeightMap::from_str_and_target(s, 'z')
    }
}

pub fn prepare(input: String) -> Input1 {
    HeightMap::from_str(&input).unwrap()
}

const ONE_MS: u32 = 1000000;

fn info(height_map: &HeightMap, should_sleep: bool, grab_bag: &GrabBag) -> String {
    if should_sleep {
        sleep(Duration::new(0, ONE_MS * 1));
    }
    let has_route: f64 = height_map
        .cheapest_journey
        .iter()
        .map(|col| col.iter().filter(|c| **c != height_map.max_dim).count())
        .sum::<usize>() as f64;
    let mut so_far_map = print_cheapest(height_map);
    so_far_map.push_str(&*format!(
        "Iteration: {}\nLast path failed at len: {} with reason: {:?}",
        grab_bag.iteration, grab_bag.last_len, grab_bag.reason
    ));
    so_far_map.push_str(&*format!(
        "\nReasons: {}",
        grab_bag
            .reasons
            .iter()
            .map(|(k, v)| format!("{:?}: {}", k, v))
            .collect::<Vec<_>>()
            .join("\n")
    ));
    so_far_map.push_str(&*format!(
        "\nVisits: {}. {:.1}% routed\n",
        height_map.visited.len(),
        (has_route / height_map.max_dim as f64) * 100.0
    ));
    so_far_map.push_str(&*format!("Highest so far: {:?}", grab_bag.highest_reached));
    // so_far_map.push_str(&*format!("\n\n{:?}", grab_bag.reasons));
    // let ht = HumanTime::from(Utc::now() - stack_info.start_time);
    // so_far_map.push_str(&*format!("{} per iteration", (stack_info.formatter)(Utc::now() - stack_info.start_time)));
    // This clears the screen
    // so_far_map.push_str(&*format!("{:?}", height_map.cheapest_journey));
    // print!("\x1B[2J{}", so_far_map);
    so_far_map
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub enum Reason {
    TouchesTail,
    RouteTooLong,
    BeenHereQuicker,
    CantContinue,
    NotStarted,
}

#[derive(Debug)]
pub struct GrabBag {
    reason: Reason,
    iteration: usize,
    last_len: usize,
    reasons: HashMap<Reason, usize>,
    highest_reached: u8,
}
impl GrabBag {
    fn new() -> GrabBag {
        GrabBag {
            reason: Reason::NotStarted,
            iteration: 0,
            last_len: 0,
            reasons: HashMap::from_iter(
                vec![
                    Reason::RouteTooLong,
                    Reason::CantContinue,
                    Reason::NotStarted,
                    Reason::TouchesTail,
                    Reason::BeenHereQuicker,
                ]
                .into_iter()
                .map(|r| (r, 0)),
            ),
            highest_reached: 0,
        }
    }

    fn update(&mut self, reason: Reason, last_len: usize) -> Option<usize> {
        self.iteration += 1;
        self.reason = reason;
        self.last_len = last_len;
        *self.reasons.get_mut(&reason).unwrap() += 1;
        None
    }
}

pub fn consider(height_map: &HeightMap, proposal: (usize, usize)) -> Option<Reason> {
    if let Some(route) = height_map.max_route {
        if height_map.visited.len() > route {
            return Some(Reason::RouteTooLong);
        }
    };
    let ans = if height_map.cheapest_journey[proposal.0][proposal.1] <= height_map.visited.len() {
        Some(Reason::BeenHereQuicker)
    } else if height_map.is_touching_self() {
        Some(Reason::TouchesTail)
    } else {
        None
    };
    if height_map.heights[height_map.current_position.0][height_map.current_position.1] >= 20 {
        // println!("Current: {:?}. Position: {:?} - {:?}", height_map.current_position, proposal, ans);
        // print!("{}", print_cheapest(height_map));
        // print!("{:?}", height_map.visited);
    }
    ans
}

// fn slices(height_map: HeightMap) -> Vec<Vec<Vec<usize>>> {
//
// }

pub fn inner(
    height_map: &mut HeightMap,
    stack_info: &mut StackInfo,
    grab_bag: &mut GrabBag,
) -> Option<usize> {
    if true {
        stack_info.update_and_show_every(10000000, || info(height_map, false, grab_bag));
    } else {
        stack_info.update_and_show_every(1, || info(height_map, true, grab_bag));
    }
    grab_bag.highest_reached = max(
        height_map.heights[height_map.current_position.0][height_map.current_position.1],
        grab_bag.highest_reached,
    );
    // if stack_info.iteration > 20 {
    //     return None;
    // }
    // let other_opts = opts.clone();
    // println!("Currently at: {:?}", height_map.current_position);
    let opts = height_map.options().collect::<Vec<_>>();
    if opts.len() == 0 {
        return grab_bag.update(Reason::CantContinue, height_map.visited.len());
    }
    let lens = opts
        .into_iter()
        .enumerate()
        .filter_map(|(i, new_pos)| {
            let route_len = height_map.visited.len();
            let cons = consider(height_map, new_pos);
            // println!("{} Visited: {:?}", i, height_map.visited);
            // println!("{} Considering: {:?} of {:?}", i, new_pos, other_opts);
            match cons {
                None => {
                    if new_pos == height_map.target_position {
                        let route_len = height_map.visited.len() + 1;
                        // println!("Target found: {:?}. Route length: {}", new_pos, route_len);
                        // println!("Route: {:?}", height_map.visited);
                        height_map.min_so_far = Some(route_len);
                        Some(route_len)
                    } else {
                        // println!("Trying: {:?}", new_pos);
                        height_map.take_step(new_pos);
                        let res = inner(height_map, stack_info, grab_bag);
                        stack_info.iteration -= 1;
                        height_map.kill_path();
                        res
                    }
                }
                Some(reason) => grab_bag.update(reason, height_map.visited.len()),
            }
        })
        .collect::<Vec<_>>();
    if lens.len() > 0 {
        lens.iter().min().map(|o| *o)
    } else {
        None
    }
}

pub fn part_1(mut input: Input1) -> Output1 {
    // println!("{:?}", input.heights);
    // println!("{:?}", input);
    // println!("Max Length: {}", input.x_len * input.y_len);
    let mut result = None;
    input.max_route = Some(10);
    input.prefer_new = false;
    let mut stack_info = StackInfo::new();
    let mut grab_bag = GrabBag::new();
    // for i in 0..100 {
    //     println!("Attempt: {}", i + 1);
    //     input.reset();
    //     let mut mini_stack = StackInfo::new();
    //     match inner(&mut input, &mut mini_stack) {
    //         Some(res) => {
    //             result = Some(res);
    //             break;
    //         },
    //         None => {
    //             // input.prefer_new = !input.prefer_new;
    //             input.max_route = input.max_route.map(|r| r + 1);
    //         }
    //     }
    //
    // }
    println!("{:?}", result);
    if result.is_none() {
        stack_info = StackInfo::new();
        input.reset();
        input.prefer_new = false;
        input.max_route = None;
        result = inner(&mut input, &mut stack_info, &mut grab_bag);
        stack_info.update_and_show();
    }
    // println!("{:?}", input.cheapest_journey);
    // println!("{:?}", grab_bag.reasons);
    println!(
        "Found answer in {:?} iterations",
        stack_info.total_iterations
    );
    // println!("{:?}", input.cheapest_journey);
    result.map(|r| r - 1).unwrap_or(0)
}

pub fn part_2(input: Input2) -> Output2 {
    let mut the_as = Vec::new();
    for x in 0..input.x_len {
        for y in 0..input.y_len {
            if input.heights[x][y] == 0 {
                the_as.push((x, y));
            }
        }
    }
    the_as
        .into_iter()
        .filter_map(|(x, y)| {
            let mut height_map = input.clone();
            height_map.start_position = (x, y);
            height_map.reset();
            match part_1(height_map) {
                0 => None,
                x => Some(x),
            }
        })
        .min()
        .unwrap()
}

pub fn make_sol() -> StructSolution<Input1, Output1, Input2, Output2> {
    let struct_solution = StructSolution {
        prepare_part_1: prepare,
        calc_part_1: part_1,
        prepare_part_2: prepare,
        calc_part_2: part_2,
        example_part_1: Example::Value(31),
        example_part_2: Example::Value(29),
        day_data: DayData::new(12, false),
    };
    struct_solution
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_rectangle() {
        let height_map = HeightMap::from_str_and_target("Sbc\nfaE", 'd').unwrap();
        print!("{:?}", height_map);
        assert_eq!(height_map.x_len, 3);
        assert_eq!(height_map.y_len, 2);
        let res = part_1(height_map);
        assert_eq!(res, 3);
    }

    #[test]
    fn part_1_rectangle_other_way() {
        let height_map = HeightMap::from_str_and_target("Sb\nfc\nhE", 'd').unwrap();
        print!("{:?}", height_map);
        assert_eq!(height_map.x_len, 2);
        assert_eq!(height_map.y_len, 3);
        let res = part_1(height_map);
        assert_eq!(res, 3);
    }

    #[test]
    fn generate_options() {
        let height_map = HeightMap::from_str_and_target("Sb\naE", 'd').unwrap();
        print!("{:?}", height_map);
        let opts = height_map.options().collect::<Vec<_>>();
        assert_eq!(opts, vec![(1, 1), (0, 0)]);
    }

    #[test]
    fn generate_options_bad() {
        let height_map = HeightMap::from_str_and_target("Se\naE", 'c').unwrap();
        let opts = height_map.options().collect::<Vec<_>>();
        assert_eq!(opts, vec![(0, 0)]);
    }

    #[test]
    fn test_part_1() {
        let height_map: HeightMap = HeightMap::from_str_and_target("Sa\naE", 'b').unwrap();
        let res = part_1(height_map);
        assert_eq!(res, 2);
    }

    #[test]
    fn test_part_1_bad() {
        let height_map: HeightMap = HeightMap::from_str_and_target("Sa\naE", 'd').unwrap();
        let res = part_1(height_map);
        assert_eq!(res, 0);
    }

    #[test]
    fn test_part_1_3_3() {
        let height_map: HeightMap = HeightMap::from_str_and_target("Sab\nedc\nfgE", 'h').unwrap();
        print!("{:?}", height_map.heights);
        print!("{:?}", height_map);
        let res = part_1(height_map);
        assert_eq!(res, 8);
        println!("{:?}", res);
    }

    #[test]
    fn test_part_1_history() {
        let height_map: HeightMap = HeightMap::from_str_and_target("Saa\naaa\naaE", 'c').unwrap();
        print!("{:?}", height_map.heights);
        print!("{:?}", height_map);
        let res = part_1(height_map);
        // println!("{:?}", height_map.cheapest_journey);
    }

    #[test]
    fn test_part_1_tail_touch() {
        let mut height_map: HeightMap =
            HeightMap::from_str_and_target("Sab\nedc\nfgE", 'h').unwrap();
        let mut height_map_touching: HeightMap =
            HeightMap::from_str_and_target("Sab\nadc\nfgE", 'h').unwrap();
        let path = vec![(0, 2), (1, 2), (2, 2), (2, 1), (1, 1), (0, 1)];
        height_map.current_position = (0, 1);
        height_map_touching.current_position = (0, 1);
        height_map.visited = path.clone();
        height_map_touching.visited = path.clone();
        let mut hist = vec![vec![false; 3]; 3];
        for p in path {
            hist[p.0][p.1] = true;
        }
        height_map.history = hist.clone();
        height_map_touching.history = hist.clone();
        print!("{:?}", height_map.heights);
        print!("{:?}", height_map);
        assert_eq!(height_map.is_touching_self(), false);
        assert_eq!(height_map_touching.is_touching_self(), true);
        // let res = part_1(height_map);
        // assert_eq!(res, 8);
        // println!("{:?}", res);
    }

    #[test]
    fn test_part_1_been_before() {
        let mut height_map: HeightMap =
            HeightMap::from_str_and_target("Sab\nedc\nfgE", 'h').unwrap();
        let mut height_map_touching: HeightMap =
            HeightMap::from_str_and_target("Sab\nadc\nfgE", 'h').unwrap();
        let path = vec![(0, 2), (1, 2), (2, 2), (2, 1), (1, 1)];
        height_map.current_position = (1, 1);
        height_map.visited = path.clone();
        let mut hist = vec![vec![false; 3]; 3];
        for p in path {
            hist[p.0][p.1] = true;
        }
        height_map.cheapest_journey[0][2] = 1;
        height_map.cheapest_journey[1][2] = 2;
        height_map.cheapest_journey[2][2] = 3;
        height_map.cheapest_journey[0][1] = 2;
        height_map.history = hist.clone();
        let a = consider(&height_map, (0, 1));
        assert_eq!(a, Some(Reason::BeenHereQuicker));
        print!("{:?}", height_map.heights);
        print!("{:?}", height_map);
        // let res = part_1(height_map);
        // assert_eq!(res, 8);
        // println!("{:?}", res);
    }
}
