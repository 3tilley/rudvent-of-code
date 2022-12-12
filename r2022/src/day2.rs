use std::str::FromStr;
use crate::DayData;
use crate::utils::{Example, StructSolution};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Play {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Copy, Clone)]
pub enum GameResult {
    Win,
    Loss,
    Draw,
}

impl Play {
    pub fn result(&self, other: &Play) -> GameResult {
        if *self == *other {
            GameResult::Draw
        } else if *self == Play::Rock && *other == Play::Scissors {
            GameResult::Win
        } else if *self == Play::Paper && *other == Play::Rock {
            GameResult::Win
        } else if *self == Play::Scissors && *other == Play::Paper {
            GameResult::Win
        } else {
            GameResult::Loss
        }
    }
}

impl FromStr for Play {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Play::Rock),
            "B" | "Y" => Ok(Play::Paper),
            "C" | "Z" => Ok(Play::Scissors),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct GamePlan {
    play: Play,
    response: Play,
}

impl GamePlan {
    pub fn score(&self) -> u8 {
        let game_score = match self.response.result(&self.play) {
            GameResult::Win => 6,
            GameResult::Loss => 0,
            GameResult::Draw => 3,
        };
        match self.response {
            Play::Rock => game_score + 1,
            Play::Paper => game_score + 2,
            Play::Scissors => game_score + 3,
        }
    }
}

pub fn prepare(input: String) -> Vec<GamePlan> {
    let mut game_plans = Vec::new();
    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let plan = GamePlan {
            play: parts.next().unwrap().parse::<Play>().unwrap(),
            response: parts.next().unwrap().parse::<Play>().unwrap(),
        };
        game_plans.push(plan);
    }
    game_plans
}

pub fn part_1(input: Vec<GamePlan>) -> u64 {
    input.iter().map(|plan| plan.score() as u64).sum()
}

pub fn part_2(input: Vec<GamePlan>) -> u64 {
    todo!("Implement part 1");
}

pub fn make_sol() -> StructSolution<Vec<GamePlan>, u64, u64> {
    let struct_solution = StructSolution {
        prepare_part_1: prepare,
        calc_part_1: part_1,
        prepare_part_2: prepare,
        calc_part_2: part_2,
        example_part_1: Example::Value(15),
        example_part_2: Example::Value(12),
        day_data: DayData::new(2, false),
    };
    struct_solution
}
