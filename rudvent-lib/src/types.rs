use std::fmt::{Debug, Display};
use crate::solution::{Solution, SolutionBuilder};

pub trait Output: Clone + Debug + PartialEq + Display {}

impl<T> Output for T where T: Clone + Debug + PartialEq + Display {}


// pub type SolutionBuilders = &'static [Option<fn(u8, Vec<(String, String)>) -> Box<dyn Solution>>];
pub type SolutionBuilders = &'static [Option<fn() -> Box<dyn SolutionBuilder>>];
