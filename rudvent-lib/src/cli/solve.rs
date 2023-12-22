use std::thread::sleep;
use std::time::Duration;
use std::io::{stdout, Write};
use tracing::{debug, info};
use chrono::{DateTime, Local};
use color_eyre::eyre::eyre;
use crossterm::{ExecutableCommand, execute, queue, QueueableCommand};
use crossterm::style::{Print, PrintStyledContent, Color, Stylize};
use crossterm::cursor::{SavePosition, RestorePosition, MoveTo};
use crossterm::terminal::{Clear, ClearType, ScrollDown, ScrollUp};
use crate::advent_interactions::ask_bool_input;
use crate::cli::App;
use crate::solution::{Solution, SolutionBuilder};
use crate::solution::execution::ExecutionResult;
use crate::types::SolutionBuilders;
use crate::utils::release_time_for_problem;

#[derive(Debug)]
pub struct SolveInstructions<'a> {
    pub day: u8,
    pub example: bool,
    pub part_1: bool,
    pub no_post: bool,
    pub other_args: Vec<String>,
    pub solutions: SolutionBuilders,
    pub app: &'a App,
}

impl SolveInstructions<'_> {
    pub fn execute(&mut self) -> color_eyre::Result<()> {
        self.app
            .printer
            .print_or_info(&*format!("Running day {}", self.day));
        debug!("Instructions: {:?}", self);
        if self.solutions.len() == 0 {
            return Err(eyre!("No solutions available, have you commented and uncommented the correct lines in main.rs?"))
        }
        let builder = self.solutions.get((self.day - 1) as usize);
        let mut solution: Box<dyn Solution> = match builder {
            None => {
                return Err(eyre!("No solution found for day {}, this probably means there is an error in {}/mod.rs", self.day, self.app.days_directory.to_string_lossy()));
            }
            Some(b) => match b {
                None => {
                    let release_time = release_time_for_problem(self.app.year, self.day);
                    match release_time {
                        Ok(dt) => {
                            return Err(color_eyre::eyre::eyre!("Day {} has been released, but you haven't created a solution for it. Try running 'new' from the CLI", self.day));
                        }
                        Err(dt) => {
                            let tz_local = DateTime::<Local>::from(dt);
                            return Err(color_eyre::eyre::eyre!(
                                "Day {} is not yet released, it will be released at {}",
                                self.day,
                                tz_local
                            ));
                        }
                    }
                }
                Some(builder) => builder().build(self.app, self.day, self.other_args.clone()),
            },
        };
        solution.day_data().is_data_available(self.part_1)?;

        if self.example {
            self.app
                .printer
                .print_or_info(&*format!("Running day {}", self.day));
            let cont = solution.check_example_and_continue(&self.app.printer, self.part_1);
            if !cont {
                return Ok(());
            }
        }
        self.app.printer.print_or_info(&*format!(
            "Running part {} against full input",
            if self.part_1 { 1 } else { 2 }
        ));
        let mut ex = solution.run(self.part_1);
        let ex_handle = ex.run();
        let mut stdout = stdout();
        stdout.execute(Clear(ClearType::All));
        // stdout.execute(ScrollUp(10));
        while !ex_handle.is_finished() {
            // execute!(stdout(), SavePosition, Print(ex.show_progress()), RestorePosition);
            stdout.queue(PrintStyledContent(ex.show_progress().with(Color::Blue))).unwrap();
            // stdout.queue(MoveTo(0,0));
            stdout.flush();
            sleep(Duration::from_secs(1));

        }
        let ex_result = ex_handle.join().unwrap();
        ex_result.show_info(&self.app.printer);
        let ans = ex_result.result().unwrap();
        self.app.printer.print_or_info(&*format!("Answer: {}", ans));
        let posted = solution.day_data().check_for_posting(self.part_1)?;
        info!("Posted: {}", posted.is_some());
        match posted {
            Some(previous_answer) => {
                if ans == previous_answer {
                    self.app.printer.success(&format!("Your answer matches what you previously posted: {}", ans));
                } else {
                    return Err(eyre!("You got it right before ({}) but your current answer ({}) doens't match", previous_answer, ans))
                }
            },
            None => {
                if !self.no_post {

                self.app
                    .printer
                    .print_or_info("You have not posted your answer yet!");
                if ask_bool_input("Would you like to post your answer now?", false) {
                    let result = solution.day_data().post_ans(&ans.to_string(), self.part_1);
                    match result {
                        Ok(x) => {
                            self.app
                                .printer
                                .success(&format!("Answer was correct! - {}", x));
                            if ask_bool_input("Would you like to see the next problem?", false) {
                                let new_html = if self.part_1 {
                                    solution.day_data().html(false, false, true)?
                                } else {
                                    let next_day = solution.day_data().next_day();
                                    next_day.fetch_data()?;
                                    next_day.html(true, false, false)?
                                };
                                let pretty = html2text::from_read(new_html.as_bytes(), 80);
                                println!("{}", pretty);
                            }
                            return Ok(());
                        }
                        Err(e) => println!("Error posting answer: {}", e),
                    }
                }
                    }
            }
        }

        Ok(())
    }
}
