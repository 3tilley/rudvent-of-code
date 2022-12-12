use std::fmt::{Debug, Display};

pub trait Output: Clone + Debug + PartialEq + Display {}

impl<T> Output for T where T: Clone + Debug + PartialEq + Display {}
