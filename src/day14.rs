use std::cmp::{max, min};
use std::str::FromStr;
use crate::solution::{Example, StructSolution};
use crate::DayData;
use crate::utils::add;

type Input1 = Cave;
type Output1 = usize;
type Input2 = Cave;
type Output2 = usize;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Square {
    Empty,
    FallSand,
    HardSand,
    Rock,
}

pub struct Cave {
    grid: Vec<Vec<Square>>,
    current_grain: (usize, usize),
    x_max: usize,
    y_max: usize,
}

impl ToString for Cave {
    fn to_string(&self) -> String {
        let mut vecs = vec![String::new(); self.y_max];
        for x in 430..self.x_max {
            for y in 0..self.y_max {
                let c = match self.grid[x][y] {
                    Square::FallSand => '+',
                    Square::HardSand => 'o',
                    Square::Rock => '#',
                    Square::Empty => '.',
                };
                vecs[y].push(c)
            }
        }
        vecs.join("\n")
    }
}

impl Cave {
    pub fn from_string(s: &str, add_floor: bool) -> Result<Cave, ()> {
        let rocks: Vec<Vec<(usize, usize)>> = s.lines().map(|s_| s_.split(" -> ").map(|xy| {
            let (x, y) = xy.split_once(",").unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        }).collect()).collect();
        let mut x_max = rocks.iter().map(|rock_line| rock_line.iter().map(|&r| r.0).max().unwrap()).max().unwrap() + 2;
        let mut y_max = rocks.iter().map(|rock_line| rock_line.iter().map(|&r| r.1).max().unwrap()).max().unwrap() + 2;
        let mut grid = if add_floor {
            y_max += 1;
            x_max += y_max;
            let mut g = vec![vec![Square::Empty; y_max]; x_max];
            for x in 0..(x_max) {
                g[x][y_max-1] = Square::Rock;
            }
            g
        } else {
            vec![vec![Square::Empty; y_max]; x_max]
        };
        for rock in rocks {
            let start_end = rock[0..rock.len()-1].iter().zip(rock[1..].iter());
            for (start, end) in start_end {
                let start_ = min(start, end);
                let end_ = max(start, end);
                // println!("{:?} {:?}", start_, end_);
                for x in start_.0..=end_.0 {
                    for y in start_.1..=end_.1 {
                        grid[x][y] = Square::Rock;
                    }
                }
            }
        }
        let current_grain = (500, 0);
        grid[current_grain.0][current_grain.1] = Square::FallSand;
        Ok(Cave {grid, x_max, y_max, current_grain})
    }

pub fn drop_grain(&mut self) -> (Option<(usize, usize)>, usize) {
    let mut iterations = 0;
    while let Some((xy, reason)) = self.next_spot() {
        // println!("Iterations: {}. (x, y): {:?} \n {}", iterations, xy, self.to_string());
            iterations += 1;
            self.update_current(xy, reason);
            if let Square::HardSand = reason {
                return (Some(xy), iterations)
            };
        }
        (None, iterations)
    }

    fn update_current(&mut self, new_pos: (usize, usize), square: Square) {
        let old = self.current_grain;
        self.grid[old.0][old.1] = Square::Empty;
        self.grid[new_pos.0][new_pos.1] = square;
        self.current_grain = new_pos;
    }

    fn reset(&mut self) -> bool {
        self.current_grain = (500,0);
        if self.grid[self.current_grain.0][self.current_grain.1] == Square::HardSand {
            false
        } else {
            self.grid[self.current_grain.0][self.current_grain.1] = Square::FallSand;
            true
        }
    }

    fn next_spot(&self) -> Option<((usize, usize), Square)> {
        if self.current_grain.1 > self.y_max - 2 {
            return None;
        }
        [(0,1), (-1,1), (1,1)].into_iter().filter_map(|(x_diff, y_diff)| {
            let x = add(self.current_grain.0, x_diff).unwrap();
            let y = add(self.current_grain.1, y_diff).unwrap();
            match self.grid[x][y] {
                Square::Empty => Some(Some(((x,y), Square::FallSand))),
                Square::HardSand | Square::Rock => None,
                Square::FallSand => Some(Some(((x,y), Square::FallSand))),
            }
        }).into_iter().nth(0).unwrap_or(Some(((self.current_grain.0, self.current_grain.1), Square::HardSand)))
    }
}

impl FromStr for Cave {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Cave::from_string(s, false)
    }
}

pub fn prepare(input: String) -> Input1 {
    input.parse().unwrap()
}

pub fn prepare_2(input: String) -> Input2 {
    Cave::from_string(&*input, true).unwrap()
}

pub fn part_1(mut input: Input1) -> Output1 {
    let mut finish = None;
    let mut particles = 0;
    let mut iterations = 0;
    println!("{}", input.to_string());
    while let None = finish {
        // println!("Particles: {}. Iterations: {}", particles, iterations);
        particles += 1;
        match input.drop_grain() {
            (None, n) => {
                iterations += n;
                finish = Some(particles)
            },
            (Some((x, y)), n) => {
                iterations += n;
                if !input.reset() {finish = Some(particles)};
            }
        }
    }
    println!("{}", input.to_string());
    finish.unwrap() - 1
}

pub fn part_2(mut input: Input2) -> Output2 {
    let mut finish = None;
    let mut particles = 0;
    let mut iterations = 0;
    println!("{}", input.to_string());
    while let None = finish {
        // println!("Particles: {}. Iterations: {}", particles, iterations);
        particles += 1;
        match input.drop_grain() {
            (None, n) => {
                iterations += n;
                finish = Some(particles)
            },
            (Some((x, y)), n) => {
                iterations += n;
                if !input.reset() {finish = Some(particles)};
            }
        }
    }
    println!("{}", input.to_string());
    finish.unwrap()
}

pub fn make_sol() -> StructSolution<Input1, Output1, Input2, Output2> {
    let struct_solution = StructSolution {
        prepare_part_1: prepare,
        calc_part_1: part_1,
        prepare_part_2: prepare_2,
        calc_part_2: part_2,
        example_part_1: Example::Value(24),
        example_part_2: Example::Value(93),
        day_data: DayData::new(14, false),
    };
    struct_solution
}
