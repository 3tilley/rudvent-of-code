use crate::advent_interactions::DayData;
use crate::types::Output;
use chrono::{DateTime, Utc};
use chrono_humanize::{Accuracy, HumanTime, Tense};
use color_eyre::eyre::{eyre, Result};
use std::fmt::Debug;

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


pub trait DayArguments {
    // fn from_vec(extra_args: Vec<(String, String)>) -> T {()}
    fn set_is_example(&mut self, is_example: bool) {
        ()
    }
}

pub struct DummyArgs {}

impl DayArguments for DummyArgs {
    fn set_is_example(&mut self, is_example: bool) {
        ()
    }
}
