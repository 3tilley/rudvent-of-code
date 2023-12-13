pub mod execution;

use crate::advent_interactions::{ask_bool_input, DayData};
use crate::cli::App;
use crate::printer::Printer;
use crate::types::Output;
use chrono::Utc;
use color_eyre::eyre::eyre;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use color_eyre::owo_colors::OwoColorize;
use execution::{DayArguments, Example, Execution, RunParams};
use crate::solution::execution::{ExecutionResult, Monitor, RuntimeMonitor, ThreadedExecution, ThreadExecutionResult};

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
        Z: Monitor + 'static,
    > SolutionBuilder for StructSolutionBuilder<T, U, V, W, X, Z>
{
    fn build(&self, app: &App, day: u8, cli_params: Vec<String>) -> Box<dyn Solution> {
        let day_args = RunParams {
            is_example: false,
            user_params: X::parse_from(std::iter::once("dummy_name".to_string()).chain(cli_params)),
        };
        println!("{:?}", day_args);
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
    pub calc_part_1: fn(T, &RunParams<X>, Arc<Mutex<RuntimeMonitor<Z>>>) -> U,
    pub prepare_part_2: fn(String) -> V,
    pub calc_part_2: fn(V, &RunParams<X>, Arc<Mutex<RuntimeMonitor<Z>>>) -> W,
    pub example_part_1: Example<U>,
    pub example_part_2: Example<W>,
}

impl<T, U: Output, V, W: Output, X: DayArguments, Z> StructSolutionBuilder<T, U, V, W, X, Z> {
    pub fn new(
        prepare_part_1: fn(String) -> T,
        calc_part_1: fn(T, &RunParams<X>, Arc<Mutex<RuntimeMonitor<Z>>>) -> U,
        prepare_part_2: fn(String) -> V,
        calc_part_2: fn(V, &RunParams<X>, Arc<Mutex<RuntimeMonitor<Z>>>) -> W,
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
    pub calc_part_1: fn(T, &RunParams<X>, Arc<Mutex<RuntimeMonitor<Z>>>) -> U,
    pub prepare_part_2: fn(String) -> V,
    pub calc_part_2: fn(V, &RunParams<X>, Arc<Mutex<RuntimeMonitor<Z>>>) -> W,
    pub example_part_1: Example<U>,
    pub example_part_2: Example<W>,
    pub day_args: RunParams<X>,
    pub day_data: DayData,
}

// U is is the result of part 1, W is the result of part 2. X is to differentiate between the
// example and the main run if required
impl<T:  'static, U: Output + 'static, V: 'static, W: Output + 'static, X: DayArguments + 'static, Z: Monitor + 'static> StructSolution<T, U, V, W, X, Z> {
    // pub fn check_example_1(&mut self) -> ThreadExecutionResult<U, Z> {
    //     self.day_args.set_is_example(true);
    //     let mut stack_info = Arc::new(Mutex::new(RuntimeMonitor::<Z>::new()));
    //     let prep_start = Instant::now();
    //     let input = (self.prepare_part_1)(self.day_data.example_1());
    //     let prep_time = prep_start.elapsed();
    //     let run_start = Instant::now();
    //     let ans = (self.calc_part_1)(input, &self.day_args, stack_info.clone());
    //     let calculation_time = run_start.elapsed();
    //     let example_val: U = self.example_part_1.value();
    //     let res = if ans == example_val {
    //         Ok(ans)
    //     } else if ans == U::default() {
    //         Err(eyre!("Example didn't match, but example == {}. Did you update EXAMPLE_1_ANS?", ans))
    //     } else {
    //         Err(eyre!(
    //             "Example 1 failed. Expected: {:?}, got: {:?}",
    //             example_val,
    //             ans
    //         ))
    //     };
    //     let ex = ThreadExecutionResult::new(res, stack_info.into(), chrono::Duration::from_std(calculation_time).unwrap(), chrono::Duration::from_std(prep_time + calculation_time).unwrap());
    //     ex
    // }

    pub fn check_example_1(&mut self) -> Box<dyn ExecutionResult> {
        let input = self.day_data.example_1();
        let mut execution = ThreadedExecution::new(input, self.prepare_part_1, self.calc_part_1, Some(self.example_part_1.value()), self.day_args.clone());
        execution.run().join().unwrap()
    }

    pub fn check_example_2(&mut self) -> Box<dyn ExecutionResult> {
        let input = self.day_data.example_2();
        let mut execution = ThreadedExecution::new(input, self.prepare_part_2, self.calc_part_2, Some(self.example_part_2.value()), self.day_args.clone());
        execution.run().join().unwrap()

    }
    pub fn run_part_1(&mut self) -> ThreadedExecution<T, U, X, Z> {
        self.day_args.set_is_example(false);
        let input = self.day_data.input_1();

        let mut execution = ThreadedExecution::new(input, self.prepare_part_1, self.calc_part_1, None, self.day_args.clone());

        execution
    }
    pub fn run_part_2(&mut self) -> ThreadedExecution<V, W, X, Z> {
        self.day_args.set_is_example(false);
        let input = self.day_data.input_2();

        let mut execution = ThreadedExecution::new(input, self.prepare_part_2, self.calc_part_2, None, self.day_args.clone());

        execution
    }
    // pub fn run_part_2(&mut self) -> Execution<W, Z> {
    //     self.day_args.set_is_example(false);
    //     let prep_start = Utc::now();
    //     let mut stack_info = RuntimeMonitor::new();
    //     let input = (self.prepare_part_2)(self.day_data.input_2());
    //     let run_start = Utc::now();
    //     let ans = (self.calc_part_2)(input, &self.day_args, &mut stack_info);
    //     let run_end = Utc::now();
    //     let ex = Execution::new(Ok(ans), prep_start, run_start, run_end, stack_info);
    //     ex
    // }
}

pub trait Solution {
    fn run(&mut self, part_1: bool) -> Box<dyn Execution>;
    // fn check_part_1(&mut self) -> Execution<String>;
    // fn check_part_2(&mut self) -> Execution<String>;
    fn check_example_and_continue(&mut self, printer: &Printer, part_1: bool) -> bool;

    fn day_data(&self) -> &DayData;

}

impl<T: 'static, U: Output + 'static, V: 'static, W: Output + 'static, X: DayArguments + 'static, Z: Monitor + 'static> Solution for StructSolution<T, U, V, W, X, Z> {
    fn run(&mut self, part_1: bool) -> Box<dyn Execution> {
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
            let ans = ex.result();
            printer.success(&format!("Example matches: {}", ans.unwrap()));
        } else {
            // TODO: Revert this
            let ex = self.check_example_2();
            ex.show_info(printer);
            let ans = ex.result();
            printer.success(&format!("Example matches: {}", ans.unwrap()));
        };
        ask_bool_input("Run the full input set?", true)
    }

    fn day_data(&self) -> &DayData {
        &self.day_data
    }
}
