use crate::solution::{Example, StructSolution};
use crate::stack_analysis::StackInfo;
use crate::DayData;
use color_eyre::owo_colors::DynColors::Xterm;
use num::{Integer, ToPrimitive};
use std::cmp::{max, min};
use std::str::FromStr;

type Input1 = Cave;
type Output1 = usize;
type Input2 = Cave;
type Output2 = usize;
type ExampleParam = i32;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Sensor {
    pub location: (i32, i32),
    pub man: usize,
}

impl Sensor {
    pub fn is_inside(&self, x: i32, y: i32) -> bool {
        manhattan(self.location.0, self.location.1, x, y) <= self.man
    }
    pub fn all_inside(&self, points: impl IntoIterator<Item = (i32, i32)>) -> bool {
        for p in points {
            if !self.is_inside(p.0, p.1) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Sector {
    pub start: (i32, i32),
    pub end: (i32, i32),
}

impl Sector {
    pub fn corners(&self) -> [(i32, i32); 4] {
        [
            (self.start.0, self.start.1),
            (self.start.0, self.end.1),
            (self.end.0, self.start.1),
            (self.end.0, self.end.1),
        ]
    }

    pub fn corners_within_sensor(&self, sensor: &Sensor) -> bool {
        sensor.all_inside(self.corners())
    }

    pub fn corners_within_any_sensor(&self, sensors: &Vec<Sensor>) -> bool {
        for s in sensors {
            if self.corners_within_sensor(s) {
                return true;
            }
        }
        return false;
    }

    pub fn check_full(&self, sensors: &Vec<Sensor>) -> Option<(i32, i32)> {
        let mut res = None;
        'outer: for x in self.start.0..=self.end.0 {
            for y in self.start.1..=self.end.1 {
                for s in sensors {
                    if s.is_inside(x, y) {
                        break;
                    }
                }
                res = Some((x, y));
                break 'outer;
            }
        }
        res
    }
}

fn split_into_sectors(start: (i32, i32), end: (i32, i32), partitions: usize) -> Vec<Sector> {
    let x_len = (end.0 - start.0) / (partitions as i32);
    let y_len = (end.1 - start.1) / (partitions as i32);
    let mut secs = Vec::new();
    for i in 0..=(partitions as i32) {
        for j in 0..=(partitions as i32) {
            let sector = Sector {
                start: (start.0 + i * x_len, start.1 + j * y_len),
                end: (
                    min(end.0, start.0 + (i + 1) * x_len - 1),
                    min(end.1, start.1 + (j + 1) * y_len - 1),
                ),
            };
            secs.push(sector);
        }
    }
    secs
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Square {
    Sensor,
    WorkingBeacon,
    Nothing,
    Unknown,
}

#[derive(Clone, Debug)]
pub struct Cave {
    // grid: Vec<Vec<Square>>,
    // source, locked beacon, manhattan
    beacon_list: Vec<((usize, usize), (usize, usize), usize)>,
    x_max: usize,
    y_max: usize,
    x_offset: i32,
    y_offset: i32,
}

fn manhattan<T>(x1: T, y1: T, x2: T, y2: T) -> usize
where
    T: Integer + ToPrimitive,
{
    let a = if x1 > x2 { x1 - x2 } else { x2 - x1 };
    let b = if y1 > y2 { y1 - y2 } else { y2 - y1 };
    (a.to_usize().unwrap()) + (b.to_usize().unwrap())
}

impl Cave {
    pub fn new(beacons: Vec<((i32, i32), (i32, i32))>) -> Cave {
        let beacon_list_man: Vec<((i32, i32), (i32, i32), usize)> = beacons
            .into_iter()
            .map(|((s_x, s_y), (b_x, b_y))| {
                let man = manhattan(s_x, s_y, b_x, b_y);
                ((s_x, s_y), (b_x, b_y), man)
            })
            .collect();
        let first = beacon_list_man[0].clone();
        let ranges = beacon_list_man.iter().fold(
            (
                (
                    min(first.0 .0 - (first.2 as i32), first.1 .0),
                    min(first.0 .1 - (first.2 as i32), first.1 .1),
                ),
                (
                    max(first.0 .0 + (first.2 as i32), first.1 .0),
                    max(first.0 .1 + (first.2 as i32), first.1 .1),
                ),
            ),
            |((min_x, min_y), (max_x, max_y)), &((s_x, s_y), (b_x, b_y), man)| {
                let man = man as i32;
                let x_min = min(min_x, min(s_x - man, b_x));
                let y_min = min(min_y, min(s_y - man, b_y));
                let x_max = max(max_x, max(s_x + man, b_x));
                let y_max = max(max_y, max(s_y + man, b_y));
                ((x_min, y_min), (x_max, y_max))
            },
        );
        let (x_offset, y_offset) = (-(ranges.0 .0), -(ranges.0 .1));
        let x_max = (ranges.1 .0 + x_offset + 1) as usize;
        let y_max = (ranges.1 .1 + y_offset + 1) as usize;
        // let mut grid = vec![vec![Square::Unknown; y_max]; x_max];
        let beacon_list: Vec<((usize, usize), (usize, usize), usize)> = beacon_list_man
            .into_iter()
            .map(|((s_x, s_y), (b_x, b_y), man)| {
                let new_s = Cave::con(s_x, s_y, x_offset, y_offset);
                let new_b = Cave::con(b_x, b_y, x_offset, y_offset);
                (new_s, new_b, man)
            })
            .collect();
        // for (s, b, _) in &beacon_list {
        //     grid[s.0][s.1] = Square::Sensor;
        //     grid[b.0][b.1] = Square::WorkingBeacon;
        // }
        Cave {
            // grid,
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

pub fn part_1(mut input: Input1, run_parameter: &ExampleParam, ex_info: &mut StackInfo) -> Output1 {
    println!("x_max: {:?}, y_max: {:?}", input.x_max, input.y_max);
    // This won't work if the offset is negative
    let t_y = input.convert(0, *run_parameter).1;
    let mut row = vec![Square::Unknown; input.x_max];
    for &((s_x, s_y), (b_x, b_y), man) in &input.beacon_list {
        if s_y == t_y {
            row[s_x] = Square::Sensor
        }
        if b_y == t_y {
            row[b_x] = Square::WorkingBeacon
        }
    }
    for x in (0..input.x_max) {
        match row[x] {
            Square::Sensor => (),
            Square::WorkingBeacon => (),
            Square::Nothing => (),
            Square::Unknown => {
                for &((s_x, s_y), (b_x, b_y), man) in &input.beacon_list {
                    if manhattan(s_x, s_y, x, t_y) <= man {
                        row[x] = Square::Nothing;
                    }
                }
            }
        }
    }
    // println!("Current row: {}", t_y);
    row.iter()
        .filter(|&s| (*s != Square::Unknown) & (*s != Square::WorkingBeacon))
        .count()
}

pub fn part_2(input: Input2, run_parameter: &ExampleParam, ex_info: &mut StackInfo) -> Output2 {
    // This won't work if the offset is negative
    let (x_min, y_min) = input.convert(0, 0);
    let (x_min, y_min) = input.convert(*run_parameter, *run_parameter);

    let beacon_list = {
        let mut b: Vec<((i32, i32), (i32, i32), usize)> = input
            .beacon_list
            .into_iter()
            .map(|((s_x, s_y), (b_x, b_y), man)| {
                (
                    (s_x as i32 - input.x_offset, s_y as i32 - input.y_offset),
                    (b_x as i32 - input.x_offset, b_y as i32 - input.y_offset),
                    man,
                )
            })
            .collect();
        b.sort_unstable_by_key(|(_, _, man)| usize::MAX - man);
        b
    };

    let sensors = beacon_list
        .iter()
        .map(|&(s, b, m)| Sensor {
            location: s,
            man: m,
        })
        .collect::<Vec<_>>();

    let secs = split_into_sectors((0, 0), (*run_parameter, *run_parameter), 1);
    let sector = secs.first().unwrap();
    let res = sector.check_full(&sensors);

    // println!("Counting");
    // let mut counter = 0usize;
    // let first = beacon_list.first().unwrap();
    // for x in 0..=(*run_parameter / 100) {
    //     for y in 0..=(*run_parameter / 100) {
    //         if manhattan(x, y, first.0 .0, first.0 .1) <= first.2 {
    //             counter += 1;
    //         }
    //     }
    // }
    // println!("Counter: {}", counter);
    // println!(
    //     "Allocating grid size {:?} x {:?}",
    //     run_parameter, run_parameter
    // );
    // let mut grid =
    //     vec![vec![Square::Unknown; (*run_parameter + 1) as usize]; (*run_parameter + 1) as usize];
    // println!("Allocated");
    //
    // println!("{:?} beacons", beacon_list.len());
    // let mut counter = 0;
    // for &((s_x, s_y), (b_x, b_y), man) in &beacon_list {
    //     counter += 1;
    //     println!("{:?} beacon", counter);
    //     let man = man as i32;
    //     for x in max(0, s_x - man)..=min(*run_parameter, s_x + man) {
    //         for y in max(0, s_y - man)..=min(*run_parameter, s_y + man) {
    //             let (xu, yu) = (x as usize, y as usize);
    //             if (x, y) == (s_x, s_y) {
    //                 grid[xu][yu] = Square::Sensor;
    //             } else if (x, y) == (b_x, b_y) {
    //                 grid[xu][yu] = Square::WorkingBeacon;
    //             } else if (grid[xu][yu] == Square::Unknown)
    //                 & (manhattan(x, y, s_x, s_y) <= (man as usize))
    //             {
    //                 grid[xu][yu] = Square::Nothing;
    //             }
    //         }
    //     }
    // }
    // // let cs: String = grid
    // //     .iter()
    // //     .map(|col| {
    // //         let mut s: String = col
    // //             .iter()
    // //             .map(|s| match s {
    // //                 Square::Unknown => ".",
    // //                 Square::Nothing => "#",
    // //                 Square::Sensor => "S",
    // //                 Square::WorkingBeacon => "B",
    // //             })
    // //             .collect();
    // //         s + "\n"
    // //     })
    // //     .collect();
    // // println!("{:?}", cs);
    //
    // // let mut res = None;
    // // 'outer: for x in 0..=(*run_parameter as usize) {
    // //     for y in 0..=(*run_parameter as usize) {
    // //         let mut found = true;
    // //         for &((s_x, s_y), (b_x, b_y), man) in &beacon_list {
    // //             if ((x as i32, y as i32) == (b_x, b_y)) | ((x as i32, y as i32) == (s_x, s_y)) {
    // //                 found = false;
    // //                 break;
    // //             } else if manhattan(s_x, s_y, x as i32, y as i32) <= man {
    // //                 found = false;
    // //                 break;
    // //             }
    // //         }
    // //         if found {
    // //             res = Some((x, y));
    // //             break 'outer;
    // //         }
    // //     }
    // // }
    // let mut res = None;
    // 'outer: for (x, col) in grid.iter().enumerate() {
    //     for (y, &s) in col.iter().enumerate() {
    //         if (s == Square::Unknown) {
    //             res = Some((x, y));
    //             break 'outer;
    //         }
    //     }
    // }
    let (x, y) = res.unwrap();
    // let (x, y) = (0, 0);
    ((4_000_000 * x) + y) as Output2
}

pub fn make_sol() -> StructSolution<Input1, Output1, Input2, Output2, ExampleParam, ExampleParam> {
    let struct_solution = StructSolution {
        prepare_part_1: prepare,
        calc_part_1: part_1,
        prepare_part_2: prepare,
        calc_part_2: part_2,
        example_part_1: Example::Value(26),
        example_part_2: Example::Value(56_000_011),
        example_1_run_parameter: (10, 2_000_000),
        example_2_run_parameter: (20, 4_000_000),
        day_data: DayData::new(15, false),
    };
    struct_solution
}
