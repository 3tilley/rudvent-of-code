use chrono::{DateTime, Duration, Utc};
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::Instant;
use chrono_humanize::{Accuracy, HumanTime, Tense};
use clap::Parser;
use color_eyre::eyre::eyre;
use crate::printer::Printer;
use crate::types::Output;
use color_eyre::Result;
use crate::utils::{unitless_formatter, unitless_formatter_i};

pub trait Monitor: Default + Clone + Debug + Send + Sync {

}

#[derive(Default, Clone, Debug)]
pub struct RuntimeMonitor<T> {
    pub total_progress: usize,
    pub current_progress: usize,
    pub user_monitor: T,
}

impl<T: Monitor> RuntimeMonitor<T> {
    pub fn new() -> RuntimeMonitor<T> {
        RuntimeMonitor {
            total_progress: 0,
            current_progress: 0,
            user_monitor: T::default(),
        }
    }

    pub fn cycles(&self, elapsed: chrono::Duration) -> String {
        if self.current_progress != 0 {
            format!("\n{} iterations. {} cycles per second", unitless_formatter(self.current_progress as u64), unitless_formatter_i((self.current_progress as f32) * 1_000_000.0  / (elapsed.num_microseconds().unwrap() as f32)))
        } else { "".to_string() }
    }

    pub fn show_progress(printer: &Printer) {

    }

    pub fn new_arc() -> Arc<Mutex<RuntimeMonitor<T>>> {
        Arc::new(Mutex::new(RuntimeMonitor::new()))
    }
}

#[derive(Default, Clone, Debug)]
pub struct EmptyUserMonitor {}
impl<T> Monitor for T where T: Clone + Debug + Default + Send + Sync {}

#[derive(Debug, Clone)]
pub enum Example<T> {
    Value(T),
    Regex(String),
}

impl<T: Clone> Example<T> {
    pub fn value(&self) -> T {
        match self {
            Example::Value(v) => v.clone(),
            Example::Regex(s) => unimplemented!(),
        }
    }
}

pub struct ThreadExecutionResult<T: Output, V> {
    pub result: Result<T>,
    pub runtime_monitor: Arc<Mutex<RuntimeMonitor<V>>>,
    pub calculation_duration: Duration,
    pub total_duration: Duration,
}

impl<T: Output, V> ThreadExecutionResult<T, V> {
    pub fn new(result: Result<T>, runtime_monitor: Arc<Mutex<RuntimeMonitor<V>>>, calculation_duration: Duration, total_duration: Duration) -> Self {
        ThreadExecutionResult {
            result,
            runtime_monitor,
            calculation_duration,
            total_duration,
        }
    }
}

pub trait ExecutionResult: Send {
    fn show_info(&self, printer: &Printer);
    fn result(&self) -> Result<String>;

}

pub trait Execution {
    fn show_progress(&self) -> String;
    fn get_current_total_iterations(&self) -> (usize, usize);
    fn run(&mut self) -> JoinHandle<Box<dyn ExecutionResult>>;
}

impl<T: Output, V: Monitor> ExecutionResult for ThreadExecutionResult<T, V> {
    fn show_info(&self, printer: &Printer) {
        let calc_frac = (self.calculation_duration.num_nanoseconds().unwrap() as f32)
            / (self.total_duration.num_nanoseconds().unwrap() as f32);
        let hc = HumanTime::from(self.calculation_duration);
        let tc = HumanTime::from(self.total_duration);
        let cycles = self.runtime_monitor.lock().unwrap().cycles(self.calculation_duration);
        printer.print_or_info(&*format!(
            "Completed part in {}.  Calculation time: {} ({:.1}%)\n{}",
            tc.to_text_en(Accuracy::Precise, Tense::Present),
            hc.to_text_en(Accuracy::Precise, Tense::Present),
            calc_frac * 100.0,
            cycles
        ));
    }

    fn result(&self) -> Result<String> {
        match &self.result {
            Ok(r) => Ok(format!("{}", r.clone())),
            Err(e) => Err(eyre!(e.to_string()))
        }
    }

}

pub struct ThreadedExecution<T, U: Output, X, Z> {
    // pub result: Result<T>,
    // pub preparation_start: DateTime<Utc>,
    is_complete: bool,
    run_start: Option<Instant>,
    // pub run_start: DateTime<Utc>,
    // pub run_end: DateTime<Utc>,
    run_params: RunParams<X>,
    runtime_monitor: Arc<Mutex<RuntimeMonitor<Z>>>,
    input: String,
    prep_function: fn(String) -> T,
    run_function: fn(T, &RunParams<X>, Arc<Mutex<RuntimeMonitor<Z>>>) -> U,
    example_check: Option<U>,
}

impl<T: 'static, U: Output + 'static, X: DayArguments + 'static, Z: Monitor + 'static> Execution for ThreadedExecution<T, U, X, Z> {
    fn show_progress(&self) -> String {
        match self.run_start {
            None => format!("Execution not started"),
            Some(start_time) => {

                let elapsed = chrono::Duration::from_std(start_time.elapsed()).unwrap();
                let cycles = self.runtime_monitor.lock().unwrap().cycles(elapsed);
                format!("Elapsed: {}{}", HumanTime::from(elapsed), cycles)
            }
        }

    }

    fn get_current_total_iterations(&self) -> (usize, usize) {
        let mon = self.runtime_monitor.lock().unwrap();
        (mon.current_progress, mon.total_progress)
    }

    fn run(&mut self) -> JoinHandle<Box<dyn ExecutionResult>> {
        ThreadedExecution::run(self)
    }
}

impl<T: 'static, U: Output + 'static, X: DayArguments + 'static, Z: Monitor + 'static> ThreadedExecution<T, U, X, Z> {
    pub fn new(input: String, prep_function: fn(String) -> T, run_function: fn(T, &RunParams<X>, Arc<Mutex<RuntimeMonitor<Z>>>) -> U, example_check: Option<U>, run_params: RunParams<X>) -> Self {
        Self {
            is_complete: false,
            run_start: None,
            run_params,
            runtime_monitor: Arc::new(Mutex::new(RuntimeMonitor::new())),
            input,
            run_function,
            prep_function,
            example_check,
        }
    }

    pub fn run(&mut self) -> JoinHandle<Box<dyn ExecutionResult>> {
        let input = self.input.clone();
        let prep_func = self.prep_function.clone();
        let run_func = self.run_function.clone();
        let monitor = self.runtime_monitor.clone();
        let example_check = self.example_check.clone();
        let mut run_params = self.run_params.clone();
        self.run_start = Some(Instant::now());
        thread::spawn(move || {
            let prep_start = Instant::now();
            let prep = (prep_func)(input);
            let prep_time = chrono::Duration::from_std(prep_start.elapsed()).unwrap();
            run_params.set_is_example(example_check.is_some());
            let run_start = Instant::now();
            let result = (run_func)(prep, &run_params, monitor.clone());
            let calculation_duration = Duration::from_std(run_start.elapsed()).unwrap();
            let res = match example_check {
                None => Ok(result),
                Some(example) => {
                    if result == example {
                        Ok(result)
                    } else if result == U::default() {
                        Err(eyre!("Example didn't match, but example == {}. Did you update EXAMPLE_1_ANS?", example))
                    } else {
                        Err(eyre!(
                            "Example 1 failed. Expected: {:?}, got: {:?}",
                            example,
                            result
                        ))
                    }
                }
            };
            Box::new(ThreadExecutionResult{
                result: res,
                runtime_monitor: monitor,
                calculation_duration,
                total_duration: prep_time + calculation_duration,
            }) as Box<dyn ExecutionResult>
        })
    }
}

// pub trait DayArguments {
//     // fn from_vec(extra_args: Vec<(String, String)>) -> T {()}
//     fn set_is_example(&mut self, is_example: bool);
//
//     fn from_cli_params(cli_params: Vec<String>) -> Self;
// }
//
pub trait DayArguments: Clone + Debug + Default + Parser + Send {}

impl<T> DayArguments for T where T: Clone + Debug + Default + Parser + Send {}

#[derive(Default, Debug, Clone)]
pub struct RunParams<T> {
    pub is_example: bool,
    pub user_params: T,
}

impl<T> RunParams<T> {
    pub fn set_is_example(&mut self, is_example: bool) {
        self.is_example = is_example
    }

}

#[derive(Parser, Clone, Debug, Default)]
pub struct EmptyUserParams {

}
