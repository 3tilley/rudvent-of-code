use num::Integer;
use std::fmt::{Debug, Display};
use std::str::FromStr;

pub trait Output: Clone + Debug + PartialEq + Display {}

impl<T> Output for T where T: Clone + Debug + PartialEq + Display {}

pub trait AdNum: Integer + Debug + Display + FromStr + Clone + Ord {}
impl<T> AdNum for T where T: Integer + Debug + Display + FromStr + Clone + Ord {}
