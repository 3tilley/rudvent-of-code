pub mod execution;

use crate::advent_interactions::{ask_bool_input, DayData};
use crate::cli::App;
use crate::printer::Printer;
use crate::types::Output;
use chrono::Utc;
use color_eyre::eyre::eyre;
use std::fmt::Debug;
use execution::{DayArguments, Example, Execution, RunParams};
use crate::solution::execution::{ExecutionResult, RuntimeMonitor};

// #[derive(Debug, Clone, Display)]
// pub enum PostResult {
//     Success,
// }

// impl<T: std::default::Default> DayArguments for BasicRunParams<T> {
//     fn set_is_example(&mut self, is_example: bool) {
//         self.is_example = is_example
//     }
//
//     fn from_cli_params(cli_params: Vec<String>) -> BasicRunParams<T> {
//         BasicRunParams { is_example: false, user_params: T::default() }
//     }
// }

pub trait SolutionBuilder {
    fn build(&self, app: &App, day: u8, cli_params: Vec<String>) -> Box<dyn Solution>;
}

impl<
        T: 'static,
        U: Output + 'static,
        V: 'static,
        W: Output + 'static,
        X: DayArguments + 'static,
        Z: Default + 'static,
    > SolutionBuilder for StructSolutionBuilder<T, U, V, W, X, Z>
{
    fn build(&self, app: &App, day: u8, cli_params: Vec<String>) -> Box<dyn Solution> {
        let day_args = RunParams {
            is_example: false,
            user_params: X::default(),
        };
        let day_data = DayData::new(
            app.year,
            day,
            false,
            app.data_directory.clone(),
            app.auth_token.clone(),
        );
        Box::new(StructSolution {
            prepare_part_1: self.prepare_part_1,
            calc_part_1: self.calc_part_1,
            prepare_part_2: self.prepare_part_2,
            calc_part_2: self.calc_part_2,
            example_part_1: self.example_part_1.clone(),
            example_part_2: self.example_part_2.clone(),
            day_args,
            day_data,
        })
    }
}

pub struct StructSolutionBuilder<T, U, V, W, X, Z> {
    pub prepare_part_1: fn(String) -> T,
    pub calc_part_1: fn(T, &RunParams<X>, &mut RuntimeMonitor<Z>) -> U,
    pub prepare_part_2: fn(String) -> V,
    pub calc_part_2: fn(V, &RunParams<X>, &mut RuntimeMonitor<Z>) -> W,
    pub example_part_1: Example<U>,
    pub example_part_2: Example<W>,
}

impl<T, U: Output, V, W: Output, X: DayArguments, Z> StructSolutionBuilder<T, U, V, W, X, Z> {
    pub fn new(
        prepare_part_1: fn(String) -> T,
        calc_part_1: fn(T, &RunParams<X>, &mut RuntimeMonitor<Z>) -> U,
        prepare_part_2: fn(String) -> V,
        calc_part_2: fn(V, &RunParams<X>, &mut RuntimeMonitor<Z>) -> W,
        example_part_1: Example<U>,
        example_part_2: Example<W>,
    ) -> StructSolutionBuilder<T, U, V, W, X, Z> {
        StructSolutionBuilder {
            prepare_part_1,
            calc_part_1,
            prepare_part_2,
            calc_part_2,
            example_part_1,
            example_part_2,
        }
    }
}

pub struct StructSolution<T, U, V, W, X, Z> {
    pub prepare_part_1: fn(String) -> T,
    pub calc_part_1: fn(T, &RunParams<X>, &mut RuntimeMonitor<Z>) -> U,
    pub prepare_part_2: fn(String) -> V,
    pub calc_part_2: fn(V, &RunParams<X>, &mut RuntimeMonitor<Z>) -> W,
    pub example_part_1: Example<U>,
    pub example_part_2: Example<W>,
    pub day_args: RunParams<X>,
    pub day_data: DayData,
}

// U is is the result of part 1, W is the result of part 2. X is to differentiate between the
// example and the main run if required
impl<T, U: Output, V, W: Output, X: DayArguments, Z: Default> StructSolution<T, U, V, W, X, Z> {
    pub fn check_example_1(&mut self) -> Execution<U, Z> {
        self.day_args.set_is_example(true);
        let prep_start = Utc::now();
        let mut stack_info = RuntimeMonitor::<Z>::new();
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

    pub fn check_example_2(&mut self) -> Execution<W, Z> {
        self.day_args.set_is_example(true);
        let prep_start = Utc::now();
        let mut stack_info = RuntimeMonitor::<Z>::new();
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
    pub fn run_part_1(&mut self) -> Execution<U, Z> {
        self.day_args.set_is_example(false);
        let prep_start = Utc::now();
        let mut stack_info = RuntimeMonitor::<Z>::new();
        let input = (self.prepare_part_1)(self.day_data.input_1());
        let run_start = Utc::now();
        let ans = (self.calc_part_1)(input, &self.day_args, &mut stack_info);
        let run_end = Utc::now();
        let ex = Execution::new(Ok(ans), prep_start, run_start, run_end, stack_info);
        ex
    }
    pub fn run_part_2(&mut self) -> Execution<W, Z> {
        self.day_args.set_is_example(false);
        let prep_start = Utc::now();
        let mut stack_info = RuntimeMonitor::new();
        let input = (self.prepare_part_2)(self.day_data.input_2());
        let run_start = Utc::now();
        let ans = (self.calc_part_2)(input, &self.day_args, &mut stack_info);
        let run_end = Utc::now();
        let ex = Execution::new(Ok(ans), prep_start, run_start, run_end, stack_info);
        ex
    }
}

pub trait Solution {
    fn run(&mut self, part_1: bool) -> Box<dyn ExecutionResult>;
    // fn check_part_1(&mut self) -> Execution<String>;
    // fn check_part_2(&mut self) -> Execution<String>;
    fn check_example_and_continue(&mut self, printer: &Printer, part_1: bool) -> bool;

    fn day_data(&self) -> &DayData;

}

impl<T, U: Output + 'static, V, W: Output + 'static, X: DayArguments, Z: Default + 'static> Solution for StructSolution<T, U, V, W, X, Z> {
    // fn run_part_1(&mut self) -> Execution<String> {
    //     self.run_part_1().into_execution_string()
    // }
    //
    // fn run_part_2(&mut self) -> Execution<String> {
    //     self.run_part_2().into_execution_string()
    // }

    // fn check_part_1(&mut self) -> Execution<String> {
    //     self.check_part_1().into_execution_string()
    // }
    //
    // fn check_part_2(&mut self) -> Execution<String> {
    //     self.check_part_2().into_execution_string()
    // }
    fn run(&mut self, part_1: bool) -> Box<dyn ExecutionResult> {
        if part_1 {
            Box::new(self.run_part_1())
        } else {
            Box::new(self.run_part_2())
        }
    }

    fn check_example_and_continue(&mut self, printer: &Printer, part_1: bool) -> bool {
        let suffix = if part_1 { "1" } else { "2" };
        println!("Checking example {}", suffix);
        if part_1 {
            let ex = self.check_example_1();
            ex.show_info(printer);
            let ans = ex.result;
            printer.success(&format!("Example matches: {}", ans.unwrap()));
        } else {
            let ex = self.check_example_2();
            ex.show_info(printer);
            let ans = ex.result;
            printer.success(&format!("Example matches: {}", ans.unwrap()));
        };
        ask_bool_input("Run the full input set?", true)
    }

    fn day_data(&self) -> &DayData {
        &self.day_data
    }
}
