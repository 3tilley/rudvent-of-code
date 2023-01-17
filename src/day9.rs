#![feature(asm)]
use crate::solution::{Example, StructSolution};
use crate::stack_analysis::{print_pointer, stack_ptr, tick};
use crate::{stack_ptr, tick, DayData};
use get_size::GetSize;
use humansize::{make_format, DECIMAL};
use std::cmp::max;
use std::collections::HashSet;
use std::str::FromStr;

type Input1 = Vec<Direction>;
type Output1 = usize;
type Input2 = Vec<Direction>;
type Output2 = usize;

#[derive(Copy, Clone, Debug, GetSize)]
pub enum Direction {
    Up { d: u8 },
    Down { d: u8 },
    Left { d: u8 },
    Right { d: u8 },
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, GetSize)]
pub struct Position {
    x: i16,
    y: i16,
}

impl Position {
    pub fn zero() -> Position {
        Position { x: 0, y: 0 }
    }

    pub fn update(&mut self, dir: Direction) -> Option<Direction> {
        match dir {
            Direction::Up { d } => {
                self.y += 1;
                if d != 1 {
                    Some(Direction::Up { d: d - 1 })
                } else {
                    None
                }
            }
            Direction::Down { d } => {
                self.y -= 1;
                if d != 1 {
                    Some(Direction::Down { d: d - 1 })
                } else {
                    None
                }
            }
            Direction::Left { d } => {
                self.x -= 1;
                if d != 1 {
                    Some(Direction::Left { d: d - 1 })
                } else {
                    None
                }
            }
            Direction::Right { d } => {
                self.x += 1;
                if d != 1 {
                    Some(Direction::Right { d: d - 1 })
                } else {
                    None
                }
            }
        }
    }
    pub fn tail_update(&mut self, head_pos: &Position) {
        let x_diff = head_pos.x - self.x;
        let y_diff = head_pos.y - self.y;

        if (x_diff.abs() >= 2) | (y_diff.abs() >= 2) {
            self.x += (x_diff / max(x_diff.abs(), 1));
            self.y += (y_diff / max(y_diff.abs(), 1));
        }
    }
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, second) = s.split_once(" ").unwrap();
        let d = second.parse().unwrap();
        match first {
            "R" => Ok(Direction::Right { d }),
            "L" => Ok(Direction::Left { d }),
            "U" => Ok(Direction::Up { d }),
            "D" => Ok(Direction::Down { d }),
            _ => Err(()),
        }
    }
}

// #[derive(Debug)]
// pub struct

pub fn prepare(input: String) -> Input1 {
    input.lines().map(|v| v.parse().unwrap()).collect()
}

fn show(head: &Position, tail: &Position, iteration: u16) {
    println!("Iteration: {}", iteration);
    for y in (-2..=10).rev() {
        for x in -2..=10 {
            if (head.x == x) & (head.y == y) {
                print!("H")
            } else if (tail.x == x) & (tail.y == y) {
                print!("T")
            } else if (x == 0) & (y == 0) {
                print!("s");
            } else {
                print!(".")
            }
        }
        print!("\n")
    }
    println!()
}

// This was a well meaning recursive solution that ultimate didn't work. Somewhere a large amount
// of memory (2kb) was allocated on the stack every call and blew the stack. There should have only
// been about 200b
fn inner(
    mut input: Input1,
    mut history: HashSet<Position>,
    mut head: Position,
    mut tail: Position,
    next_move: Direction,
    iteration: u16,
    original_rsp: usize,
) -> HashSet<Position> {
    // show(&head, &tail, iteration);
    let known_allocations = (Position::get_stack_size() * 2)
        + (Vec::<Direction>::get_stack_size())
        + (HashSet::<Position>::get_stack_size() + 20);
    println!("Known allocations: {}", known_allocations);
    tick();
    print_pointer(original_rsp, "Start of function");
    let mut tracer = 0usize;
    let new_move = head.update(next_move);
    tracer = print_pointer(tracer, "After head move");
    tail.tail_update(&head);
    tracer = print_pointer(tracer, "After tail move");
    history.insert(tail);
    tracer = print_pointer(tracer, "After map insert");
    match new_move {
        None => {
            tracer = print_pointer(tracer, "After popping");
            match input.pop() {
                Some(x) => inner(input, history, head, tail, x, iteration + 1, original_rsp),
                None => history,
            }
        }
        Some(x) => {
            tracer = print_pointer(tracer, "After regular finish");
            inner(input, history, head, tail, x, iteration + 2, original_rsp)
        }
    }
}
fn loop_inner_part_1(
    mut input: Input1,
    mut head: Position,
    mut tail: Position,
) -> HashSet<Position> {
    // show(&head, &tail, iteration);
    let mut history = HashSet::new();
    let mut current_move = input.pop();
    loop {
        match current_move {
            None => {
                break;
            }
            Some(m) => {
                let new_move = head.update(m);
                tail.tail_update(&head);
                history.insert(tail);
                match new_move {
                    Some(_) => current_move = new_move,
                    None => current_move = input.pop(),
                }
            }
        }
    }
    history
}

pub fn part_1(mut input: Input1) -> Output1 {
    input.reverse();
    let new_hist = loop_inner_part_1(input, Position::zero(), Position::zero());
    new_hist.len()
}

fn loop_inner_part_2(
    mut input: Input1,
    mut head: Position,
    mut tail: Vec<Position>,
) -> HashSet<Position> {
    let mut history = HashSet::new();
    let mut current_move = input.pop();
    println!("{:?}", current_move);
    let mut iteration = 0;
    loop {
        // show(&head, &tail.last().unwrap(), iteration);
        match current_move {
            None => {
                break;
            }
            Some(m) => {
                let new_move = head.update(m);
                let mut last_val = head;
                for mut t in &mut tail {
                    t.tail_update(&last_val);
                    last_val = *t;
                }
                history.insert(last_val);
                match new_move {
                    Some(_) => current_move = new_move,
                    None => current_move = input.pop(),
                }
            }
        }
        iteration += 1;
    }
    history
}

pub fn part_2(mut input: Input2) -> Output2 {
    input.reverse();
    let hist = loop_inner_part_2(input, Position::zero(), vec![Position::zero(); 9]);
    hist.len()
}

pub fn make_sol() -> StructSolution<Input1, Output1, Input2, Output2> {
    let struct_solution = StructSolution {
        prepare_part_1: prepare,
        calc_part_1: part_1,
        prepare_part_2: prepare,
        calc_part_2: part_2,
        example_part_1: Example::Value(13),
        example_part_2: Example::Value(36),
        day_data: DayData::new(9, false),
    };
    struct_solution
}
