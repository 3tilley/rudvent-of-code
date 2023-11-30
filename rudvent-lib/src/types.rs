use std::fmt::{Debug, Display};
use std::str::FromStr;

pub trait Output: Clone + Debug + PartialEq + Display {}

impl<T> Output for T where T: Clone + Debug + PartialEq + Display {}
