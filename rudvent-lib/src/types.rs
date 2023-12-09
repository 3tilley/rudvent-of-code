use crate::solution::{Solution, SolutionBuilder};
use std::fmt::{Debug, Display};

pub trait Output: Clone + Debug + PartialEq + Display + Default {}

impl<T> Output for T where T: Clone + Debug + PartialEq + Display + Default {}

// pub type SolutionBuilders = &'static [Option<fn(u8, Vec<(String, String)>) -> Box<dyn Solution>>];
pub type SolutionBuilders = &'static [Option<fn() -> Box<dyn SolutionBuilder>>];
