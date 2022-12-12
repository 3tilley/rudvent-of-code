use std::str::FromStr;
use crate::DayData;
use crate::solution::{Example, StructSolution};

#[derive(Debug, Copy, Clone)]
pub enum GameResult {
    Win,
    Loss,
    Draw,
}

impl GameResult {
    pub fn points(&self) -> u8 {
            match self {
                GameResult::Win => 6,
                GameResult::Loss => 0,
                GameResult::Draw => 3,
            }
        }
    }

impl FromStr for GameResult {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(GameResult::Loss),
            "Y" => Ok(GameResult::Draw),
            "Z" => Ok(GameResult::Win),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Play {
    Rock,
    Paper,
    Scissors,
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

    pub fn achieve_result(&self, result: GameResult) -> Play {
        match result {
            GameResult::Win => match self {
                Play::Rock => Play::Paper,
                Play::Paper => Play::Scissors,
                Play::Scissors => Play::Rock,
            },
            GameResult::Loss => match self {
                Play::Rock => Play::Scissors,
                Play::Paper => Play::Rock,
                Play::Scissors => Play::Paper,
            },
            GameResult::Draw => *self,
        }
    }

    pub fn points(&self) -> u8 {
        match self {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
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

#[derive(Debug, Copy, Clone)]
pub struct GamePlanPart2 {
    play: Play,
    desired_result: GameResult,
}

impl GamePlan {
    pub fn score(&self) -> u8 {
        let game_score = self.response.result(&self.play).points();
        game_score + self.response.points()
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

pub fn prepare_2(input: String) -> Vec<GamePlanPart2> {
    let mut game_plans = Vec::new();
    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let plan = GamePlanPart2 {
            play: parts.next().unwrap().parse::<Play>().unwrap(),
            desired_result: parts.next().unwrap().parse::<GameResult>().unwrap(),
        };
        game_plans.push(plan);
    }
    game_plans
}

pub fn part_1(input: Vec<GamePlan>) -> u64 {
    input.iter().map(|plan| plan.score() as u64).sum()
}

pub fn part_2(input: Vec<GamePlanPart2>) -> u64 {
    input.iter().map(|plan| {
        let play = plan.play.achieve_result(plan.desired_result);
        (play.points() as u64) + (plan.desired_result.points() as u64)
    }).sum()
}

pub fn make_sol() -> StructSolution<Vec<GamePlan>, u64, Vec<GamePlanPart2>, u64> {
    let struct_solution = StructSolution {
        prepare_part_1: prepare,
        calc_part_1: part_1,
        prepare_part_2: prepare_2,
        calc_part_2: part_2,
        example_part_1: Example::Value(15),
        example_part_2: Example::Value(12),
        day_data: DayData::new(2, false),
    };
    struct_solution
}
