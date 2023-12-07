use chrono::{DateTime, Utc};
use std::fmt::Debug;
use chrono_humanize::{Accuracy, HumanTime, Tense};
use color_eyre::eyre::eyre;
use crate::printer::Printer;
use crate::types::Output;
use color_eyre::Result;

trait Monitor {

}

pub struct RuntimeMonitor<T> {
    pub total_progress: usize,
    pub current_progress: usize,
    pub user_monitor: T,
}

impl<T: Default> RuntimeMonitor<T> {
    pub fn new() -> RuntimeMonitor<T> {
        RuntimeMonitor {
            total_progress: 0,
            current_progress: 0,
            user_monitor: T::default(),
        }
    }
}

#[derive(Default)]
pub struct EmptyUserMonitor {}

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

pub struct Execution<T: Output, V> {
    pub result: color_eyre::Result<T>,
    pub preparation_start: DateTime<Utc>,
    pub run_start: DateTime<Utc>,
    pub run_end: DateTime<Utc>,
    pub stack_info: RuntimeMonitor<V>,
}

pub trait ExecutionResult {
    fn show_info(&self, printer: &Printer);
    fn result(&self) -> Result<String>;
}

impl<T: Output, V> ExecutionResult for Execution<T, V> {
    fn show_info(&self, printer: &Printer) {
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

    fn result(&self) -> Result<String> {
        match &self.result {
            Ok(r) => Ok(format!("{}", r.clone())),
            Err(e) => Err(eyre!(e.to_string()))
        }
    }
}

impl<T: Output, V> Execution<T, V> {
    pub fn new(
        result: color_eyre::Result<T>,
        preparation_start: DateTime<Utc>,
        run_start: DateTime<Utc>,
        run_end: DateTime<Utc>,
        stack_info: RuntimeMonitor<V>,
    ) -> Execution<T, V> {
        Execution {
            result,
            preparation_start,
            run_start,
            run_end,
            stack_info,
        }
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
    pub(crate) is_example: bool,
    pub(crate) user_params: T,
}

impl<T> RunParams<T> {
    pub fn set_is_example(&mut self, is_example: bool) {
        self.is_example = is_example
    }
}

