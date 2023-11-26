use crate::solution::{Example, StructSolution};
use crate::DayData;
use std::str::FromStr;

type Input1 = Vec<Instruction>;
type Output1 = isize;
type Input2 = Vec<Instruction>;
type Output2 = String;

static CYCLES: &[usize; 6] = &[20, 60, 100, 140, 180, 220];

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Instruction {
    Noop,
    Addx { change: isize },
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[..4] {
            "noop" => Ok(Instruction::Noop),
            "addx" => {
                let (_, change) = s.split_once(" ").unwrap();
                Ok(Instruction::Addx {
                    change: change.parse().unwrap(),
                })
            }
            _ => Err(()),
        }
    }
}

pub fn prepare(input: String) -> Input1 {
    input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<_>>()
}

fn inner(
    mut input: impl Iterator<Item = Instruction>,
    cycle: usize,
    x: isize,
    x_next: isize,
    mut x_hist: Vec<isize>,
) -> Vec<isize> {
    let next = input.next();
    match next {
        Some(Instruction::Noop) => {
            x_hist.push(x);
            inner(input, cycle + 1, x, x_next, x_hist)
        }
        Some(Instruction::Addx { change }) => {
            x_hist.push(x as isize);
            x_hist.push(x as isize);
            let x = x + change;
            inner(input, cycle + 1, x, x_next, x_hist)
        }
        None => x_hist,
    }
}

pub fn part_1(input: Input1) -> Output1 {
    let mut x_hist = Vec::new();
    let output = inner(input.into_iter(), 0, 1, 0, x_hist);
    println!("{:?}", output);
    let vals = CYCLES.map(|cycle| cycle as isize * output[cycle - 1]);
    println!("{:?}", vals);
    vals.iter().sum()
}

pub fn part_2(input: Input2) -> Output2 {
    let mut x_hist = Vec::new();
    let output = inner(input.into_iter(), 0, 1, 0, x_hist);
    let ticks = (0..240).map(|t| t % 40).zip(output);
    ticks
        .map(|(t, x)| {
            if (x - 1..=x + 1).contains(&t) {
                "#"
            } else {
                "."
            }
        })
        .collect::<Vec<_>>()
        .chunks(40)
        .map(|chunk| chunk.join(""))
        .for_each(|chunk| {
            println!("{:?}", chunk);
        });
    "hello".to_string()
}

pub fn make_sol() -> StructSolution<Input1, Output1, Input2, Output2> {
    let struct_solution = StructSolution {
        prepare_part_1: prepare,
        calc_part_1: part_1,
        prepare_part_2: prepare,
        calc_part_2: part_2,
        example_part_1: Example::Value(13140),
        // This is tricky, a really slick solution would be able to read what's printed
        example_part_2: Example::Value("blah".to_string()),
        day_data: DayData::new(10, false),
    };
    struct_solution
}
