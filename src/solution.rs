use crate::stack_analysis::StackInfo;
use crate::{DayData, Output};
use chrono::{DateTime, Utc};
use chrono_humanize::{Accuracy, HumanTime, Tense};
use color_eyre::eyre::{eyre, Result};
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

pub struct Execution<T> {
    pub result: Result<T>,
    pub preparation_start: DateTime<Utc>,
    pub run_start: DateTime<Utc>,
    pub run_end: DateTime<Utc>,
    pub stack_info: StackInfo,
}

impl<T> Execution<T> {
    pub fn new(
        result: Result<T>,
        preparation_start: DateTime<Utc>,
        run_start: DateTime<Utc>,
        run_end: DateTime<Utc>,
        stack_info: StackInfo,
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
        if self.stack_info.total_iterations != 0 {
            println!("{} iterations recorded", self.stack_info.total_iterations);
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

pub struct StructSolution<T, U, V, W, X> {
    pub prepare_part_1: fn(String) -> T,
    pub calc_part_1: fn(T, &X, &mut StackInfo) -> U,
    pub prepare_part_2: fn(String) -> V,
    pub calc_part_2: fn(V, &X, &mut StackInfo) -> W,
    pub example_part_1: Example<U>,
    pub example_part_2: Example<W>,
    pub day_args: X,
    pub day_data: DayData,
}

// U is is the result of part 1, W is the result of part 2. X is to differentiate between the
// example and the main run if required
impl<T, U: Output, V, W: Output, X: DayArguments> StructSolution<T, U, V, W, X> {
    pub fn check_example_1(&mut self) -> Execution<U> {
        self.day_args.set_is_example(true);
        let prep_start = Utc::now();
        let mut stack_info = StackInfo::new();
        let input = (self.prepare_part_1)(self.day_data.example_1());
        let run_start = Utc::now();
        let ans = (self.calc_part_1)(input, &self.day_args, &mut stack_info);
        let run_end = Utc::now();
        let example_val: U = self.example_part_1.value();
        let res = if ans == example_val {
            Ok(ans)
        } else {
            Err(eyre!(
                "Example 1 failed. Expected: {:?}, got: {:?}",
                example_val,
                ans
            ))
        };
        let ex = Execution::new(res, prep_start, run_start, run_end, stack_info);
        ex
    }

    pub fn check_example_2(&mut self) -> Execution<W> {
        self.day_args.set_is_example(true);
        let prep_start = Utc::now();
        let mut stack_info = StackInfo::new();
        let input = (self.prepare_part_2)(self.day_data.example_2());
        let run_start = Utc::now();
        let ans = (self.calc_part_2)(input, &self.day_args, &mut stack_info);
        let run_end = Utc::now();
        let example_val = self.example_part_2.value();
        let res = if ans == example_val {
            Ok(ans)
        } else {
            Err(eyre!(
                "Example 2 failed. Expected: {:?}, got: {:?}",
                example_val,
                ans
            ))
        };
        let ex = Execution::new(res, prep_start, run_start, run_end, stack_info);
        ex
    }
    pub fn run_part_1(&mut self) -> Execution<U> {
        self.day_args.set_is_example(false);
        let prep_start = Utc::now();
        let mut stack_info = StackInfo::new();
        let input = (self.prepare_part_1)(self.day_data.input_1());
        let run_start = Utc::now();
        let ans = (self.calc_part_1)(input, &self.day_args, &mut stack_info);
        let run_end = Utc::now();
        let ex = Execution::new(Ok(ans), prep_start, run_start, run_end, stack_info);
        ex
    }
    pub fn run_part_2(&mut self) -> Execution<W> {
        self.day_args.set_is_example(false);
        let prep_start = Utc::now();
        let mut stack_info = StackInfo::new();
        let input = (self.prepare_part_2)(self.day_data.input_2());
        let run_start = Utc::now();
        let ans = (self.calc_part_2)(input, &self.day_args, &mut stack_info);
        let run_end = Utc::now();
        let ex = Execution::new(Ok(ans), prep_start, run_start, run_end, stack_info);
        ex
    }
}
