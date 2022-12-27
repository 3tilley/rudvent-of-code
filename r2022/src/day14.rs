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
    pub fn drop_grain(&mut self) -> (Option<(usize, usize)>, usize) {
        let mut iterations = 0;
        while let Some((xy, reason)) = self.next_spot() {
            println!("Iterations: {}. (x, y): {:?} \n {}", iterations, xy, self.to_string());
            iterations += 1;
            self.grid[xy.0][xy.1] = reason;
            self.update_current(xy);
            if let Square::FallSand = reason {
                return (Some(xy), iterations)
            };
        }
        (None, iterations)
    }

    fn update_current(&mut self, new_pos: (usize, usize)) {
        let old = self.current_grain;
        self.grid[old.0][old.1] = Square::Empty;
        self.current_grain = new_pos;
    }

    fn next_spot(&self) -> Option<((usize, usize), Square)> {
        [(0,1), (-1,1), (1,1), (0,0)].into_iter().filter_map(|(x_diff, y_diff)| {
            let x = add(self.current_grain.0, x_diff).unwrap();
            let y = add(self.current_grain.1, y_diff).unwrap();
            match self.grid[x][y] {
                Square::Empty => Some(Some(((x,y), Square::FallSand))),
                Square::HardSand | Square::Rock => None,
                Square::FallSand => Some(Some(((x,y), Square::FallSand))),
            }
        }).into_iter().nth(0).unwrap_or(None)
    }
}

impl FromStr for Cave {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rocks: Vec<Vec<(usize, usize)>> = s.lines().map(|s_| s_.split(" -> ").map(|xy| {
            let (x, y) = xy.split_once(",").unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        }).collect()).collect();
        let x_max = rocks.iter().map(|rock_line| rock_line.iter().map(|&r| r.0).max().unwrap()).max().unwrap() + 2;
        let y_max = rocks.iter().map(|rock_line| rock_line.iter().map(|&r| r.1).max().unwrap()).max().unwrap() + 2;
        let mut grid = vec![vec![Square::Empty; y_max]; x_max];
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
        grid[current_grain.0][current_grain.1];
        Ok(Cave {grid, x_max, y_max, current_grain})
    }
}

pub fn prepare(input: String) -> Input1 {
    input.parse().unwrap()
}

pub fn part_1(mut input: Input1) -> Output1 {
    let mut finish = None;
    let mut particles = 0;
    let mut iterations = 0;
    println!("{}", input.to_string());
    while let None = finish {
        println!("Particles: {}. Iterations: {}", particles, iterations);
        particles += 1;
        match input.drop_grain() {
            (None, n) => {
                iterations += n;
                finish = Some(particles)
            },
            (Some((x, y)), n) => {
                iterations += x;
                input.update_current((500, 0));
            }
        }
    }
    finish.unwrap() - 1
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
        example_part_1: Example::Value(14),
        example_part_2: Example::Value(0),
        day_data: DayData::new(14, false),
    };
    struct_solution
}
