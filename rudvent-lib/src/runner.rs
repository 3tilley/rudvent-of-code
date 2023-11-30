use chrono::{DateTime, Utc};
use chrono_humanize::{Accuracy, HumanTime, Tense};
use color_eyre::eyre::{eyre, Result};
use std::fmt::Debug;
use crate::advent_interactions::DayData;
use crate::types::Output;

/// This currently does nothing, but is here to allow for future expansion
pub struct Monitor {}

impl Monitor {
    pub fn new() -> Monitor {
        Monitor {}
    }
}

#[derive(Debug, Clone)]
pub enum Example<T> {
    Value(T),
    Regex(String),
}

#[derive(Debug, Clone)]
pub enum PostError {
    TooLow,
    TooHigh,
}

// #[derive(Debug, Clone, Display)]
// pub enum PostResult {
//     Success,
// }

impl<T: Clone> Example<T> {
    pub fn value(&self) -> T {
        match self {
            Example::Value(v) => v.clone(),
            Example::Regex(s) => unimplemented!(),
        }
    }
}

pub struct Execution<T> {
    pub result: Result<T>,
    pub preparation_start: DateTime<Utc>,
    pub run_start: DateTime<Utc>,
    pub run_end: DateTime<Utc>,
    pub stack_info: Monitor,
}

impl<T> Execution<T> {
    pub fn new(
        result: Result<T>,
        preparation_start: DateTime<Utc>,
        run_start: DateTime<Utc>,
        run_end: DateTime<Utc>,
        stack_info: Monitor,
    ) -> Execution<T> {
        Execution {
            result,
            preparation_start,
            run_start,
            run_end,
            stack_info,
        }
    }

    pub fn show_info(&self) {
        let calc_duration = self.run_end - self.run_start;
        let total_duration = self.run_end - self.preparation_start;
        let calc_frac = (calc_duration.num_nanoseconds().unwrap() as f32)
            / (total_duration.num_nanoseconds().unwrap() as f32);
        let hc = HumanTime::from(calc_duration);
        let tc = HumanTime::from(total_duration);
        println!(
            "Completed part in {}.  Calculation time: {} ({:.1}%)",
            tc.to_text_en(Accuracy::Precise, Tense::Present),
            hc.to_text_en(Accuracy::Precise, Tense::Present),
            calc_frac * 100.0
        );
    }
}

pub trait TraitSolution<T, U, V, W> {
    fn a(example: bool) -> T {
        todo!()
    }
    fn b(example: bool) -> U {
        todo!()
    }
    fn prepare_a(example: bool) -> V {
        todo!()
    }
    fn prepare_b(example: bool) -> W {
        todo!()
    }
    // fn inner_a(prep: U) -> V {
    //     todo!()
    // }
    // fn inner_b(prep: U) -> V {
    //     todo!()
    // }
    // fn output_a(answer: V) -> T {
    //     todo!()
    // }
    // fn output_b(answer: V) -> T {
    //     todo!()
    // }
}

pub trait DayArguments {
    // fn from_vec(extra_args: Vec<(String, String)>) -> T {()}
    fn set_is_example(&mut self, is_example: bool) {()}
}

pub struct DummyArgs {

}

impl DayArguments for DummyArgs {
    fn set_is_example(&mut self, is_example: bool) {
        ()
    }
}

