use crate::utils;
use array2d::Array2D;
use std::cmp::{max, min};
use std::fmt;
use std::ops::Range;
use std::path::Display;

pub struct Vent {
    start: (u32, u32),
    end: (u32, u32),
}

impl Vent {
    fn xs(&self) -> Vec<u32> {
        (min(self.start.0, self.end.0)..(max(self.start.0, self.end.0))+1).collect()
    }
    fn ys(&self) -> Vec<u32> {
        (min(self.start.1, self.end.1)..(max(self.start.1, self.end.1))+1).collect()
    }
    fn points(&self) -> Vec<(u32, u32)> {
        let (x_diff, y_diff, len) = match (self.start.0 as i32 - self.end.0 as i32, self.start.1 as i32 - self.end.1 as i32) {
            (0, diff) => {
                (0, diff / diff.abs(), diff.abs())
            }
            (diff, 0) => {
                (diff / diff.abs(), 0, diff.abs())
            }
            (x_diff, y_diff) => {
                if x_diff.abs() != y_diff.abs() {
                    panic!("Vectors aren't at 45 degree angle");
                } else {
                    (x_diff / x_diff.abs(), y_diff / y_diff.abs(), x_diff.abs())
                }
            }
        };
        let mut output = Vec::new();
        let start_x = self.start.0 as i32;
        let start_y = self.start.1 as i32;
        for i in 0..len {
            output.push(((start_x + (i * x_diff)) as u32, (start_y + (i * y_diff)) as u32));
        }
        output
        }

    fn from_str(line: &str) -> Vent {
        let (start, end) = line.trim().split_once(" -> ").unwrap();
        let (x_1, y_1) = start.split_once(",").unwrap();
        let (x_2, y_2) = end.split_once(",").unwrap();
        Vent {
            start: (x_1.parse::<u32>().unwrap(), y_1.parse::<u32>().unwrap()),
            end: (x_2.parse::<u32>().unwrap(), y_2.parse::<u32>().unwrap()),
        }
    }
    fn max(&self) -> (u32, u32) {
        (max(self.start.0, self.end.0), max(self.start.1, self.end.1))
    }

    fn is_diagonal(&self) -> bool {
        !((self.start.0 == self.end.0) || (self.start.1 == self.end.1))
    }
}

impl fmt::Display for Vent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}) -> ({}, {})", self.start.0, self.start.1, self.end.0, self.end.1)
    }
}

#[derive(Clone)]
pub struct Seafloor {
    floor: array2d::Array2D<u32>,
}

impl Seafloor {
    fn fill_vent(&mut self, vent: Vent) {
        log::info!("{}", vent);
        log::info!("is_diagonal: {}", vent.is_diagonal());
        if vent.is_diagonal() {
            let ys = vent.ys();
            for (i, &x) in vent.xs().iter().enumerate() {
                *self.floor.get_mut(x as usize, ys[i] as usize).unwrap() += 1;
            }

        } else {
            for x in vent.xs() {
                for y in vent.ys() {
                    *self.floor.get_mut(x as usize, y as usize).unwrap() += 1;
                }
            }
        }
    }
    fn above_x(&self, depth: u32) -> usize {
        self.floor
            .elements_column_major_iter()
            .filter(|&x| x >= &depth)
            .count()
    }
}

impl fmt::Debug for Seafloor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Display for Seafloor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ls = String::new();
        for r in self.floor.rows_iter() {
            for &e in r {
                if e == 0 {
                    ls.push_str(".");
                } else {
                    ls.push_str(&e.to_string());
                }
            }
            ls.push_str("\n");
        }
        write!(f, "{}", ls)
    }
}
pub fn load_data(name: &str) -> Vec<Vent> {
    let contents = utils::read_file(name, file!());

    let vents = contents.lines().map(|x| Vent::from_str(x));
    vents.collect()
}

pub fn a(name: &str) -> u32 {
    let input = load_data(name);
    let largest_x = input.iter().map(|x| x.max().0).max().unwrap() as usize;
    let largest_y = input.iter().map(|y| y.max().0).max().unwrap() as usize;
    let mut seafloor = Seafloor {
        floor: array2d::Array2D::filled_with(0, largest_x + 1, largest_y + 1),
    };
    for vent in input {
        if !vent.is_diagonal() {
            seafloor.fill_vent(vent);
        }
    }
    seafloor.above_x(2) as u32
}
pub fn b(name: &str) -> u32 {
    let input = load_data(name);
    let largest_x = input.iter().map(|x| x.max().0).max().unwrap() as usize;
    let largest_y = input.iter().map(|y| y.max().0).max().unwrap() as usize;
    let mut seafloor = Seafloor {
        floor: array2d::Array2D::filled_with(0, largest_x + 1, largest_y + 1),
    };
    for vent in input {
        seafloor.fill_vent(vent);
        log::debug!("\n{:?}", &seafloor);
    }
    seafloor.above_x(2) as u32
}
