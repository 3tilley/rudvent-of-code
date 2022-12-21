use crate::solution::{Example, StructSolution};
use crate::stack_analysis::StackInfo;
use crate::utils::add;
use crate::DayData;
use std::fmt;
use std::str::{from_utf8_unchecked, FromStr};

type Input1 = HeightMap;
type Output1 = usize;
type Input2 = HeightMap;
type Output2 = usize;

pub struct HeightMap {
    // This is in XY
    heights: Vec<Vec<u8>>,
    x_len: usize,
    y_len: usize,
    current_position: (usize, usize),
    target_position: (usize, usize),
    visited: Vec<(usize, usize)>,
    min_so_far: Option<usize>,
    cheapest_journey: Vec<Vec<usize>>,
    max_dim: usize,
}

impl HeightMap {
    pub fn options(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
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
                        } else {
                            Some((x, y))
                        }
                    }
                },
            )
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

    pub fn take_step(&mut self, new_pos: (usize, usize)) {
        self.visited.push(new_pos);
        self.current_position = new_pos
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
            visited: vec![current_position],
            min_so_far: None,
            cheapest_journey: cheapest,
            max_dim,
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

fn print_cheapest(height_map: &HeightMap) -> () {
    print!("\n");
    for y in (0..height_map.y_len).rev() {
        for x in (0..height_map.x_len) {
            let spot = height_map.cheapest_journey[x][y];
            let clone = spot.clone().to_string();
            let char: char = if spot == height_map.max_dim {
                'm'
            } else if spot < 10 {
                clone.chars().next().unwrap()
            } else if spot < 100 {
                'x'
            } else {
                'X'
            };
            print!("{}", char);
        }
        print!("\n");
    }
    print!("\n");
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

fn info(height_map: &HeightMap) -> String {
    let has_route: f64 = height_map
        .cheapest_journey
        .iter()
        .map(|col| col.iter().filter(|c| **c != height_map.max_dim).count())
        .sum::<usize>() as f64;
    print_cheapest(height_map);
    format!(
        "Visits: {}. {:.1}% routed",
        height_map.visited.len(),
        (has_route / height_map.max_dim as f64) * 100.0
    )
}

pub fn inner(height_map: &mut HeightMap, stack_info: &mut StackInfo) -> Option<usize> {
    stack_info.update_and_show_every(100000, || info(height_map));
    // if stack_info.iteration > 20 {
    //     return None;
    // }
    if let Some(good_route) = height_map.min_so_far {
        if height_map.visited.len() >= good_route {
            return None;
        }
    }
    let opts = height_map.options().collect::<Vec<_>>();
    // let other_opts = opts.clone();
    // println!("Currently at: {:?}", height_map.current_position);
    let lens = opts
        .into_iter()
        .enumerate()
        .filter_map(|(i, new_pos)| {
            let route_len = height_map.visited.len();
            if height_map.cheapest_journey[new_pos.0][new_pos.1] < route_len {
                return None;
            } else {
                height_map.cheapest_journey[new_pos.0][new_pos.1] = route_len;
            }
            // println!("{} Visited: {:?}", i, height_map.visited);
            // println!("{} Considering: {:?} of {:?}", i, new_pos, other_opts);
            if new_pos == height_map.target_position {
                let route_len = height_map.visited.len() + 1;
                println!("Target found: {:?}. Route length: {}", new_pos, route_len);
                // println!("Route: {:?}", height_map.visited);
                height_map.min_so_far = Some(route_len);
                Some(route_len)
            } else if height_map.visited.contains(&new_pos) {
                // println!("Already visited: {:?}", new_pos);
                None
            } else {
                // println!("Trying: {:?}", new_pos);
                height_map.take_step(new_pos);
                let res = inner(height_map, stack_info);
                stack_info.iteration -= 1;
                height_map.visited.pop();
                // if let None = res {
                //     height_map.visited.pop();
                //     // stack_info.iteration -= 1;
                // }
                res
            }
        })
        .collect::<Vec<_>>();
    if lens.len() > 0 {
        lens.iter().min().map(|o| *o)
    } else {
        // height_map.visited.pop();
        // stack_info.iteration -= 1;
        None
    }
}

pub fn part_1(mut input: Input1) -> Output1 {
    println!("{:?}", input.heights);
    println!("{:?}", input);
    let mut stack_info = StackInfo::new();
    println!("Max Length: {}", input.x_len * input.y_len);
    let results = inner(&mut input, &mut stack_info);
    stack_info.update_and_show();
    println!("{:?}", input.cheapest_journey);
    results.map(|r| r - 1).unwrap_or(0)
}

pub fn part_2(input: Input2) -> Output2 {
    todo!("Implement part 2")
}

pub fn make_sol() -> StructSolution<Input1, Output1, Input2, Output2> {
    let struct_solution = StructSolution {
        prepare_part_1: prepare,
        calc_part_1: part_1,
        prepare_part_2: prepare,
        calc_part_2: part_2,
        example_part_1: Example::Value(31),
        example_part_2: Example::Value(0),
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
}
