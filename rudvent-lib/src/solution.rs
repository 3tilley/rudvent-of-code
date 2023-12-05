use crate::advent_interactions::{ask_bool_input, DayData};
use crate::cli::App;
use crate::day_data::Monitor;
use crate::printer::Printer;
use crate::types::Output;
use chrono::{DateTime, Utc};
use chrono_humanize::{Accuracy, HumanTime, Tense};
use color_eyre::eyre::{eyre, Result};
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub enum Example<T> {
    Value(T),
    Regex(String),
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

pub struct Execution<T: Output> {
    pub result: Result<T>,
    pub preparation_start: DateTime<Utc>,
    pub run_start: DateTime<Utc>,
    pub run_end: DateTime<Utc>,
    pub stack_info: Monitor,
}

impl<T: Output> Execution<T> {
    pub fn new(
        result: Result<T>,
        preparation_start: DateTime<Utc>,
        run_start: DateTime<Utc>,
        run_end: DateTime<Utc>,
        stack_info: Monitor,
    ) -> Execution<T> {
        Execution {
            result,
            preparation_start,
            run_start,
            run_end,
            stack_info,
        }
    }

    pub fn show_info(&self, printer: &Printer) {
        let calc_duration = self.run_end - self.run_start;
        let total_duration = self.run_end - self.preparation_start;
        let calc_frac = (calc_duration.num_nanoseconds().unwrap() as f32)
            / (total_duration.num_nanoseconds().unwrap() as f32);
        let hc = HumanTime::from(calc_duration);
        let tc = HumanTime::from(total_duration);
        printer.print_or_info(&*format!(
            "Completed part in {}.  Calculation time: {} ({:.1}%)",
            tc.to_text_en(Accuracy::Precise, Tense::Present),
            hc.to_text_en(Accuracy::Precise, Tense::Present),
            calc_frac * 100.0
        ));
    }

    pub fn into_execution_string(self) -> Execution<String> {
        let res = match self.result {
            Ok(r) => Ok(format!("{}", r)),
            Err(e) => Err(e),
        };
        Execution::new(
            res,
            self.preparation_start,
            self.run_start,
            self.run_end,
            self.stack_info,
        )
    }
}

// pub trait DayArguments {
//     // fn from_vec(extra_args: Vec<(String, String)>) -> T {()}
//     fn set_is_example(&mut self, is_example: bool);
//
//     fn from_cli_params(cli_params: Vec<String>) -> Self;
// }
//
pub trait DayArguments: Clone + Debug + Default {}
impl<T> DayArguments for T where T: Clone + Debug + Default {}

pub struct RunParams<T> {
    is_example: bool,
    user_params: T,
}

impl<T> RunParams<T> {
    pub fn set_is_example(&mut self, is_example: bool) {
        self.is_example = is_example
    }
}

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
    > SolutionBuilder for StructSolutionBuilder<T, U, V, W, X>
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

pub struct StructSolutionBuilder<T, U, V, W, X> {
    pub prepare_part_1: fn(String) -> T,
    pub calc_part_1: fn(T, &RunParams<X>, &mut Monitor) -> U,
    pub prepare_part_2: fn(String) -> V,
    pub calc_part_2: fn(V, &RunParams<X>, &mut Monitor) -> W,
    pub example_part_1: Example<U>,
    pub example_part_2: Example<W>,
}

impl<T, U: Output, V, W: Output, X: DayArguments> StructSolutionBuilder<T, U, V, W, X> {
    pub fn new(
        prepare_part_1: fn(String) -> T,
        calc_part_1: fn(T, &RunParams<X>, &mut Monitor) -> U,
        prepare_part_2: fn(String) -> V,
        calc_part_2: fn(V, &RunParams<X>, &mut Monitor) -> W,
        example_part_1: Example<U>,
        example_part_2: Example<W>,
    ) -> StructSolutionBuilder<T, U, V, W, X> {
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

pub struct StructSolution<T, U, V, W, X> {
    pub prepare_part_1: fn(String) -> T,
    pub calc_part_1: fn(T, &RunParams<X>, &mut Monitor) -> U,
    pub prepare_part_2: fn(String) -> V,
    pub calc_part_2: fn(V, &RunParams<X>, &mut Monitor) -> W,
    pub example_part_1: Example<U>,
    pub example_part_2: Example<W>,
    pub day_args: RunParams<X>,
    pub day_data: DayData,
}

// U is is the result of part 1, W is the result of part 2. X is to differentiate between the
// example and the main run if required
impl<T, U: Output, V, W: Output, X: DayArguments> StructSolution<T, U, V, W, X> {
    pub fn check_example_1(&mut self) -> Execution<U> {
        self.day_args.set_is_example(true);
        let prep_start = Utc::now();
        let mut stack_info = Monitor::new();
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
        let mut stack_info = Monitor::new();
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
        let mut stack_info = Monitor::new();
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
        let mut stack_info = Monitor::new();
        let input = (self.prepare_part_2)(self.day_data.input_2());
        let run_start = Utc::now();
        let ans = (self.calc_part_2)(input, &self.day_args, &mut stack_info);
        let run_end = Utc::now();
        let ex = Execution::new(Ok(ans), prep_start, run_start, run_end, stack_info);
        ex
    }
}

pub trait Solution {
    fn run(&mut self, part_1: bool) -> Execution<String>;
    // fn check_part_1(&mut self) -> Execution<String>;
    // fn check_part_2(&mut self) -> Execution<String>;
    fn check_example_and_continue(&mut self, printer: &Printer, part_1: bool) -> bool;

    fn day_data(&self) -> &DayData;
}

impl<T, U: Output, V, W: Output, X: DayArguments> Solution for StructSolution<T, U, V, W, X> {
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
    fn run(&mut self, part_1: bool) -> Execution<String> {
        if part_1 {
            self.run_part_1().into_execution_string()
        } else {
            self.run_part_2().into_execution_string()
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
