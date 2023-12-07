use crate::advent_interactions::{ask_bool_input, DayData};
use crate::cli::{Cli, Commands};
use crate::printer::Printer;
use crate::solution::Solution;
use chrono::{DateTime, Local, TimeZone};
use clap_verbosity_flag::Level;
use color_eyre::eyre::{eyre, Result};
use std::path::{Path, PathBuf};
use tracing::{debug, info};

use crate::types::SolutionBuilders;
use crate::utils::{pathbuf_to_import_string, release_time_for_problem};

static DAY_TEMPLATE: &str = include_str!("../templates/day_template.rs");
static DAY_MOD_TEMPLATE: &str = include_str!("../templates/mod_template.rs");

#[derive(Debug)]
pub struct App {
    pub data_directory: PathBuf,
    pub days_directory: PathBuf,
    pub log_level: Level,
    pub cli: Cli,
    pub printer: Printer,
    pub year: u16,
    pub day_format: String,
    pub solutions: SolutionBuilders,
    pub auth_token: String,
}

impl App {
    pub fn run(&self) -> Result<()> {
        match &self.cli.sub_cmd {
            Commands::New { day, overwrite } => {
                let instructions = NewInstructions {
                    day: *day,
                    overwrite: *overwrite,
                    app: self,
                };
                instructions.execute()?;
                Ok(())
            }
            Commands::Fetch {
                day,
                overwrite,
                dry_run,
            } => {
                self.fetch_data(*day, *dry_run)?;
                Ok(())
            }
            Commands::Desc {
                day,
                dry_run,
                all_html,
                part_1,
            } => {
                println!("Fetching description for day {}", day);
                let day_data = DayData::new(
                    self.year,
                    *day,
                    *dry_run,
                    self.data_directory.clone(),
                    self.auth_token.clone(),
                );
                if *all_html {
                    Ok(println!("{}", day_data.html(*part_1, true, false)?))
                } else {
                    let html = day_data.html(*part_1, false, false)?;
                    let pretty = html2text::from_read(html.as_bytes(), 80);
                    Ok(println!("{}", pretty))
                }
            }
            Commands::Run {
                day,
                example,
                part,
                other_args,
            } => {
                let mut instructions = RunInstructions {
                    day: *day,
                    example: *example,
                    part_1: part.is_part_1(),
                    other_args: other_args.clone(),
                    solutions: self.solutions,
                    app: self,
                };
                instructions.execute()?;
                Ok(())
            }
        }
    }

    fn fetch_data(&self, day: u8, dry_run: bool) -> Result<()> {
        self.printer
            .print_or_info(&format!("Fetching data {}", day));
        let day_data = DayData::new(
            self.year,
            day,
            dry_run,
            self.data_directory.clone(),
            self.auth_token.clone(),
        );
        day_data.fetch_data()?;
        if dry_run {
            self.printer.success(&*format!(
                "Access successful, but because of --dry-run no data saved for day {}",
                day
            ));
        } else {
            self.printer
                .success(&*format!("Fetched and saved data for day {}", day));
        }
        Ok(())
    }
}

#[derive(Debug)]
struct NewInstructions<'a> {
    day: u8,
    overwrite: bool,
    app: &'a App,
}

impl NewInstructions<'_> {
    pub fn execute(&self) -> Result<()> {
        info!("Creating new day {}", self.day);
        debug!("Instructions: {:?}", self);
        if self.app.days_directory.exists() {
            info!("Days directory exists");
        } else {
            info!("Days directory does not exist");
            fs_err::create_dir_all(&self.app.days_directory)?;
        }
        let day_file = self
            .app
            .days_directory
            .join(self.app.day_format.replace("{day}", &*self.day.to_string()));
        if day_file.exists() {
            info!("Day file exists");
            if self.overwrite {
                info!("Overwriting day file");
                self.create_and_replace(&day_file, DAY_TEMPLATE)?;
            } else {
                info!("Not overwriting day file");
            }
        } else {
            info!("Day file does not exist, creating");
            self.create_and_replace(&day_file, DAY_TEMPLATE)?;
        }

        info!("Updating mod.rs to include new day");
        let mod_file = self.app.days_directory.join("mod.rs");
        let mut mod_file_contents = fs_err::read_to_string(&mod_file)?;
        let day_comment = format!("// Day {:0>2}", self.day);
        debug!("Looking for day comment: {}", day_comment);
        let mod_declaration = format!(
            "mod {};",
            self.app
                .day_format
                .replace("{day}", &*self.day.to_string())
                .replace(".rs", "")
        );
        debug!("Looking for mod declaration: {}", mod_declaration);
        let mut changes = 0;
        let mut within_mods = false;
        let mut found_mod_declaration = false;
        let new_lines = mod_file_contents
            .lines()
            .map(|line| {
                if line.contains("// Begin mod declarations") {
                    within_mods = true;
                    line.to_string()
                } else if within_mods & line.contains(&mod_declaration) {
                    found_mod_declaration = true;
                    line.to_string()
                } else if line.contains("// End mod declarations") {
                    within_mods = false;
                    if !found_mod_declaration {
                        changes += 1;
                        format!("{}\n{}", mod_declaration, line)
                    } else {
                        line.to_string()
                    }
                } else if line.contains(&day_comment) & line.contains("None") {
                    // We need a day file relative to the mod file, otherwise too many directories will be included
                    // Giving up on this "better way", just going to hardcode it, I can't imagine the mod file won't be next our day files
                    // let rel_day = pathdiff::diff_paths(&day_file, &mod_file.parent().unwrap()).unwrap();
                    // let new_data = format!("Some({}::make_sol)", pathbuf_to_import_string(&rel_day, None));
                    let new_data = format!(
                        "Some({}::make_sol)",
                        &self
                            .app
                            .day_format
                            .replace("{day}", &*self.day.to_string())
                            .replace(".rs", "")
                    );
                    changes += 1;
                    line.replace("None", &*new_data)
                } else {
                    line.to_string()
                }
            })
            .collect::<Vec<String>>();
        if changes == 0 {
            return Err(eyre!(
                "No changes made to mod.rs, perhaps day {} has already been included",
                self.day
            ));
        } else if changes > 2 {
            return Err(eyre!("Too many matching lines found in mod.rs, this probably means there has been an error"));
        }
        fs_err::write(mod_file, new_lines.join("\n"))?;
        self.app.printer.success(&format!(
            "Created template for day {} in {}",
            self.day,
            day_file.to_string_lossy()
        ));
        Ok(())
    }

    fn create_and_replace(&self, target_path: &Path, template_contents: &str) -> Result<()> {
        let updated = template_contents.replace("use crate::", "use rudvent_lib::");
        fs_err::write(&target_path, updated)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct RunInstructions<'a> {
    pub day: u8,
    pub example: bool,
    pub part_1: bool,
    pub other_args: Vec<String>,
    pub solutions: SolutionBuilders,
    pub app: &'a App,
}

impl RunInstructions<'_> {
    pub fn execute(&mut self) -> Result<()> {
        self.app
            .printer
            .print_or_info(&*format!("Running day {}", self.day));
        debug!("Instructions: {:?}", self);
        let builder = self.solutions.get((self.day - 1) as usize);
        let mut solution: Box<dyn Solution> = match builder {
            None => {
                return Err(color_eyre::eyre::eyre!("No solution found for day {}, this probably means there is an error in {}/mod.rs", self.day, self.app.days_directory.to_string_lossy()));
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
        let ex = solution.run(self.part_1);
        ex.show_info(&self.app.printer);
        let ans = ex.result().unwrap();
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

        Ok(())
    }
}
