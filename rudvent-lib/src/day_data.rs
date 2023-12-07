use chrono::{DateTime, Utc};
use color_eyre::eyre::Result;
use std::fmt::Debug;

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
