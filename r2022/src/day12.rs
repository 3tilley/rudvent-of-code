use crate::solution::{Example, StructSolution};
use crate::utils::add;
use crate::DayData;
use std::str::FromStr;

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
}

impl HeightMap {
    pub fn options(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .filter_map(
                |(x_, y_)| match (self.add_step(x_, true), self.add_step(y_, false)) {
                    (None, _) => None,
                    (_, None) => None,
                    (Some(x), Some(y)) => {
                        println!("{}{}", x, y);
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
        if is_x {
            add(self.current_position.0, diff).filter(|r| r >= &self.x_len)
        } else {
            add(self.current_position.1, diff).filter(|r| r >= &self.y_len)
        }
    }

    pub fn take_step(&mut self, new_pos: (usize, usize)) {
        self.visited.push(new_pos);
        self.current_position = new_pos
    }
}

impl FromStr for HeightMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut current_position = (0, 0);
        let mut target_position = (0, 0);
        let heights: Vec<Vec<u8>> = s
            .lines()
            .rev()
            .enumerate()
            .map(|(x, s)| {
                s.chars()
                    .enumerate()
                    .map(|(y, c)| match c {
                        'S' => {
                            current_position = (x, y);
                            0
                        }
                        'E' => {
                            target_position = (x, y);
                            25
                        }
                        x => c as u8 - 97,
                    })
                    .collect()
            })
            .collect();

        let x_len = heights.len();
        let y_len = heights[0].len();
        Ok(HeightMap {
            heights,
            x_len,
            y_len,
            current_position,
            target_position,
            visited: vec![current_position],
        })
    }
}

pub fn prepare(input: String) -> Input1 {
    HeightMap::from_str(&input).unwrap()
}

pub fn inner(height_map: &mut HeightMap) -> Option<usize> {
    let opts = height_map.options().collect::<Vec<_>>();
    println!("{:?}", opts);
    for new_pos in opts {
        if new_pos == height_map.target_position {
            height_map.visited.pop();
            return Some(height_map.visited.len() + 1);
        }
        if height_map.visited.contains(&new_pos) {
            height_map.visited.pop();
            return None;
        } else {
            height_map.take_step(new_pos);
            let res = inner(height_map);
            height_map.visited.pop();
            return res;
        }
    }
    height_map.visited.pop();
    None
}

pub fn part_1(mut input: Input1) -> Output1 {
    let results = inner(&mut input);
    results.unwrap()
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
