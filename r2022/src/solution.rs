use std::fmt::{Debug};
use color_eyre::eyre::eyre;
use crate::DayData;

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

pub struct StructSolution<T, U, V, W> {
    pub prepare_part_1: fn(String) -> T,
    pub calc_part_1: fn(T) -> U,
    pub prepare_part_2: fn(String) -> V,
    pub calc_part_2: fn(V) -> W,
    pub example_part_1: Example<U>,
    pub example_part_2: Example<W>,
    pub day_data: DayData,
}

// U is is the result of part 1, W is the result of part 2
impl<T, U: PartialEq<U> + Debug + Clone, V, W: PartialEq<W> + Debug + Clone> StructSolution<T, U, V, W> {

    pub fn check_example_1(&self) -> crate::Result<U> {
        let input = (self.prepare_part_1)(self.day_data.example_1());
        let ans = (self.calc_part_1)(input);
        let example_val: U = self.example_part_1.value();
        if ans == example_val {
            Ok(ans)
        } else {
            Err(eyre!(
                "Example 1 failed. Expected: {:?}, got: {:?}",
                example_val,
                ans
            ))
        }

    }

    pub fn check_example_2(&self) -> crate::Result<W> {
        let input = (self.prepare_part_2)(self.day_data.example_2());
        let ans = (self.calc_part_2)(input);
        let example_val = self.example_part_2.value();
        if ans == example_val {
            Ok(ans)
        } else {
            Err(eyre!(
                "Example 2 failed. Expected: {:?}, got: {:?}",
                example_val,
                ans
            ))
        }
    }
    pub fn run_part_1(&self) -> U {
        let input = (self.prepare_part_1)(self.day_data.input_1());
        let ans = (self.calc_part_1)(input);
        ans
    }
    pub fn run_part_2(&self) -> W {
        let input = (self.prepare_part_2)(self.day_data.input_2());
        let ans = (self.calc_part_2)(input);
        ans
    }
}
