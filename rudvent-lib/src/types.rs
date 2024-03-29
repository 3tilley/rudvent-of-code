use crate::solution::{Solution, SolutionBuilder};
use std::fmt::{Debug, Display};

pub trait Output: Clone + Debug + PartialEq + Display + Default + Send {}

impl<T> Output for T where T: Clone + Debug + PartialEq + Display + Default + Send {}

// pub type SolutionBuilders = &'static [Option<fn(u8, Vec<(String, String)>) -> Box<dyn Solution>>];
pub type SolutionBuilders = &'static [Option<fn() -> Box<dyn SolutionBuilder>>];
