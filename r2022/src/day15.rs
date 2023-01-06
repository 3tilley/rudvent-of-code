use crate::solution::{Example, StructSolution};
use crate::DayData;
use std::cmp::{max, min};
use std::str::FromStr;

type Input1 = Cave;
type Output1 = usize;
type Input2 = Cave;
type Output2 = usize;

#[derive(Copy, Clone, Debug)]
pub enum Square {
    WorkingSensor,
    Nothing,
    Unknown,
}

#[derive(Clone, Debug)]
pub struct Cave {
    grid: Vec<Vec<Square>>,
    // source, locked beacon, manhattan
    beacon_list: Vec<((usize, usize), (usize, usize), usize)>,
    x_max: usize,
    y_max: usize,
    x_offset: i32,
    y_offset: i32,
}

fn manhattan(x1: usize, y1: usize, x2: usize, y2: usize) -> usize {
    x1.abs_diff(x2) + y1.abs_diff(y2)
}

impl Cave {
    pub fn new(beacons: Vec<((i32, i32), (i32, i32))>) -> Cave {
        let first = beacons[0].clone();
        let ranges = beacons.iter().fold(
            (
                (min(first.0 .0, first.1 .0), min(first.0 .1, first.1 .1)),
                (max(first.0 .0, first.1 .0), max(first.0 .1, first.1 .1)),
            ),
            |((min_x, min_y), (max_x, max_y)), &((s_x, s_y), (b_x, b_y))| {
                let x_min = min(min_x, min(s_x, b_x));
                let y_min = min(min_y, min(s_y, b_y));
                let x_max = max(max_x, max(s_x, b_x));
                let y_max = max(max_y, max(s_y, b_y));
                ((x_min, y_min), (x_max, y_max))
            },
        );
        let (x_offset, y_offset) = (-(ranges.0 .0), -(ranges.0 .1));
        let x_max = (ranges.1 .0 + x_offset + 1) as usize;
        let y_max = (ranges.1 .1 + y_offset + 1) as usize;
        let mut grid = vec![vec![Square::Unknown; y_max]; x_max];
        let beacon_list: Vec<((usize, usize), (usize, usize), usize)> = beacons
            .iter()
            .map(|&((s_x, s_y), (b_x, b_y))| {
                let new_s = Cave::con(s_x, s_y, x_offset, y_offset);
                let new_b = Cave::con(b_x, b_y, x_offset, y_offset);
                let man = manhattan(new_s.0, new_s.1, new_b.0, new_b.1);
                (new_s, new_b, man)
            })
            .collect();
        for (s, b, _) in &beacon_list {
            grid[s.0][s.1] = Square::WorkingSensor;
            grid[b.0][b.1] = Square::WorkingSensor;
        }
        Cave {
            grid,
            beacon_list,
            x_max,
            y_max,
            x_offset,
            y_offset,
        }
    }

    pub fn convert(&self, x: i32, y: i32) -> (usize, usize) {
        Cave::con(x, y, self.x_offset, self.y_offset)
    }

    fn con(x: i32, y: i32, x_offset: i32, y_offset: i32) -> (usize, usize) {
        ((x + x_offset) as usize, (y + y_offset) as usize)
    }
}

impl FromStr for Cave {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let beacons = s
            .lines()
            .map(|line| {
                let (source, found) = line.split_once(":").unwrap();
                let (s_x_, s_y_) = source.split_once(", ").unwrap();
                let s_x = s_x_.split_once("x=").unwrap().1.parse().unwrap();
                let s_y = s_y_.split_once("y=").unwrap().1.parse().unwrap();
                let (b_x_, b_y_) = found.split_once(", ").unwrap();
                let b_x = b_x_.split_once("x=").unwrap().1.parse().unwrap();
                let b_y = b_y_.split_once("y=").unwrap().1.parse().unwrap();
                ((s_x, s_y), (b_x, b_y))
            })
            .collect();
        Ok(Cave::new(beacons))
    }
}

pub fn prepare(input: String) -> Input1 {
    input.parse().unwrap()
}

pub fn part_1(input: Input1) -> Output1 {
    todo!("Implement part 1")
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
        example_part_1: Example::Value(26),
        example_part_2: Example::Value(0),
        day_data: DayData::new(15, false),
    };
    struct_solution
}
