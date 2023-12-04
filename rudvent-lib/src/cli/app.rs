use std::path::{Path, PathBuf};
use chrono::{DateTime, Local, TimeZone};
use clap_verbosity_flag::Level;
use tracing::{debug, info};
use crate::cli::{Cli, Commands};
use color_eyre::eyre::{eyre, Result};
use crate::printer::Printer;
use crate::solution::Solution;

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
    pub fn run(&self) -> () {
        match &self.cli.sub_cmd {
            Commands::New { day, overwrite } => {
                let instructions = NewInstructions {
                    day: *day,
                    overwrite: *overwrite,
                    app: self,
                };
                instructions.execute().unwrap()
            }
            Commands::Fetch { .. } => {
                unimplemented!()
            }
            Commands::Desc { .. } => {
                unimplemented!()
            }
            Commands::Run { day, example, part_2, other_args } => {
                let mut instructions = RunInstructions {
                    day: *day,
                    example: *example,
                    part_2: *part_2,
                    other_args: other_args.clone(),
                    solutions: self.solutions,
                    app: self,
                };
                instructions.execute().unwrap()
            }
        }
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
        let day_file = self.app.days_directory.join(self.app.day_format.replace("{day}", &*self.day.to_string()));
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
        let mod_declaration = format!("mod {};", self.app.day_format.replace("{day}", &*self.day.to_string()).replace(".rs", ""));
        debug!("Looking for mod declaration: {}", mod_declaration);
        let mut changes = 0;
        let mut within_mods = false;
        let mut found_mod_declaration = false;
        let new_lines = mod_file_contents.lines().map(|line| {
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
                let new_data = format!("Some({}::make_sol)", &self.app.day_format.replace("{day}", &*self.day.to_string()).replace(".rs", ""));
                changes += 1;
                line.replace("None", &*new_data)
            } else {
                line.to_string()
            }
        }).collect::<Vec<String>>();
        if changes == 0 {
            return Err(eyre!("No changes made to mod.rs, perhaps day {} has already been included", self.day));
        } else if changes > 2 {
            return Err(eyre!("Too many matching lines found in mod.rs, this probably means there has been an error"));
        }
        fs_err::write(mod_file, new_lines.join("\n"))?;
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
    pub part_2: bool,
    pub other_args: Vec<String>,
    pub solutions: SolutionBuilders,
    pub app: &'a App,

}

impl RunInstructions<'_> {
    pub fn execute(&mut self) -> Result<()> {
        self.app.printer.print_or_info(&*format!("Running day {}", self.day));
        debug!("Instructions: {:?}", self);
        let builder = self.solutions.get((self.day - 1) as usize);
        let mut solution: Box<dyn Solution> = match builder {
            None => {
                return Err(color_eyre::eyre::eyre!("No solution found for day {}, this probably means there is an error in {}/mod.rs", self.day, self.app.days_directory.to_string_lossy()));
            },
            Some(b) => {
                match b {
                    None => {
                        let release_time = release_time_for_problem(self.app.year, self.day);
                        match release_time {
                            Ok(dt) => {
                                return Err(color_eyre::eyre::eyre!("Day {} has been released, but you haven't created a solution for it. Try running 'new' from the CLI", self.day));
                            },
                            Err(dt) => {
                                let tz_local = DateTime::<Local>::from(dt);
                                return Err(color_eyre::eyre::eyre!("Day {} is not yet released, it will be released at {}", self.day, tz_local));
                            }
                        }

                    }
                    Some(builder) => {
                        builder(self.day, vec![])
                    }
                }
            },
        };
        if self.example {
            self.app.printer.print_or_info(&*format!("Running day {}", self.day));
            let cont = solution.check_example_and_continue(&self.app.printer, !self.part_2);
            if !cont {
                return Ok(());
            }
        }
        self.app.printer.print_or_info(&*format!("Running part {} against full input", if self.part_2 { 2 } else { 1 }));
        let ex = solution.run_part_1();

        Ok(())
    }
}
